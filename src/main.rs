mod models;
mod face;
mod stats;

use axum::{
    extract::{ConnectInfo, Request, State},
    http::{Method, StatusCode, header},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use std::env;
use tower::ServiceBuilder;
use tower_http::limit::RequestBodyLimitLayer;
use anyhow::{anyhow, Result};
use governor::{clock::DefaultClock, state::keyed::DefaultKeyedStateStore, Quota, RateLimiter};
use base64::{engine::general_purpose, Engine as _};
use opencv::{core, imgcodecs, objdetect, prelude::*};
use std::net::{IpAddr, SocketAddr};
use std::num::NonZeroU32;
use std::sync::Arc;
use tokio::sync::Mutex;
use models::*;
use face::*;
use stats::*;

type SharedRateLimiter = Arc<RateLimiter<IpAddr, DefaultKeyedStateStore<IpAddr>, DefaultClock>>;

// Middleware: reject any request that doesn't carry the correct internal secret.
// This ensures only our Node.js auth server can reach the Rust backend.
async fn require_internal_secret(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    let provided = request
        .headers()
        .get("x-internal-secret")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if provided != state.internal_secret {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    next.run(request).await
}

#[derive(Clone)]
struct AppState {
    engine: Arc<Mutex<FaceEngine>>,
    limiter: SharedRateLimiter,
    stats: RequestStats,
    internal_secret: String,
}

async fn rate_limit_middleware(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
) -> Response {
    if state.limiter.check_key(&addr.ip()).is_err() {
        return StatusCode::TOO_MANY_REQUESTS.into_response();
    }
    next.run(request).await
}

async fn download_and_decode(url: &str) -> Result<Mat> {
    let bytes: Vec<u8> = if url.starts_with("data:") {
        let comma = url.find(',').ok_or_else(|| anyhow!("Invalid data URI"))?;
        general_purpose::STANDARD.decode(&url[comma + 1..])?
    } else {
        let client = reqwest::Client::builder().user_agent("IrisAPI/1.0").build()?;
        let response = client.get(url).send().await?;
        response.bytes().await?.to_vec()
    };
    let vector_uint8 = core::Vector::<u8>::from_iter(bytes);
    let img = imgcodecs::imdecode(&vector_uint8, imgcodecs::IMREAD_COLOR)?;
    if img.empty() { return Err(anyhow!("Empty image")); }
    Ok(img)
}

async fn handle_compare(
    State(state): State<AppState>,
    Json(payload): Json<CompareRequest>,
) -> Json<CompareResponse> {
    state.stats.record().await;

    let target_img = match download_and_decode(&payload.target_url).await {
        Ok(img) => img,
        Err(_) => return Json(CompareResponse { matches: vec![] }),
    };

    let mut target_embedding: Option<Mat> = None;
    {
        let mut guard = state.engine.lock().await;
        let (det, rec) = unsafe {
            (
                &mut *(guard.detector.as_raw_mut() as *mut objdetect::FaceDetectorYN),
                &mut *(guard.recognizer.as_raw_mut() as *mut objdetect::FaceRecognizerSF)
            )
        };
        if let Ok(Some(emb)) = get_embedding(&target_img, det, rec) {
            target_embedding = Some(emb);
        }
    }

    let Some(t_emb) = target_embedding else {
        return Json(CompareResponse { matches: vec![] });
    };

    let mut results = Vec::new();
    for person in payload.people {
        if let Ok(p_img) = download_and_decode(&person.image_url).await {
            let mut guard = state.engine.lock().await;
            let (det, rec) = unsafe {
                (
                    &mut *(guard.detector.as_raw_mut() as *mut objdetect::FaceDetectorYN),
                    &mut *(guard.recognizer.as_raw_mut() as *mut objdetect::FaceRecognizerSF)
                )
            };

            if let Ok(Some(p_emb)) = get_embedding(&p_img, det, rec) {
                if let Ok(score) = rec.match_(&t_emb, &p_emb, objdetect::FaceRecognizerSF_DisType::FR_COSINE as i32) {
                    if score > 0.363 {
                        results.push(MatchResult {
                            name: person.name,
                            probability: (score.max(0.0) * 100.0).round(),
                        });
                    }
                }
            }
        }
    }

    results.sort_by(|a, b| b.probability.partial_cmp(&a.probability).unwrap());
    Json(CompareResponse { matches: results })
}

async fn handle_stats(State(state): State<AppState>) -> Json<StatsResponse> {
    Json(state.stats.get_stats().await)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    println!("Initializing Iris Face AI...");

    // 5 requests/second per IP, burst up to 10
    let quota = Quota::per_second(NonZeroU32::new(5).unwrap())
        .allow_burst(NonZeroU32::new(10).unwrap());
    let limiter: SharedRateLimiter = Arc::new(RateLimiter::keyed(quota));

    let internal_secret = env::var("IRIS_INTERNAL_SECRET")
        .expect("IRIS_INTERNAL_SECRET env var is required");

    let state = AppState {
        engine: Arc::new(Mutex::new(FaceEngine::new()?)),
        limiter,
        stats: RequestStats::new(),
        internal_secret,
    };

    // CORS is no longer needed — the Rust backend is internal-only.
    // Only the Node.js auth server talks to it, not browsers directly.
    let app = Router::new()
        .route("/v1/compare", post(handle_compare))
        .route("/v1/stats", get(handle_stats))
        .route("/v1/health", get(|| async { "OK" }))
        .layer(middleware::from_fn_with_state(state.clone(), rate_limit_middleware))
        .layer(middleware::from_fn_with_state(state.clone(), require_internal_secret))
        .layer(ServiceBuilder::new()
            .layer(RequestBodyLimitLayer::new(50 * 1024 * 1024)) // 50MB limit
        )
        .with_state(state);

    let port = 8080;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    println!("Iris API running on http://localhost:{}", port);

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;
    Ok(())
}

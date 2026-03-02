mod models;
mod face;
mod stats;

use axum::{
    extract::{ConnectInfo, State},
    http::{Method, StatusCode, header},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Request, Router,
};
use anyhow::{anyhow, Result};
use governor::{clock::DefaultClock, state::keyed::DefaultKeyedStateStore, Quota, RateLimiter};
use opencv::{core, imgcodecs, objdetect, prelude::*};
use std::net::{IpAddr, SocketAddr};
use std::num::NonZeroU32;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use models::*;
use face::*;
use stats::*;

type SharedRateLimiter = Arc<RateLimiter<IpAddr, DefaultKeyedStateStore<IpAddr>, DefaultClock>>;

#[derive(Clone)]
struct AppState {
    engine: Arc<Mutex<FaceEngine>>,
    limiter: SharedRateLimiter,
    stats: RequestStats,
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
    let client = reqwest::Client::builder().user_agent("IrisAPI/1.0").build()?;
    let response = client.get(url).send().await?;
    let bytes = response.bytes().await?;
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
    println!("Initializing Iris Face AI...");

    // 5 requests/second per IP, burst up to 10
    let quota = Quota::per_second(NonZeroU32::new(5).unwrap())
        .allow_burst(NonZeroU32::new(10).unwrap());
    let limiter: SharedRateLimiter = Arc::new(RateLimiter::keyed(quota));

    let state = AppState {
        engine: Arc::new(Mutex::new(FaceEngine::new()?)),
        limiter,
        stats: RequestStats::new(),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::POST, Method::GET])
        .allow_headers([header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/compare", post(handle_compare))
        .route("/stats", get(handle_stats))
        .route("/health", get(|| async { "OK" }))
        .layer(middleware::from_fn_with_state(state.clone(), rate_limit_middleware))
        .layer(cors)
        .with_state(state);

    let port = 8080;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    println!("Iris API running on http://localhost:{}", port);

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;
    Ok(())
}

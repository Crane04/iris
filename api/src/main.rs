mod models;
mod face;

use axum::{routing::post, Json, Router, extract::State};
use anyhow::{Result, anyhow};
use opencv::{core, imgcodecs, objdetect, prelude::*};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use axum::http::{Method, header};
use models::*;
use face::*;

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
    State(engine_arc): State<Arc<Mutex<FaceEngine>>>,
    Json(payload): Json<CompareRequest>,
) -> Json<CompareResponse> {
    let target_img = match download_and_decode(&payload.target_url).await {
        Ok(img) => img,
        Err(_) => return Json(CompareResponse { matches: vec![] }),
    };

    let mut target_embedding: Option<Mat> = None;
    {
        let mut guard = engine_arc.lock().await;
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
            let mut guard = engine_arc.lock().await;
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

#[tokio::main]
async fn main() -> Result<()> {
    println!("Initializing Iris Face AI...");
    let engine = Arc::new(Mutex::new(FaceEngine::new()?));

    // Fix: Proper initialization of CorsLayer
    let cors = CorsLayer::new()
        .allow_origin(Any) 
        .allow_methods([Method::POST]) // Only allow POST
        .allow_headers([header::CONTENT_TYPE]); // Fix: removed Method::POST from here

    // IMPORTANT: You must call .layer(cors) on the Router
    let app = Router::new()
        .route("/compare", post(handle_compare))
        .layer(cors) // This applies the CORS policy to all routes
        .with_state(engine);
    
    let port = 3002;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    println!("Iris API running on http://localhost:{}", port);
    
    axum::serve(listener, app).await?;
    Ok(())
}
use axum::{routing::post, Json, Router, extract::State};
use anyhow::{Result, anyhow};
use opencv::{core, imgcodecs, objdetect, prelude::*};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;



#[derive(Deserialize)]
struct CompareRequest {
    target_url: String,
    people: Vec<Person>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Person {
    name: String,
    image_url: String,
}

#[derive(Serialize)]
struct MatchResult {
    name: String,
    probability: f64, // 0.0 - 100.0 percentage
}

#[derive(Serialize)]
struct CompareResponse {
    matches: Vec<MatchResult>,
}


struct FaceEngine {
    detector: core::Ptr<objdetect::FaceDetectorYN>,
    recognizer: core::Ptr<objdetect::FaceRecognizerSF>,
}

impl FaceEngine {
    fn new() -> Result<Self> {
        let detector = objdetect::FaceDetectorYN::create(
            "face_detection_yunet_2023mar.onnx", "", core::Size::new(320, 320), 0.9, 0.3, 5000, 0, 0
        )?;
        let recognizer = objdetect::FaceRecognizerSF::create(
            "face_recognition_sface_2021dec.onnx", "", 0, 0
        )?;
        Ok(Self { detector, recognizer })
    }
}



fn get_embedding(img: &Mat, det: &mut objdetect::FaceDetectorYN, rec: &mut objdetect::FaceRecognizerSF) -> Result<Option<Mat>> {
    det.set_input_size(img.size()?)?;
    let mut faces = Mat::default();
    det.detect(img, &mut faces)?;
    if faces.rows() > 0 {
        let face_data = faces.row(0)?;
        let mut aligned = Mat::default();
        rec.align_crop(img, &face_data, &mut aligned)?;
        let mut feature = Mat::default();
        rec.feature(&aligned, &mut feature)?;
        return Ok(Some(feature.clone()));
    }
    Ok(None)
}

async fn download_and_decode(url: &str) -> Result<Mat> {
    let client = reqwest::Client::builder().user_agent("Mozilla/5.0").build()?;
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
        let (det_ref, rec_ref) = unsafe {
            (
                &mut *(guard.detector.as_raw_mut() as *mut objdetect::FaceDetectorYN),
                &mut *(guard.recognizer.as_raw_mut() as *mut objdetect::FaceRecognizerSF)
            )
        };
        if let Ok(Some(emb)) = get_embedding(&target_img, det_ref, rec_ref) {
            target_embedding = Some(emb);
        }
    }

    let Some(target_emb) = target_embedding else {
        return Json(CompareResponse { matches: vec![] });
    };

    let mut matches = Vec::new();

    for person in payload.people {
        if let Ok(p_img) = download_and_decode(&person.image_url).await {
            let mut guard = engine_arc.lock().await;
            let (det_ref, rec_ref) = unsafe {
                (
                    &mut *(guard.detector.as_raw_mut() as *mut objdetect::FaceDetectorYN),
                    &mut *(guard.recognizer.as_raw_mut() as *mut objdetect::FaceRecognizerSF)
                )
            };

            if let Ok(Some(p_emb)) = get_embedding(&p_img, det_ref, rec_ref) {
                if let Ok(score) = rec_ref.match_(&target_emb, &p_emb, objdetect::FaceRecognizerSF_DisType::FR_COSINE as i32) {
                  
                    // If score is 1.0 then 100%. If score is 0.0 then 0%
                    let probability = (score.max(0.0) * 100.0).round();

                    if score > 0.363 {
                        matches.push(MatchResult {
                            name: person.name,
                            probability,
                        });
                    }
                }
            }
        }
    }

    matches.sort_by(|a, b| b.probability.partial_cmp(&a.probability).unwrap());

    Json(CompareResponse { matches })
}

#[tokio::main]
async fn main() -> Result<()> {
    let engine = Arc::new(Mutex::new(FaceEngine::new()?));
    let app = Router::new().route("/compare", post(handle_compare)).with_state(engine);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await?;
    Ok(())
}
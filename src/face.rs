use anyhow::{Result};
use opencv::{core, objdetect, prelude::*};

pub struct FaceEngine {
    pub detector: core::Ptr<objdetect::FaceDetectorYN>,
    pub recognizer: core::Ptr<objdetect::FaceRecognizerSF>,
}

impl FaceEngine {
    pub fn new() -> Result<Self> {
        let detector = objdetect::FaceDetectorYN::create(
            "face_detection_yunet_2023mar.onnx", "", core::Size::new(320, 320), 0.9, 0.3, 5000, 0, 0
        )?;
        let recognizer = objdetect::FaceRecognizerSF::create(
            "face_recognition_sface_2021dec.onnx", "", 0, 0
        )?;
        Ok(Self { detector, recognizer })
    }
}

pub fn get_embedding(
    img: &Mat, 
    det: &mut objdetect::FaceDetectorYN, 
    rec: &mut objdetect::FaceRecognizerSF
) -> Result<Option<Mat>> {
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
use anyhow::{Result, anyhow};
use opencv::{
    core,
    imgcodecs,
    objdetect,
    prelude::*,
};
use std::io::Read;


struct Person {
    name: String,
    image_url: String,
}

fn get_face_embedding(
    img: &Mat, 
    detector: &mut objdetect::FaceDetectorYN, 
    recognizer: &mut objdetect::FaceRecognizerSF
) -> Result<Option<Mat>> {
  
    detector.set_input_size(img.size()?)?;

    let mut faces = Mat::default();
    detector.detect(img, &mut faces)?;

    if faces.rows() > 0 {
  
        let face_data = faces.row(0)?; 
   
        let mut aligned_face = Mat::default();
        recognizer.align_crop(img, &face_data, &mut aligned_face)?;

        // Extract Features
        let mut feature = Mat::default();
        recognizer.feature(&aligned_face, &mut feature)?;
        
        return Ok(Some(feature.clone()));
    }
    
    Ok(None)
}

fn compare_faces(target_url: &str, people: Vec<Person>) -> Result<Vec<String>> {
    let mut detector = objdetect::FaceDetectorYN::create(
        "face_detection_yunet_2023mar.onnx", "", core::Size::new(320, 320), 0.9, 0.3, 5000, 0, 0
    )?;
    let mut recognizer = objdetect::FaceRecognizerSF::create(
        "face_recognition_sface_2021dec.onnx", "", 0, 0
    )?;

    println!("Downloading target image...");
    let target_img = download_and_decode(target_url)?;
    
    let (det_ref, rec_ref) = unsafe {
        (
            &mut *(detector.as_raw_mut() as *mut objdetect::FaceDetectorYN),
            &mut *(recognizer.as_raw_mut() as *mut objdetect::FaceRecognizerSF)
        )
    };

    let target_embedding = match get_face_embedding(&target_img, det_ref, rec_ref)? {
        Some(emb) => emb,
        None => return Err(anyhow!("No face found in target image")),
    };

    let mut matching_names = Vec::new();

    for person in people {
        println!("Checking {}...", person.name);
        let p_img = match download_and_decode(&person.image_url) {
            Ok(img) => img,
            Err(e) => {
                eprintln!("  [!] Failed to download {}: {}", person.name, e);
                continue;
            }
        };

        match get_face_embedding(&p_img, det_ref, rec_ref) {
            Ok(Some(p_embedding)) => {
                let score = rec_ref.match_(
                    &target_embedding, 
                    &p_embedding, 
                    objdetect::FaceRecognizerSF_DisType::FR_COSINE as i32
                )?;

                println!("  [+] Score for {}: {:.4}", person.name, score);

                if score > 0.363 {
                    matching_names.push(person.name);
                }
            }
            Ok(None) => eprintln!("  [!] No face detected for {}", person.name),
            Err(e) => eprintln!("  [!] Error processing {}: {}", person.name, e),
        }
    }

    Ok(matching_names)
}

fn download_and_decode(url: &str) -> Result<Mat> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
        .build()?;
        
    let mut response = client.get(url).send()?;
    
    if !response.status().is_success() {
        return Err(anyhow!("HTTP Status {}", response.status()));
    }

    let mut buffer = Vec::new();
    response.read_to_end(&mut buffer)?;
    let vector_uint8 = core::Vector::<u8>::from_iter(buffer);
    let img = imgcodecs::imdecode(&vector_uint8, imgcodecs::IMREAD_COLOR)?;
    if img.empty() { return Err(anyhow!("Empty image decoded from {}", url)); }
    Ok(img)
}

fn main() -> Result<()> {
    let target_image_url = "https://static.wikia.nocookie.net/amazingspiderman/images/3/33/Tobey_Maguire_Infobox.png/revision/latest/scale-to-width-down/535?cb=20240322015635";
    
    let people = vec![
            Person { 
                name: "John".to_string(), 
                image_url: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQqiVCCW7eH5Q_8q4VULShU7O8QnOgp7Us2RBNhAlnesh2_iho_D1Toosuxj_x66J1w8ks&usqp=CAU".to_string() 
            },
            Person { 
                name: "Jane".to_string(), 
                image_url: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcT1RMxB8hqtNe37Tua2VWEg_Z79JPTjQmwvsQ&s".to_string() 
            },
            Person { 
                name: "Bob".to_string(), 
                image_url: "https://images.pexels.com/photos/1139743/pexels-photo-1139743.jpeg?auto=compress&cs=tinysrgb&dpr=1&w=500".to_string() 
            },
    ];

    match compare_faces(target_image_url, people) {
        Ok(matches) => println!("Matching names: {:?}", matches),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
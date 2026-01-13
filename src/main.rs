use anyhow::{Result, anyhow};
use opencv::{
    core::{self, Size},
    imgcodecs,
    imgproc,
    objdetect,
    prelude::*,
};
use std::io::Read;

struct Person {
    name: String,
    image_url: String,
}

fn download_and_decode(url: &str) -> Result<Mat> {
    let mut response = reqwest::blocking::get(url)?;
    let mut buffer = Vec::new();
    response.read_to_end(&mut buffer)?;
    
    let vector_uint8 = core::Vector::<u8>::from_iter(buffer);
    let img = imgcodecs::imdecode(&vector_uint8, imgcodecs::IMREAD_COLOR)?;
    
    if img.empty() {
        return Err(anyhow!("Failed to decode image from {}", url));
    }
    Ok(img)
}

fn detect_and_extract_face(img: &Mat, cascade: &mut objdetect::CascadeClassifier) -> Result<Option<Mat>> {
    let mut gray = Mat::default();
    imgproc::cvt_color(img, &mut gray, imgproc::COLOR_BGR2GRAY, 0, core::AlgorithmHint::ALGO_HINT_APPROX)?;

    let mut faces = core::Vector::<core::Rect>::new();
    cascade.detect_multi_scale(
        &gray,
        &mut faces,
        1.1,
        5,
        0,
        Size::new(30, 30),
        Size::new(0, 0),
    )?;

    if !faces.is_empty() {
        let face_rect = faces.get(0)?;
        // Create a ROI (Region of Interest) view
        let face_roi_view = Mat::roi(img, face_rect)?;
        // Clone the view to get an owned Mat
        let face_roi = face_roi_view.try_clone()?;
        return Ok(Some(face_roi));
    }
    Ok(None)
}

fn compare_faces(target_url: &str, people: Vec<Person>) -> Result<Vec<String>> {
    // Initialize Cascade
    let mut face_cascade = objdetect::CascadeClassifier::new(
        &opencv::core::find_file("haarcascades/haarcascade_frontalface_default.xml", true, false)?
    )?;

    // 1. Process Target Image once
    let target_img = download_and_decode(target_url)?;
    let target_face_opt = detect_and_extract_face(&target_img, &mut face_cascade)?;

    let mut matching_names = Vec::new();

    if let Some(t_face) = target_face_opt {
        // Prepare target face (Resize and Gray)
        let mut t_resized = Mat::default();
        imgproc::resize(&t_face, &mut t_resized, Size::new(100, 100), 0.0, 0.0, imgproc::INTER_LINEAR)?;
        
        let mut t_gray = Mat::default();
        imgproc::cvt_color(&t_resized, &mut t_gray, imgproc::COLOR_BGR2GRAY, 0, core::AlgorithmHint::ALGO_HINT_APPROX)?;

        // 2. Iterate over people
        for person in people {
            let person_img = match download_and_decode(&person.image_url) {
                Ok(img) => img,
                Err(_) => continue,
            };

            if let Ok(Some(p_face)) = detect_and_extract_face(&person_img, &mut face_cascade) {
                let mut p_resized = Mat::default();
                imgproc::resize(&p_face, &mut p_resized, Size::new(100, 100), 0.0, 0.0, imgproc::INTER_LINEAR)?;

                let mut p_gray = Mat::default();
                imgproc::cvt_color(&p_resized, &mut p_gray, imgproc::COLOR_BGR2GRAY, 0, core::AlgorithmHint::ALGO_HINT_APPROX)?;

                // Calculate Absolute Difference
                let mut diff = Mat::default();
                core::absdiff(&t_gray, &p_gray, &mut diff)?;

                let mean_val = core::mean(&diff, &core::no_array())?;
                
                // Thresholding logic
                if mean_val[0] < 50.0 {
                    matching_names.push(person.name);
                }
            }
        }
    }

    Ok(matching_names)
}

fn main() -> Result<()> {
    let target_image_url = "https://static.wikia.nocookie.net/amazingspiderman/images/3/33/Tobey_Maguire_Infobox.png/revision/latest/scale-to-width-down/535?cb=20240322015635";
    
    let people = vec![
        Person { 
            name: "John".to_string(), 
            image_url: "https://exncrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQqiVCCW7eH5Q_8q4VULShU7O8QnOgp7Us2RBNhAlnesh2_iho_D1Toosuxj_x66J1w8ks&usqp=CAU".to_string() 
        },
        Person { 
            name: "Jane".to_string(), 
            image_url: "https://m.media-amazon.com/images/M/MV5BMTYwMTI5NTM2OF5BMl5BanBnXkFtZTcwODk3MDQ2Mg@@._V1_FMjpg_UX1000_.jpg".to_string() 
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
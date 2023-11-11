use std::env::current_dir;
use opencv::{
    Result,
    prelude::*,
    objdetect,
    highgui,
    imgproc,
    core,
    types,
    videoio,
};

const ASCII_Q: i32 = 113;

fn main() -> Result<()> {
    let path_absolute: String = match current_dir() {
        Ok(path) => path.to_str().unwrap().to_owned(),
        _ => "".to_string(),
    };

    println!("{:?}", &path_absolute);

    /* Option 1: fetch media from camera */
    let mut camera = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    /* Option 2: fetch media from video

    let mut camera = videoio::VideoCapture::from_file(&format!("{path_absolute}/resources/demo.mp4"), videoio::CAP_FFMPEG)?;
    */

    let mut image_camera = Mat::default();
    let xml = format!("{path_absolute}/haarcascades/frontalface_default.xml");
    let mut face_detector = objdetect::CascadeClassifier::new(&xml)?;
    loop {
        camera.read(&mut image_camera)?;

        /* transfer RGB to Gray Scale can improve detection performances */
        let mut image_gray = Mat::default();
        imgproc::cvt_color(&image_camera, &mut image_gray, imgproc::COLOR_BGR2GRAY, 0)?;
        let mut result_faces = types::VectorOfRect::new();
        face_detector.detect_multi_scale(
            &image_gray,
            &mut result_faces,
            1.1,
            5,
            objdetect::CASCADE_SCALE_IMAGE,
            core::Size::new(30, 30),
            core::Size::new(0, 0)
        );
        if result_faces.len() > 0 {
            for face in result_faces {
                imgproc::rectangle(
                    &mut image_camera,
                    face,
                    core::Scalar::new(0.0, 255.0, 0.0, 0.0),
                    2,
                    imgproc::LINE_8,
                    0
                )?;
            }
        }
        highgui::imshow("Camera", &image_camera)?;

        // press `q` can escape
        let press: i32 = highgui::wait_key(1)?;
        if press == ASCII_Q {
            break;
        }
    }

    // unreachable!()
    return Ok(())
}

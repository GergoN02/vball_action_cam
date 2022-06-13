use nokhwa::{Camera, CameraFormat};

fn main() {
    println!("Hello, world!");

    let mut camera = Camera::new(
        0,
        Some(CameraFormat::new_from(
            640,
            480,
            nokhwa::FrameFormat::MJPEG,
            30,
        )),
    )
    .unwrap();

    camera.open_stream().unwrap();

    loop {
        let frame = camera.frame().unwrap();
        println!("{}, {}", frame.width(), frame.height());
    }
}

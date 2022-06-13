use nokhwa::{Camera, CameraFormat, CameraInfo, CaptureAPIBackend, NokhwaError};

fn main() {
    println!("Hello, world!");

    // let dev = nokhwa::query_devices(CaptureAPIBackend::Auto);

    // println!("{:?}", dev);

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

extern crate image_ssd;
extern crate image;
extern crate rscam;
use std::fs;
use std::io::Write;

pub fn main() {
    example().unwrap();
}

fn example() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut camera = rscam::new("/dev/video0")?;
    camera.start(&rscam::Config {
        interval: (1, 30),      // 30 fps.
        resolution: (1280, 720),
        format: b"MJPG",
        ..Default::default()
    })?;

    let ssd_graph = image_ssd::get_or_load_ssd_mobilenet_v2_graph()?;
    let ssd_net = image_ssd::SSDMobileNetV2::load(&ssd_graph)?;

    loop {
        let src_frame_buffer = camera.capture()?.to_vec();
        let src_frame_img = image::ImageBuffer::from_raw(1280, 720, src_frame_buffer)
            .map(image::DynamicImage::ImageRgba8)
            .ok_or_else(|| "Cannot contruct a buffer")?;

        let ssd_boxes = ssd_net.shot(&src_img)?;
        let dst_frame_img = image_ssd::ImageBoxes::new()
            .border(image_ssd::Color::rgb8(0xBF, 0x90, 0x00), 3.0)
            .font("Arial", 24.0)
            .score(0.5)
            .draw(&src_frame_img, ssd_boxes)?;
        // TODO: write dst_frame_img to the HTTP stream
    }

    Ok(())
}

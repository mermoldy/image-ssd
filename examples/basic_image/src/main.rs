extern crate image_ssd;
extern crate image;

pub fn main() {
    example().unwrap();
}

fn example() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let src_img_path = &std::path::Path::new("images/car.jpg");
    let src_img = image::open(&src_img_path)?;

    let ssd_graph = image_ssd::get_or_load_ssd_mobilenet_v2_graph()?;
    let ssd_net = image_ssd::SSDMobileNetV2::load(&ssd_graph)?;
    let ssd_boxes = ssd_net.shot(&src_img)?;

    let dest_img_path = &std::path::Path::new("images/car-out.png");
    let dst_img = image_ssd::ImageBoxes::new()
        .border(image_ssd::Color::rgb8(0xBF, 0x90, 0x00), 3.0)
        .font("Arial", 24.0)
        .score(0.5)
        .draw(&src_img, ssd_boxes)?;
    dst_img.save_with_format(&dest_img_path, image::ImageFormat::Png)?;

    Ok(())
}

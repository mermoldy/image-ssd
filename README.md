# Image SSD

[![Build](https://github.com/mermoldy/image-ssd/workflows/Build/badge.svg)](https://github.com/mermoldy/image-ssd/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust library for object detection via SSD MobileNet.

## Usage

```rust
let src_img_path = &std::path::Path::new("images/example.jpg");
let src_img = image::open(&src_img_path)?;

let ssd_graph = image_ssd::get_or_load_ssd_mobilenet_v2_graph()?;
let ssd_net = image_ssd::SSDMobileNetV2::load(&ssd_graph)?;
let ssd_boxes = ssd_net.shot(&src_img)?;

let dest_img_path = &std::path::Path::new("images/example-out.png");
let dst_img = image_ssd::ImageBoxes::new()
    .border(image_ssd::Color::rgb8(0xBF, 0x90, 0x00), 3.0)
    .font("Arial", 24.0)
    .score(0.5)
    .draw(&src_img, ssd_boxes)?;
dst_img.save_with_format(&dest_img_path, image::ImageFormat::Png)?;
```

### Input

<div style="text-align:center"><img src="examples/basic_image/images/car.jpg" alt="Input" width="400"/></div>

### Output

<div style="text-align:center"><img src="examples/basic_image/images/car-out.png" alt="Output" width="400"/></div>

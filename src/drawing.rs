extern crate image;
use crate::error;
use crate::ssd_mobilenet;

use piet::kurbo;

use image::GenericImageView;
use piet::{RenderContext, Text, TextLayout, TextLayoutBuilder};

pub struct ImageBoxes {
    label_color: piet::Color,
    border_color: piet::Color,
    border_width: f64,
    font_name: String,
    font_size: f64,
    score: f32,
}

impl ImageBoxes {
    pub fn new() -> Self {
        ImageBoxes {
            border_color: piet::Color::BLACK,
            border_width: 2.0,
            label_color: piet::Color::WHITE,
            font_name: "Arial".to_string(),
            font_size: 16.0,
            score: 0.5,
        }
    }

    pub fn font(mut self, font_name: &str, font_size: f64) -> Self {
        self.font_name = font_name.to_string();
        self.font_size = font_size;
        self
    }

    pub fn label(mut self, color: piet::Color) -> Self {
        self.label_color = color;
        self
    }

    pub fn border(mut self, color: piet::Color, width: f64) -> Self {
        self.border_color = color;
        self.border_width = width;
        self
    }

    pub fn score(mut self, score: f32) -> Self {
        self.score = score;
        self
    }

    pub fn draw_image<T: piet::RenderContext>(
        &self,
        ctx: &mut T,
        image: &image::DynamicImage,
    ) -> Result<(), piet::Error> {
        let width = image.width();
        let height = image.height();
        let img_raw = image.to_rgba().to_vec();

        let size = kurbo::Size::new(width as f64, height as f64);
        let img = ctx.make_image(
            width as usize,
            height as usize,
            &img_raw,
            piet::ImageFormat::RgbaPremul,
        )?;
        ctx.draw_image(
            &img,
            size.to_rect(),
            piet::InterpolationMode::NearestNeighbor,
        );

        Ok(())
    }

    pub fn draw_boxes<T: piet::RenderContext>(
        &self,
        ctx: &mut T,
        boxes: Vec<ssd_mobilenet::DetectionBox>,
        width: f64,
        height: f64,
    ) -> Result<(), piet::Error> {
        let brush = ctx.solid_brush(self.border_color.clone());

        for b in boxes.iter() {
            if b.score > self.score {
                ctx.stroke(
                    kurbo::Rect::new(
                        b.x1 as f64 * width,
                        b.y1 as f64 * height,
                        b.x2 as f64 * width,
                        b.y2 as f64 * height,
                    ),
                    &brush,
                    self.border_width,
                );

                let font_family = ctx
                    .text()
                    .font_family(self.font_name.as_str())
                    .ok_or(piet::Error::MissingFont)?;

                let label_text = format!(" {} ({:.1}%)", b.label, b.score * 100.0);
                let layout = ctx
                    .text()
                    .new_text_layout(label_text)
                    .font(font_family, self.font_size)
                    .text_color(self.label_color.clone())
                    .build()?;

                let text_pos = kurbo::Vec2::new(
                    (b.x1 * (width as f32)) as f64,
                    (b.y1 * (height as f32)) as f64,
                );

                let layout_rect = layout.size().to_rect() + text_pos;
                layout_rect.width();

                ctx.fill(layout_rect, &self.border_color);
                ctx.draw_text(&layout, text_pos.to_point());
            }
        }

        Ok(())
    }

    pub fn draw(
        self,
        input_image: &image::DynamicImage,
        boxes: Vec<ssd_mobilenet::DetectionBox>,
    ) -> Result<image::DynamicImage, error::Error> {
        let img = input_image.resize_exact(
            (input_image.width() as f32 / 16.0).ceil() as u32 * 16,
            input_image.height(),
            image::imageops::FilterType::Nearest,
        ); // XXX: non-16-divisible width affect the color accuracy in some way

        let width = img.width();
        let height = img.height();

        let mut device = piet_common::Device::new()?;
        let mut bitmap = device.bitmap_target(width as usize, height as usize, 1.0)?;
        let mut ctx = bitmap.render_context();
        ctx.clear(piet::Color::WHITE);

        self.draw_image(&mut ctx, &img)?;
        self.draw_boxes(&mut ctx, boxes, width as f64, height as f64)?;

        ctx.finish()?;
        std::mem::drop(ctx);

        let mut buffer = vec![0; (width * height * 4) as usize];
        bitmap.copy_raw_pixels(piet::ImageFormat::RgbaPremul, &mut buffer)?;
        let res_img = image::ImageBuffer::from_raw(width, height, buffer)
            .map(image::DynamicImage::ImageRgba8)
            .ok_or_else(|| "Cannot contruct a buffer from a generic container")?;

        Ok(res_img)
    }
}

impl Default for ImageBoxes {
    fn default() -> Self {
        ImageBoxes::new()
    }
}

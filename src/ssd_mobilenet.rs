use crate::error;
use crate::ms_coco;
use crate::utils;

use std::path;
use tensorflow as tf;

#[derive(PartialEq, Clone, Debug)]
pub struct DetectionBox {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub label: String,
    pub score: f32,
}

type SSDTensors = (
    tensorflow::Tensor<f32>,
    tensorflow::Tensor<f32>,
    tensorflow::Tensor<f32>,
    tensorflow::Tensor<f32>,
);

pub struct SSDMobileNetV2 {
    graph: tf::Graph,
    session: tf::Session,
    label_map: ms_coco::LabelMap,
}

impl SSDMobileNetV2 {
    pub fn load(ssd_graph_path: &path::Path) -> Result<Self, error::Error> {
        let label_map = ms_coco::LabelMap::load()?;
        let mut graph = tf::Graph::new();
        let session = tf::Session::new(&tf::SessionOptions::new(), &graph)?;

        let graph_def = utils::get_file_as_byte_vec(ssd_graph_path);
        graph.import_graph_def(&graph_def[..], &tf::ImportGraphDefOptions::new())?;

        Ok(SSDMobileNetV2 {
            graph,
            session,
            label_map,
        })
    }

    fn transform_image(
        &self,
        img: &image::DynamicImage,
    ) -> Result<(tf::Operation, tf::Tensor<u8>), error::Error> {
        let (height, width) = (300, 300);
        let img_min = img.resize_exact(width, height, image::imageops::FilterType::Nearest);

        let image_array = ndarray::Array::from_shape_vec(
            (height as usize, width as usize, 3),
            img_min.to_rgb().to_vec(),
        )?;
        let image_array_expanded = image_array.insert_axis(ndarray::Axis(0));
        let image_array_slice = image_array_expanded
            .as_slice()
            .ok_or_else(|| "Failed to convert the data array to slice")?;

        let image_tensor_op = self.graph.operation_by_name_required("image_tensor")?;
        let input_image_tensor = tf::Tensor::new(&[1, u64::from(height), u64::from(width), 3])
            .with_values(image_array_slice)?;

        Ok((image_tensor_op, input_image_tensor))
    }

    fn run(&self, img: &image::DynamicImage) -> Result<SSDTensors, error::Error> {
        let (image_tensor_op, input_image_tensor) = self.transform_image(img)?;

        let mut args = tf::SessionRunArgs::new();
        args.add_feed(&image_tensor_op, 0, &input_image_tensor);

        let num_detections = self.graph.operation_by_name_required("num_detections")?;
        let classes = self.graph.operation_by_name_required("detection_classes")?;
        let boxes = self.graph.operation_by_name_required("detection_boxes")?;
        let scores = self.graph.operation_by_name_required("detection_scores")?;

        let num_detections_token = args.request_fetch(&num_detections, 0);
        let classes_token = args.request_fetch(&classes, 0);
        let boxes_token = args.request_fetch(&boxes, 0);
        let scores_token = args.request_fetch(&scores, 0);

        self.session.run(&mut args)?;

        Ok((
            args.fetch::<f32>(num_detections_token)?,
            args.fetch::<f32>(boxes_token)?,
            args.fetch::<f32>(classes_token)?,
            args.fetch::<f32>(scores_token)?,
        ))
    }

    pub fn shot(&self, img: &image::DynamicImage) -> Result<Vec<DetectionBox>, error::Error> {
        let (_, boxes_tensor, classes_tensor, scores_tensor) = self.run(img)?;

        let label_names: Result<Vec<String>, ms_coco::LabelNotFound> = classes_tensor
            .iter()
            .map(|class| self.label_map.get_label_name(&(*class as i32)))
            .collect();
        let label_names = label_names?;

        let boxes: Vec<DetectionBox> = zip!(
            boxes_tensor.chunks_exact(4),
            label_names.iter(),
            scores_tensor.iter()
        )
        .map(|(bbox, (label, score))| DetectionBox {
            y1: bbox[0],
            x1: bbox[1],
            y2: bbox[2],
            x2: bbox[3],
            label: label.to_string(),
            score: *score,
        })
        .collect();

        Ok(boxes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate image;
    use crate::cache;

    #[test]
    fn shot() {
        let src_img_path = &std::path::Path::new("examples/basic_image/images/car.jpg");
        let src_img = image::open(&src_img_path).unwrap();
        let ssd_graph = cache::get_or_load_ssd_mobilenet_v2_graph().unwrap();

        let ssd_net = SSDMobileNetV2::load(&ssd_graph).unwrap();
        let mut ssd_boxes = ssd_net.shot(&src_img).unwrap();

        assert_eq!(ssd_boxes.len(), 100);

        ssd_boxes = ssd_boxes
            .iter()
            .filter(|d_box| d_box.score > 0.3)
            .cloned()
            .collect();
        assert_eq!(ssd_boxes.len(), 2);
        assert_eq!(ssd_boxes[0].label, "car");
        assert_eq!(ssd_boxes[1].label, "car");
    }
}

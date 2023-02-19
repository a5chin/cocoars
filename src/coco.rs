mod info;
mod license;
mod image;
mod annotation;
mod category;

use std::iter::zip;
use serde::{Serialize, Deserialize};
use polars::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Coco {
    pub info: info::Info,
    pub licenses: Vec<license::License>,
    pub images: Vec<image::Image>,
    pub annotations: Vec<annotation::Annotation>,
    pub categories: Vec<category::Category>,
}

impl Coco {
    pub fn new(path: &str) -> Result<Coco, serde_json::Error> {
        let json_str = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => panic!("Error: {e}")
        };

        serde_json::from_str(&json_str)
    }

    pub fn create_df(&self) -> Result<DataFrame, PolarsError> {
        let mut ids: Vec<i64> = Vec::new();
        let mut file_names: Vec<&str> = Vec::new();
        let mut heights: Vec<i32> = Vec::new();
        let mut widths: Vec<i32> = Vec::new();

        let mut areas: Vec<f64> = Vec::new();
        let mut iscrowds: Vec<i32> = Vec::new();
        let mut image_id: Vec<i64> = Vec::new();
        let mut category_ids: Vec<i32> = Vec::new();

        for (image, annotation) in zip(&self.images, &self.annotations) {
            ids.push(image.id);
            file_names.push(&image.file_name);
            heights.push(image.height);
            widths.push(image.width);

            areas.push(annotation.area);
            iscrowds.push(annotation.iscrowd);
            image_id.push(annotation.image_id);
            category_ids.push(annotation.category_id);
        }

        let id = Series::new("id", ids);
        let file_name = Series::new("file_name", file_names);
        let height = Series::new("height", heights);
        let width = Series::new("width", widths);
        let area = Series::new("area", areas);
        let iscrowd = Series::new("iscrowd", iscrowds);
        let image_id = Series::new("image_id", image_id);
        let category_id = Series::new("category_id", category_ids);

        DataFrame::new(vec![id, file_name, height, width, area, iscrowd, image_id, category_id])
    }
}

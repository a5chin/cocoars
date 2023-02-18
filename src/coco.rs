mod info;
mod license;
mod image;
mod annotation;
mod category;

use serde::{Serialize, Deserialize};

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
}

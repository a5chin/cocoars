use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub license: i8,
    pub file_name: String,
    pub coco_url: String,
    pub height: i32,
    pub width: i32,
    pub date_captured: String,
    pub flickr_url: String,
    pub id: i64,
}

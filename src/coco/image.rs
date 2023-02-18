use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    license: i8,
    file_name: String,
    coco_url: String,
    height: i16,
    width: i16,
    date_captured: String,
    flickr_url: String,
    id: i64,
}

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Annotation {
    pub segmentation: SegmentationUnion,
    pub area: f64,
    pub iscrowd: i32,
    pub image_id: i64,
    pub bbox: Vec<f64>,
    pub category_id: i32,
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObscureSegment {
    counts: Vec<i64>,
    size: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SegmentationUnion {
    ApparentSegment(Vec<Vec<f64>>),
    ObscureSegment(ObscureSegment),
}

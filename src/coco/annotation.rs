use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Annotation {
    segmentation: SegmentationUnion,
    area: f64,
    iscrowd: i8,
    image_id: i64,
    bbox: Vec<f64>,
    category_id: i8,
    id: i64,
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

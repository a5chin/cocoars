use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    supercategory: String,
    id: i8,
    name: String,
}

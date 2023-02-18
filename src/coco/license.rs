use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct License {
    url: String,
    id: i8,
    name: String,
}

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    description: String,
    url: String,
    version: String,
    year: i16,
    contributor: String,
    date_created: String,
}

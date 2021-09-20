use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Teacher {
    id: u32,
    caption: String,
    long_caption: String,
}

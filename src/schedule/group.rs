use serde::{Serialize, Deserialize};
use super::Teacher;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Group {
    class: String,
    short_caption: String,
    caption: String,
    full_caption: String,
    teachers: Vec<Teacher>
}

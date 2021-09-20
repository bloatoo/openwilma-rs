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

impl Group {
    pub fn class(&self) -> &String {
        &self.class
    }

    pub fn short_caption(&self) -> &String {
        &self.short_caption
    }

    pub fn caption(&self) -> &String {
        &self.caption
    }

    pub fn full_caption(&self) -> &String {
        &self.full_caption
    }

    pub fn teachers(&self) -> &Vec<Teacher> {
        &self.teachers
    }
}

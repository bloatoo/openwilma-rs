use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Teacher {
    id: u32,
    caption: String,
    long_caption: String,
}

impl Teacher {
    pub fn id(&self) -> &u32 {
        &self.id
    }

    pub fn caption(&self) -> &String {
        &self.caption
    }

    pub fn long_caption(&self) -> &String {
        &self.long_caption
    }
}

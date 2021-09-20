use serde::{Serialize, Deserialize};
use serde_json::{Value, from_value};
use super::Group;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Reservation {
    start: String,
    end: String,
    day: u8,
    groups: Vec<Group>
}

impl Reservation {
    pub fn from_json(json: &Value) -> Result<Self, Box<dyn std::error::Error>> {
        let result = from_value::<Self>(json.clone())?;
        Ok(result)
    }

    /*pub fn caption(&self) -> &String {
        &self.groups[0].caption()
    }*/

    pub fn groups(&self) -> &Vec<Group> {
        &self.groups
    }

    pub fn start(&self) -> &String {
        &self.start
    }

    pub fn end(&self) -> &String {
        &self.end
    }
}

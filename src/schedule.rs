use serde_json::{Value, from_str, from_value};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Reservation {
    start: String,
    end: String,
    groups: Vec<Group>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Group {
    class: String,
    short_caption: String,
    caption: String,
    full_caption: String,
    teachers: Vec<Teacher>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Teacher {
    id: u32,
    caption: String,
    long_caption: String,
}

impl Reservation {
    pub fn from_json(json: &Value) -> Result<Self, Box<dyn std::error::Error>> {
        let result = from_value::<Self>(json.clone())?;
        Ok(result)
    }

    pub fn caption(&self) -> &String {
        &self.groups[0].caption
    }

    pub fn start(&self) -> &String {
        &self.start
    }

    pub fn end(&self) -> &String {
        &self.end
    }
}

#[derive(Debug, Clone)]
pub struct Schedule {
    reservations: Vec<Reservation>
}

impl Schedule {
    pub fn from_json(json: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json: Value = from_str(&json)?;

        let reservations_json = json.get("Schedule")
            .unwrap()
            .as_array()
            .unwrap();

        let mut reservations = vec![];

        for reserv in reservations_json {
            let reservation = Reservation::from_json(reserv)?;
            reservations.push(reservation);
        }

        Ok(Self {
            reservations
        })
    }

    pub fn reservations(&self) -> &Vec<Reservation> {
        &self.reservations
    }
}

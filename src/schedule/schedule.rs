use serde_json::{Value, from_str};
use super::Reservation;

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

use serde_json::{Value, from_str};

#[derive(Debug, Clone)]
pub struct Reservation {
    caption: String,
    start: String,
    end: String,
}

impl Reservation {
    pub fn new(caption: String, start: String, end: String) -> Self {
        Self {
            caption,
            start,
            end,
        }
    }

    pub fn caption(&self) -> &String {
        &self.caption
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
            let group = &reserv.get("Groups")
                .unwrap()
                .as_array()
                .unwrap()
                [0];

            let caption = group.get("Caption")
                .unwrap()
                .as_str()
                .unwrap();

            let start = reserv.get("Start")
                .unwrap()
                .as_str()
                .unwrap();

            let end = reserv.get("End")
                .unwrap()
                .as_str()
                .unwrap();
            
            let reservation = Reservation::new(
                caption.into(),
                start.into(),
                end.into()
            );

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

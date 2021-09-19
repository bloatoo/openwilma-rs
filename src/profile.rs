pub struct Profile {
    name: String,
    school: String,
}

impl Profile {
    pub fn new(name: String, school: String) -> Self {
        Self {
            name,
            school
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn school(&self) -> &String {
        &self.school
    }
}

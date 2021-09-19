pub struct Profile {
    name: String,
    school: String,
    formkey: String,
}

impl Profile {
    pub fn new(name: String, school: String, formkey: String) -> Self {
        Self {
            name,
            school,
            formkey
        }
    }
    
    pub fn formkey(&self) -> &String {
        &self.formkey
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn school(&self) -> &String {
        &self.school
    }
}

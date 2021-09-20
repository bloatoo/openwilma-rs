#[derive(Debug, Clone)]
pub enum UserType {
    Teacher, //Teacher
    Student, //Student
    Personnel, //School personnel
    Guardian, //Parent of the student
    Instructor, //Workplace instructor
    Management, //Management
    Passwd, //Wilma Account (this type means that account has to choose a role before continuing, for example if the account's type is guardian with multiple students "owned" by it)
    Unknown,
}

impl<'a> From<&'a str> for UserType {
    fn from(data: &'a str) -> Self {
        use UserType::*;

        match data {
            "teacher" => Teacher,
            "student" => Student,
            "personnel" => Personnel,
            "guardian" => Guardian,
            "instructor" => Instructor,
            "management" => Management,
            "passwd" => Passwd,
            _ => Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Profile {
    name: String,
    school: String,
    formkey: String,
    user_type: UserType,
    user_id: String,
}

impl Profile {
    pub fn new(name: String, school: String, formkey: String) -> Self {
        let mut formkey_data = formkey.split(":");
        let user_type = UserType::from(formkey_data.next().unwrap());
        let user_id = formkey_data.next().unwrap().into();

        Self {
            name,
            school,
            formkey,
            user_type,
            user_id
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

    pub fn user_type(&self) -> &UserType {
        &self.user_type
    }

    pub fn user_id(&self) -> &String {
        &self.user_id
    }
}

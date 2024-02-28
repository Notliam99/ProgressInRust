// use std::option;

mod grade;

pub struct Student {
    names: Vec<&'static str>,
    email: &'static str,
    grades: Vec<grade::Grades>,
}

impl Student {
    pub fn new() -> Self {
        Student {
            names: Vec::new(),
            email: "",
            grades: Vec::new(),
        }
    }

    pub fn user_details(&mut self, names: Option<Vec<&'static str>>, email: Option<&'static str>) {
        self.names = names.unwrap_or(Vec::new());
        self.email = email.unwrap_or("");
    }

    pub fn add_assesment(&mut self, assesment_name: &'static str, score: u8) {
        let mut grade: grade::Grades = grade::Grades::new();
        grade.set(score, assesment_name).unwrap();

        self.grades.push(grade);
    }
    //
    // pub fn get_grades(&mut self) -> Vec<grade::Grades> {
    //     let return_val: Vec<grade::Grades>;
    //     return_val
    // }
}

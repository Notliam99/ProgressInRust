use std::fmt;
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
        let grade: grade::Grades = grade::Grades {
            assesment_name: assesment_name,
            score: score,
        };

        self.grades.push(grade);
    }

    pub fn get_grades(&self) -> Vec<(&'static str, u8)> {
        let mut grades: Vec<(&'static str, u8)> = Vec::new();

        for i in &self.grades {
            let value = i.get();

            grades.push(value);
        }

        grades
    }
}

// yet to implement the display method so cant be printed still geting my head arround this :)

// impl fmt::Display for Student {
//     fn fmt(&self, f: fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "(names: {:?}, email: {}, grades: {:?})",
//             self.names, self.email, self.grades
//         )
//     }
// }

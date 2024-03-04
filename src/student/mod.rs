use std::fmt;
pub mod grade;

pub struct Student {
    pub names: Vec<String>,
    pub email: &'static str,
    pub grades: Vec<grade::Grades>,
}

impl Student {
    pub fn new() -> Self {
        Student {
            names: Vec::new(),
            email: "",
            grades: Vec::new(),
        }
    }

    pub fn user_details(&mut self, names: Option<Vec<String>>, email: Option<&'static str>) {
        self.names = names.unwrap_or(Vec::new());
        self.email = email.unwrap_or("");
    }

    pub fn add_assesment(&mut self, assesment_name: String, score: u8) {
        let grade: grade::Grades = grade::Grades {
            assesment_name: assesment_name,
            score: score,
        };

        self.grades.push(grade);
    }

    pub fn grades_to_vec(&self) -> Vec<(String, u8)> {
        let mut grades: Vec<(String, u8)> = Vec::new();

        for i in &self.grades {
            let value = i.get();

            grades.push(value);
        }

        grades
    }
}

// Display Sturct used when used in a print macro or somthing alike.
impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut names: String = String::new();

        {
            names.push_str("[");

            for i in &self.names {
                names.push_str(format!("{}, ", i).as_str());
            }

            names.replace_range((names.len() - 2).., "");

            names.push_str("]");
        }

        let mut grade_str: String = String::new();

        if self.grades.len() != 0usize {
            grade_str.push_str("[");

            for i in &self.grades {
                grade_str.push_str(format!("{}, ", i).as_str())
            }

            grade_str.replace_range((grade_str.len() - 2).., "");

            grade_str.push_str("]");
        } else {
            grade_str.push_str("[]");
        }

        write!(
            f,
            "(names: {}, email: ({}), grades: {})",
            names, self.email, grade_str
        )
    }
}

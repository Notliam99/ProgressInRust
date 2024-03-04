use std::fmt;

pub struct Grades {
    pub assesment_name: String,
    pub score: u8,
}

impl Grades {
    pub fn get(&self) -> (String, u8) {
        (self.assesment_name.clone(), self.score)
    }
}

impl fmt::Display for Grades {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(assesment_name: {}, score: {})",
            self.assesment_name, self.score
        )
    }
}

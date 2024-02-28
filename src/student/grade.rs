pub struct Grades {
    assesment_name: &'static str,
    score: u8,
}

impl Grades {
    pub fn new() -> Self {
        Grades {
            score: 0,
            assesment_name: "",
        }
    }

    pub fn set(&mut self, score: u8, assesment_name: &'static str) -> Result<(), &'static str> {
        if score > 100 {
            return Err("Error: Score Is Greater than 100");
        }
        self.assesment_name = assesment_name;
        self.score = score;
        Ok(())
    }

    pub fn get(&self) -> (&'static str, u8) {
        (self.assesment_name, self.score)
    }
}

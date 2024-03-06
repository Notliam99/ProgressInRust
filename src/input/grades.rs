use crate::student::grade::Grades;
use std::io::{stdin, stdout, Write};

pub fn add_grade() -> Grades {
    print!("Enter the name of the assesment\n\n> ");
    stdout().flush().expect("Error: Couldnot flush term output");

    let mut assesment_name: String = String::new();

    stdin()
        .read_line(&mut assesment_name)
        .expect("Error Cant Read Line From Console");

    assesment_name = assesment_name.trim().to_string();

    if assesment_name == "" {
        assesment_name = "No Name Was Set".to_string()
    }

    let mut assesment_score: i64 = 111;
    let mut _assesment_score_string: String = String::new();

    while assesment_score < 0 || assesment_score > 100 {
        _assesment_score_string = String::new();

        print!("Enter the Score For The assesment\nScore Must Be Between 0 - 100\n\n> ");

        stdout().flush().expect("Error: Couldnot flush term output");

        stdin()
            .read_line(&mut _assesment_score_string)
            .expect("Error Cant Read Line From Console");

        // println!("{assesment_score}");

        assesment_score = _assesment_score_string.trim().parse().unwrap_or(111);
    }

    Grades {
        assesment_name,
        score: assesment_score as u8,
    }
}

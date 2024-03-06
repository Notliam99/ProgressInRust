use std::io::{stdin, stdout, Write};

pub fn names() -> Vec<String> {
    let mut names: Vec<String> = vec![
        String::from("first name"),
        String::from("middle name"),
        String::from("last name"),
    ];

    for i in &mut names {
        print!("Enter Your {} Bellow\n\n> ", i);

        *i = String::new();

        stdout().flush().expect("Error: Couldnot flush term output");

        stdin()
            .read_line(&mut *i)
            .expect("Error: Cant Read Line From Console");

        *i = i.trim().to_string();

        if i == "" {
            *i = "Nothing Entered".to_string()
        }
    }

    names
}

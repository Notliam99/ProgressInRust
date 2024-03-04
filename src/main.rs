use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};

use std::{
    io::{stdin, stdout, Write},
    thread::sleep,
    time::Duration,
    vec,
};

use progress_in_rust::student::{grade::Grades, Student};

fn names() -> Vec<String> {
    let mut names: Vec<String> = vec![
        String::from("first name"),
        String::from("middle name"),
        String::from("last name"),
    ];

    for i in &mut names {
        print!("Enter Your {} Bellow\n\n> ", i);

        stdout().flush().expect("Error: Couldnot flush term output");

        stdin()
            .read_line(&mut *i)
            .expect("Error: Cant Read Line From Console");

        *i = i.trim().to_string()
    }

    names
}

fn add_grade() -> Grades {
    print!("Enter the name of the assesment\n\n> ");
    stdout().flush().expect("Error: Couldnot flush term output");

    let mut assesment_name: String = String::from("No Name Was Entered");

    stdin()
        .read_line(&mut assesment_name)
        .expect("Error Cant Read Line From Console");

    let mut assesment_score_temp: i64 = 111;
    let mut assesment_score: String = String::new();

    while { 0 > assesment_score_temp } && { assesment_score_temp > 100 } {
        assesment_score = String::new();

        print!("Enter the Score For The assesment\nScore Must Be Between 0 - 100\n\n> ");

        stdout().flush().expect("Error: Couldnot flush term output");

        stdin()
            .read_line(&mut assesment_score)
            .expect("Error Cant Read Line From Console");

        assesment_score_temp = assesment_score.parse().unwrap();
    }

    Grades {
        assesment_name: assesment_name,
        score: assesment_score.parse().unwrap(),
    }
}

fn main() {
    execute!(stdout(), EnterAlternateScreen)
        .expect("Uncaught Error: Couldnot Leave Alternate Screen");

    println!("Hello, world!");

    let names: Vec<String> = names();

    let mut me = Student {
        names: names.clone(),
        email: "",
        grades: Vec::new(),
    };

    me.grades.push(add_grade());

    println!("{}", me);

    sleep(Duration::from_secs(5));

    execute!(stdout(), LeaveAlternateScreen)
        .expect("Uncaught Error: Couldnot Leave Alternate Screen");
}

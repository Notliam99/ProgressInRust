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

fn add_grade() -> Grades {
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

        println!("{assesment_score}");

        assesment_score = _assesment_score_string.trim().parse().unwrap_or(111);
    }

    Grades {
        assesment_name: assesment_name,
        score: assesment_score as u8,
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

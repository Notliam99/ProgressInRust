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

use progress_in_rust::student;

fn names() -> Vec<String> {
    let mut names: Vec<String> = vec![
        String::from("first name"),
        String::from("middle name"),
        String::from("lastname"),
    ];

    /*  let mut buffer = String::new(); */

    for i in &mut names {
        print!("Enter Your {} Bellow\n\n> ", i);

        stdout().flush().expect("Error: Couldnot flush term output");

        stdin()
            .read_line(&mut *i)
            .expect("Error: Cant Read Line From Console");

        /* let name: String = buffer; */
    }

    names
}

fn main() {
    execute!(stdout(), EnterAlternateScreen)
        .expect("Uncaught Error: Couldnot Leave Alternate Screen");

    println!("Hello, world!");

    let _names = names();

    // for i in _names {
    // println!("{i}")
    // }

    println!("{:?}", _names);

    sleep(Duration::from_secs(5));

    // let mut student_liam: student::Student = student::Student::new();
    //
    // student_liam.user_details(Some(vec!["first", "last"]), Some("wow@gmail.com"));
    //
    // let _results = student_liam.grades_to_vec();
    //
    // println!("\n\n{}", student_liam);
    //
    // let student_var: student::Student = student::Student {
    //     names: vec!["0", "1", "2"],
    //     email: "1",
    //     grades: vec![student::grade::Grades {
    //         assesment_name: "0",
    //         score: 0,
    //     }],
    // };
    //
    // println!("{}", student_var);

    execute!(stdout(), LeaveAlternateScreen)
        .expect("Uncaught Error: Couldnot Leave Alternate Screen");
}

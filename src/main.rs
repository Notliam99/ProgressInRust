use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};

use std::io::{stdin, stdout};

mod student;

fn main() {
    execute!(stdout(), EnterAlternateScreen)
        .expect("Uncaught Error: Couldnot Leave Alternate Screen");

    println!("Hello, world!");

    let mut input: String = String::new();

    stdin()
        .read_line(&mut input)
        .expect("Uncaught Error: Cant Read Line From Console");

    let mut student_liam: student::Student = student::Student::new();

    execute!(stdout(), LeaveAlternateScreen)
        .expect("Uncaught Error: Couldnot Leave Alternate Screen");
}

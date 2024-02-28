// use crossterm::{
//     execute,
//     terminal::{EnterAlternateScreen, LeaveAlternateScreen},
// };

use std::io::{stdin, stdout, Write};

mod student;

fn main() {
    // execute!(stdout(), EnterAlternateScreen)
    //     .expect("Uncaught Error: Couldnot Leave Alternate Screen");

    println!("Hello, world!");

    let mut input: String = String::new();

    stdout().flush().expect("Error: Couldnot flush term output");

    stdin()
        .read_line(&mut input)
        .expect("Error: Cant Read Line From Console");

    let mut student_liam: student::Student = student::Student::new();

    student_liam.add_assesment("wow", 50);
    student_liam.add_assesment("wow", 50);

    let results = student_liam.get_grades();

    println!("\n\n{:?}", student_liam);

    // execute!(stdout(), LeaveAlternateScreen)
    //     .expect("Uncaught Error: Couldnot Leave Alternate Screen");
}

use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::{io::stdout};

use ctrlc;

use progress_in_rust::{
    input::{ask_if_loop::ask_if_loop, grades::add_grade, names::names},
    student::Student,
};

fn main() {
    execute!(
        stdout(),
        EnterAlternateScreen,
        Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .expect("Uncaught Error: Couldnot Enter Alternate Screen");

    ctrlc::set_handler(move || {
        execute!(stdout(), LeaveAlternateScreen)
            .expect("Uncaught Error: Couldnot Leave Alternate Screen");
        std::process::exit(1)
    })
    .unwrap();

    let mut students: Vec<Student> = Vec::new(); 

    loop {
        let names: Vec<String> = names();

        let mut _student = Student {
            names: names.clone(),
            email: "",
            grades: Vec::new(),
        };

        loop {
            _student.grades.push(add_grade());

            println!("{}", _student);
            
            print!("Add Another Grade");

            if ask_if_loop() {
                break;
            }
        }

        students.push(_student);
        
        print!("Add Another User");

        if ask_if_loop() {
            break;
        }
    }

    execute!(stdout(), LeaveAlternateScreen)
        .expect("Uncaught Error: Couldnot Leave Alternate Screen");

    for student in students {
        println!("{student}");
    }
}

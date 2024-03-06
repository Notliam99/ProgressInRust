use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::{io::stdout, thread::sleep, time::Duration};

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

    let names: Vec<String> = names();

    let mut me = Student {
        names: names.clone(),
        email: "",
        grades: Vec::new(),
    };
    loop {
        me.grades.push(add_grade());

        println!("{}", me);

        sleep(Duration::from_secs(5));

        if ask_if_loop() {
            break;
        }
    }

    println!("{me}");

    execute!(stdout(), LeaveAlternateScreen)
        .expect("Uncaught Error: Couldnot Leave Alternate Screen");
}

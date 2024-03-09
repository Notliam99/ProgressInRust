// use crossterm::{
//     cursor, execute,
//     terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
// };

use std::io::{stdin, stdout, Write};

pub fn ask_if_loop() -> bool {
    print!(" [Y es / N o]\n\n> ");
    stdout()
        .flush()
        .expect("Error: Could Not Flush Terminal Output");

    let mut yay_or_nay: String = String::new();

    stdin()
        .read_line(&mut yay_or_nay)
        .expect("Error Cant Read Line From Console");

    println!("{}", yay_or_nay.contains("y"));

    if yay_or_nay.contains("y") || yay_or_nay.contains("Y") {
        false
    } else if yay_or_nay.contains("n") || yay_or_nay.contains("N") {
        true
    } else {
        println!("Didnt Reconise Input So Continueing");
        false
    }
}

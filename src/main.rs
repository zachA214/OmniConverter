mod title_menu;

use std::io;

struct Option<'a> {
    value: &str,
    param_count: u32,
}


//handles each iteration of the user input
//1. asks for input
//2. compares input to see if its a valid command
//3. if it is a valid command
//   a. run command
//   b. if command is exit command return false
//4. if it is not valid command, warn user
//In all cases other than exit command, return true
fn handler() -> bool{
    let mut command: String = String::new(); // Create a string variable
    io::stdin() // Get the standard input stream
        .read_line(&mut command) // The read_line function reads data until it reaches a '\n' character
        .expect("Unable to read Stdin"); // In case the read operation fails, it panics with the given message

    print!("Mocks your {}", command);
    //println!("Mocks your {}", command);
    return true;
}

fn main() {
    title_menu::display_title();
    while handler(){

    }
}

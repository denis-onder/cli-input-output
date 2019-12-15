use std::io;
use std::string::String;
use std::vec::Vec;

fn display_inputs(vec: Vec<String>) {
    println!("\n//----- Output -----");
    let mut counter = 0;
    for i in vec {
        counter += 1;
        println!("{}. {}", counter, i);
    }
}

fn get_input() {
    // Toggle condition
    let mut done = false;
    let mut vec: Vec<String> = Vec::new();
    while !done {
        println!("{}. Your input: (exit to quit)", vec.len());
        let mut user_input = String::new();
        // Read user input
        io::stdin()
            .read_line(&mut user_input)
            .expect("An error has occured.");
        // Trim whitespace
        let res = String::from(user_input.trim());
        /* If user_input == "done", toggle the variable
         * Else, push the value to the output vector
         */
        if res == "exit" {
            done = true;
        } else {
            vec.push(res);
        }
    }
    display_inputs(vec);
}

fn main() {
    get_input();
}

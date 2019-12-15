use std::fs::File;
use std::fs::OpenOptions;
// use std::io::prelude::*;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;
use std::string::String;
// use std::vec::Vec;

// fn display_inputs(vec: Vec<String>) {
//     println!("\n//----- Output -----");
//     let mut counter = 0;
//     for i in vec {
//         counter += 1;
//         println!("{}. {}", counter, i);
//     }
// }

fn display_inputs() -> io::Result<()> {
    let file_path: &Path = Path::new("./store.txt");
    println!("\n//----- Output -----");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
    // let mut counter = 0;
}

fn store_to_file(todo: String) {
    let file_path: &Path = Path::new("./store.txt");
    // Create file if it does not exist.
    if !file_path.exists() {
        let _ = File::create(file_path);
    }
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", todo) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn get_input() {
    // Toggle condition
    let mut done = false;
    let mut counter = 1;
    // let mut vec: Vec<String> = Vec::new();
    while !done {
        println!("{}. Your input: (exit to quit)", counter);
        let mut user_input = String::new();
        // Read user input
        io::stdin()
            .read_line(&mut user_input)
            .expect("An error has occured.");
        // Trim whitespace
        let res = String::from(user_input.trim());
        /* If user_input == "done", toggle the variable.
         * Else, push the value to the output vector.
         * NOTE: Moving the storage to a text file.
         */
        if res == "exit" {
            done = true;
        } else {
            // vec.push(res);
            store_to_file(res);
            counter += 1;
        }
    }
    let _ = display_inputs();
}

fn main() {
    println!("Rust -> CLI Todo App:");
    get_input();
}

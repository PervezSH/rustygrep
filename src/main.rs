use std::env; // to use args function provided by rust's standard library
use std::fs; // to handle files

fn main() {
    let args: Vec<String> = env::args().collect(); // collect() turns iterator into vector

    let query = &args[1];
    let file = &args[2];

    print!("Searching for {} in file {}", query, file);

    let contents = fs::read_to_string(file).expect("Something went wrong while reading the file!");
    print!(" with text: \n{}", contents);
}

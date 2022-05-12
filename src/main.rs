use std::env; // to use args function provided by rust's standard library

fn main() {
    let args: Vec<String> = env::args().collect(); // collect() turns iterator into vector

    let query = &args[1];
    let file = &args[2];

    println!("Searching for {} in file {}", query, file);
}

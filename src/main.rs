use rustygrep::Params; // import Params Struct
use std::env; // to use args function provided by rust's standard library
use std::process; // to exit program without panicking

fn main() {
    let args: Vec<String> = env::args().collect(); // collect() turns iterator into vector
    let params = Params::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {} in {}...", params.query, params.file);
    if let Err(e) = rustygrep::run(params) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

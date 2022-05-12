use std::env; // to use args function provided by rust's standard library
use std::fs; // to handle files

struct Params<'a> {
    query: &'a String,
    file: &'a String,
}

impl<'a> Params<'a> {
    fn new(args: &[String]) -> Params {
        let query = &args[1];
        let file = &args[2];
        Params { query, file }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect(); // collect() turns iterator into vector
    let params = Params::new(&args);

    print!("Searching for {} in file {}", params.query, params.file);

    let contents =
        fs::read_to_string(params.file).expect("Something went wrong while reading the file!");
    print!(" with text: \n{}", contents);
}

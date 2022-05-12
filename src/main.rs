use std::env; // to use args function provided by rust's standard library
use std::fs; // to handle files
use std::process; // to exit program without panicking

fn main() {
    let args: Vec<String> = env::args().collect(); // collect() turns iterator into vector
    let params = Params::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    print!("Searching for {} in file {}", params.query, params.file);

    let contents =
        fs::read_to_string(params.file).expect("Something went wrong while reading the file!");
    print!(" with text: \n{}", contents);
}

struct Params<'a> {
    query: &'a String,
    file: &'a String,
}

impl<'a> Params<'a> {
    fn new(args: &[String]) -> Result<Params, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = &args[1];
        let file = &args[2];
        Ok(Params { query, file })
    }
}

use std::error::Error;
use std::fs; // to handle files

// in success return unit type and in error case we return an error
// Box<dyn Error> means the function will return a type that implements the Error trait
pub fn run(params: Params) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(params.file)?;
    print!("With text: \n{}", contents);
    Ok(())
}

pub struct Params<'a> {
    pub query: &'a String,
    pub file: &'a String,
}

impl<'a> Params<'a> {
    pub fn new(args: &[String]) -> Result<Params, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = &args[1];
        let file = &args[2];
        Ok(Params { query, file })
    }
}

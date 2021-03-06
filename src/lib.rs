use std::env;
use std::error::Error;
use std::fs; // to handle files

// in success return unit type and in error case we return an error
// Box<dyn Error> means the function will return a type that implements the Error trait
pub fn run(params: Params) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(params.file)?;
    let results = if params.case_sensitive {
        search_case_insensitive(&params.query, &contents)
    } else {
        search(&params.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub struct Params<'a> {
    pub query: &'a String,
    pub file: &'a String,
    pub case_sensitive: bool,
}

impl<'a> Params<'a> {
    pub fn new(args: &[String]) -> Result<Params, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = &args[1];
        let file = &args[2];
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();
        Ok(Params {
            query,
            file,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

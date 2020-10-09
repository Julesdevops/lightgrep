use std::fs;
use std::env;
use std::error::Error;

// Struct that holds the arguments/options passed in
// Also hold useful environment informations from env vars
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    // Validates arguments/options and return a new Config instance
    // Or, return an error
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        //TODO refactor : don't use clone as it is not efficient
        let query = args[1].clone();
        let filename = args[2].clone();

        //TODO style : change env var name to LGREP_CASE_INSENSITIVE
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        
        Ok(Config { query, filename, case_sensitive })
    }
}

// Core business logic, find matches and display them
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Get file content as a String
    let contents = fs::read_to_string(config.filename)?;

    // Get matches, case sensitive or else depending on config
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    // Display matches
    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// Perform a case sensitive search
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // TODO refactor
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

// Perform a case insensitive search
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    //TODO refactor
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// Unit tests for search and search_case_insensitive methods
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
Duct tape.";

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

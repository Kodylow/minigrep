use std::error::Error;
use std::{env, fs};

// runs execution logic of program
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read file contents into a single string woah that's cool
    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}

// search function of grep, finds query within contents
// lifetime required because data referenced BY a slice needs to be valid
// compiler would assume we're making string slices of query vs contents otherwise
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// search function of grep, case_insensitive if env variable triggered
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

// extracts and returns a Config struct from 2 cmd linearguments,
// query and filename. index 0 is this program's name.
// example: cargo run test sample.txt

impl Config {
    //Return a Result for better error handling
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // pop through first arg
        args.next();

        // args[1]
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        
        // args[2]
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
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

        assert_eq!(vec!["Rust: ",], search_case_insensitive(query, contents));
    }
}

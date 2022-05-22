use std::error::Error;
use std::fs;

// runs execution logic of program
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read file contents into a single string woah that's cool
    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

// extracts and returns a Config struct from 2 cmd linearguments,
// query and filename. index 0 is this program's name.
// example: cargo run test sample.txt

impl Config {
    //Return a Result for better error handling
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

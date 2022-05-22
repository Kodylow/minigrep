use minigrep::Config;
use std::{env, process};

fn main() {
    // iterators produce a series of values, collect turns the vals into a collection
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        // unwrap_or_else lets you handle error message
        // instead of throwing panic on bad unwraps
        eprintln!("Problem parsing arguments: {}", err);
        // A nonzero exit status is a convention to signal to the
        // that the program exited with an error state.
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}

use minigrep::Config;
use std::{env, process};

fn main() {
    // env args returns an iterator just use that!
    let config = Config::new(env::args()).unwrap_or_else(|err| {
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

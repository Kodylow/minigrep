use std::env;

fn main() {
    // iterators produce a series of values, collect turns the vals into a collection
    //2 arguments, query and filename. index 0 is this program's name

    //example: cargo run test sample.txt
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}

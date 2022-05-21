use std::env;

fn main() {
    // iterators produce a series of values, collect turns the vals into a collection
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}

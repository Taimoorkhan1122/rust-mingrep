use std::{env, fs}; 

fn main() {
    // env::args() returns an iterator of command line arguments
    let args : Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];
    println!("Searching for: {:?} \nin: {:?}", query, filename);

    let contents = fs::read_to_string("./poem.txt")
        .expect("something went wrong!");
    println!("with contents: \n{}", contents)
}

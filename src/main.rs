use std::{env, fs};

fn main() {
    // env::args() returns an iterator of command line arguments
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    println!("Searching for: {:?} \nin: {:?}", config.query, config.filename);

    let contents = fs::read_to_string("./poem.txt").expect("something went wrong!");
    println!("with contents: \n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename }
    }
}

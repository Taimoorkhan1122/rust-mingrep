use std::{env, fs, process};

fn main() {
    // env::args() returns an iterator of command line arguments
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error parsing the arguments: {}", err);        
        process::exit(1);
    });
    println!(
        "Searching for: {:?} \nin: {:?}",
        config.query, config.filename
    );

    let contents = fs::read_to_string("./poem.txt").expect("something went wrong!");
    println!("with contents: \n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        };

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

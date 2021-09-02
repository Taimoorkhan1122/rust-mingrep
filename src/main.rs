use std::{env, fs, process};

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

fn main() {
    // env::args() returns an iterator of command line arguments
    let args: Vec<String> = env::args().collect();

    // unwrap_or_else extracts OK value  and helps to define non panic errors
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error parsing the arguments: {}", err);
        process::exit(1);
    });
    println!(
        "Searching for: {:?} \nin file: {:?}",
        config.query, config.filename
    );
    run(config);
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.filename).expect("something went wrong!");
    println!("with contents: \n{}", contents);
}

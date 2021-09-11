use std::{env, process};

use minigrep::Config;

fn main() {
    // env::args() returns an iterator of command line arguments
    // let args: Vec<String> = env::args().collect();

    // unwrap_or_else extracts OK value  and helps to define non panic errors
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // this will send error to standard error stream
        eprintln!("Error parsing the arguments: {}", err);
        process::exit(1);
    });
    println!(
        "Searching for: {:?} \nin file: {:?}",
        config.query, config.filename
    );
    if let Err(e) = minigrep::run(config)  {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
    
}   


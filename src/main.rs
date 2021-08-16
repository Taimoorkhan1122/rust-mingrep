use std::env; 

fn main() {
    // env::args() returns an iterator of command line arguments
    let args : Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];
    println!("Searching for: {:?} \n in: {:?}", query, filename);
}

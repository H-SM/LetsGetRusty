use std::env;
//to exit the program w/o panic
use std::process;

use minigrep::FileHere;

fn main() {
    let args : Vec<String> = env::args().collect();
    //first index is the binary path

    let config = FileHere::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing the arguments : {}", err);
        process::exit(1);
    });

    println!("Searching for {}",config.query);
    println!("in the file : {}",config.filename);

    
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error : {}",e);
        process::exit(1);
    }
    // run(config);
}
// cargo run test poem.txt
// cargo run > output.txt 
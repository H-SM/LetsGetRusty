use std::env;
use std::fs;

//to exit the program w/o panic
use std::process;
use std::error::Error;

fn main() {
    let args : Vec<String> = env::args().collect();
    //first index is the binary path

    let config = FileHere::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing the arguments : {}", err);
        process::exit(1);
    });

    println!("Searching for {}",config.query);
    println!("in the file : {}",config.filename);

    
    if let Err(e) = run(config) {
        println!("Application error : {}",e);
        process::exit(1);
    }
    // run(config);
}

fn run(config : FileHere) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;

    println!("content : {}",contents);

    Ok(())
}

//showing the relationship between query and filename
struct FileHere {
    query : String,
    filename : String,
}

//adding the parse_Config to the struct
impl FileHere {
    fn new(args : &[String]) -> Result<FileHere, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok(FileHere {query, filename})
    }
    //new -> constructor function
}

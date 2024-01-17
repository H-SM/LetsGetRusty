use std::{fs, env};
use std::error::Error;

pub fn run(config : FileHere) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    }else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}",line);
    }

    Ok(())
}

//showing the relationship between query and filename
pub struct FileHere {
    pub query : String,
    pub filename : String,
    pub case_sensitive : bool,
}

//adding the parse_Config to the struct
impl FileHere {
    pub fn new(mut args : env::Args) -> Result<FileHere, &'static str> {
        args.next(); //path
        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("didn't get a query"),
        };
    
        let filename = match args.next(){
            Some(arg) => arg,
            None => return Err("didn't get a query"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(FileHere {query, filename, case_sensitive})
    }
    //new -> constructor function
}

pub fn search<'a>(query : &str , content : &'a str ) -> Vec<&'a str> {
    // let mut res = Vec::new();
    
    // for line in content.lines() {
    //     if line.contains(query) {
    //         res.push(line);
    //     }
    // }

    // res

    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query : &str , content : &'a str ) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut res = Vec::new();
    
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line);
        }
    }

    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust: 
safe, fast, productive.
Pick three.
Duct Tape";

        assert_eq!(vec!["safe, fast, productive."],search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        
        assert_eq!(vec!["Rust:","Trust me."],search_case_insensitive(query, contents))
    }
}

//  cargo run world poem.txt                                          

// Compiling minigrep v0.1.0 (/home/hsm/Projects/LetsGetRusty/minigrep)
//  Finished dev [unoptimized + debuginfo] target(s) in 0.19s
//   Running `target/debug/minigrep world poem.txt`
// Searching for world
// in the file : poem.txt
// the world was grey,
// the world was cold,
// the world was gone.
// the world was calm,
// the world was brown,
// the world was born
// the world was grey,

// export CASE_INSENSITIVE=true                                      

// cargo run world poem.txt                                         
//  Finished dev [unoptimized + debuginfo] target(s) in 0.00s
//   Running `target/debug/minigrep world poem.txt`
// Searching for world
// in the file : poem.txt
// the world was grey,
// the world was cold,
// the world was gone.
// the world was calm,
// the world was brown,
// the world was born
// the world was grey,

// cargo run worlD poem.txt                                         
//  Finished dev [unoptimized + debuginfo] target(s) in 0.00s
//   Running `target/debug/minigrep worlD poem.txt`
// Searching for worlD
// in the file : poem.txt
// the world was grey,
// the world was cold,
// the world was gone.
// the world was calm,
// the world was brown,
// the world was born
// the world was grey,

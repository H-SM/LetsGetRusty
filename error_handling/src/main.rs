fn main() {
    // panic!("crash and burn!");
    a();
    main_2();
}

fn a() {
    b();
}

fn b() {
    c(21);
}
fn c(num : i32) {
    if num == 22 {
        panic!("Don't pass 22 number!");
    }
}

fn errors_here() {
    enum Result<T,E> {
        Ok(T),
        Err(E),
    }
    // this is close to option enum, but rather than Some or none, this shows Ok(Success) or Err 
}

use std::error;
use std::fs::{self,File};
use std::io::ErrorKind;
fn main_2() {
    let f = File::open("hello.txt"); 

    // let f = match f { 
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem in opening the file : {:?}",error), 
    // };

    //If we wnat the error to get out gracefully rather than a panic state, we can do this ->
    let f = match f { 
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc, 
                Err(e) => panic!("Problem creating a new file : {:?}",e),
            },
            other_error => {
                panic!("Problem in opening the file : {:?}",other_error)
            }
        }, 
    };
    
}

fn main_3() {
    //using closure in the abv -> 

    let f = File::open("hello.txt").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|e|{
                panic!("Problem creating a new file : {:?}",e);
            })
        }else {
            panic!("Problem in opening the file : {:?}",error)
        }
    }); 
    
}

fn unwrapping_here() { 
    // let f = File::open("hello.txt").unwrap(); 
    let f = File::open("hello.txt").expect("Problem in opening the file : hello.txt"); 

    // let f = match f { 
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem in opening the file : {:?}",error), 
    // };
}

//ERROR PROPOGATION

// use std::fs::File;
use std::io;
use std::io::Read;
use std::net::IpAddr;

fn read_username_from_file() -> Result<String, io::Error>{
    let mut f = File::open("hello.txt")?; 
    // ? => panic state looker over the file extraction (returns the error which occured in the condition) [CAN ONLY BE USED IN A SUB FUNCTION AND NOT IN MAIN - nothing to return back the result]
    
    // let mut f = match f { 
    //     Ok(file) => file,
    //     Err(error) => return Err(error), 
    // };

    let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_smaller() -> Result<String, io::Error>{
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_even_smaller() -> Result<String, io::Error>{
    fs::read_to_string("hello.txt")
}

//IF WE WANT TO HANDLE THIS IN THE MAIN FUNCTION 
// fn main() -> Result<(), Box<dyn Error>> {
//     let f = File::open("hello.txt")?;

//     Ok(())
// }

//IF WE KNOW THE FUNCTION WILL BE OUT 
fn function_here() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();
}

//error handling in Guessing game -> 
pub struct Guess {
    value : i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("number out of the range of 1 and 100!")
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    } 
}
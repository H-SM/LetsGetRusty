use std::io;
use std::cmp::Ordering;
use rand::Rng;
// we bring the range tree in the scope abv
use colored::*;
fn main() {
    println!("Hello, world!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    

    println!("The random number : {}",secret_number);
    loop{
        println!("Please input a number");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 =match guess.trim().parse(){
            Ok(num)=> num,
            Err(_) => continue,
        };
        // we shadowed guess here mathing the expectation of becoming a u32
        
        println!("You guessed : {}",guess);
        

        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too Big!".red()),
            Ordering::Equal => {
                println!("{}", "BINGO!!!".green());
                break;
            }
        }
    }
    
}

// the code base represents the front of the house(restaurant) - a modal for it in the library of commands we could use in lib.rs
// mod front_of_house { 
//     mod hosting { 
//         fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }
//     // a module could have another module, struct, enums, instance, traits, etc..
//     mod serving { 
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}

//     }
// }

//a crate is also a module which is default for crate root (main.rs, ... any bin file with a .rs)

//Looking over the paths we have in hand ( in the module) the variable properties att our disposal (path to the module partition (functoins in it) from the root crate (the main.rs))
mod front_of_house_2 { 
    pub mod hosting { 
        pub fn add_to_waitlist() {}

    }
}

pub fn eat_at_restaurant() { 

    // an absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // a relative path (from current module)
    front_of_house::hosting::add_to_waitlist();

    //THINK CRATES/MODULES AND PATH AS DIRECTORIES
    // BY Default the child module is private from parent thus, front_of_house cant see hosting module abv
    // we can use "pub" to make the module public for anyone to take the properties from
}

// fn serve_order() {}

// mod back_of_house {
//     fn fix_incorrect_order(){
//         cook_order();
//         super::serve_order();
//         //we can call the the 2 functions here relatively (super to ref the parent module)
//     }

//     fn cook_order() {}
// }


mod back_of_house {
    
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer (toast: &str) -> Breakfast {
            Breakfast {
                toast : String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant_2() {
    let mut meal = back_of_house::Breakfast::summer("brown");

    meal.toast = String::from("Wheat");

    //we cant make the Breakfast struct directly like -> 
    // let meal2 = back_of_house::Breakfast { 
    //     toast: String::from("Wheat"),
    //     seasonal_fruit: String::from("peaches"),
    // }
    // we will again get the error of the seasonal_fruit begin private
}

mod back_of_house_2 {   
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
// if you make a enum as public all it's variant will be public as well unlike the case in the struct abv
pub fn eat_at_restaurant_3() {
    let order1 = back_of_house_2::Appetizer::Soup;
    let order2 = back_of_house_2::Appetizer::Salad;
}



mod front_of_house;
//use the front_of_house module here from front_of_house.rs

pub use crate::front_of_house_2::hosting;
//FOR EXPORTING THE PATH TO THE PARENT MODULE TO ANOTHER FILE WE WILL MAKE THE PATH "PUB" HERE SO THAT AN EXTERNAL FILE CAN USE THE FUNCTIONS/FEATURES IN THE MODULE LIKE -> hosting::add_to_waitlist();

// use crate::front_of_house_2::hosting;
// use self::front_of_house_2::hosting;
// self refers to the current module 

pub fn eat_at_restaurant_4() { 

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    //Use -> allows the path to bring into the scope 
    // eg -> bringing the hosting module above in the scope
}

//as an expection if we require 2 modules in a scope not 1 as abv we "use" the path to the parent module for the variant features

// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
//     // ...
// }

// fn function2() -> io::Result<()> {
//     // ...
// }

// OR simply rename the another path in use

use std::fmt::Result;
use std::io::Result as IOResult;

fn function1() -> Result {
    // ...
    Ok(())
}

fn function2() -> IOResult<()> {
    // ...
    Ok(())
}

//using another module in another file (using the path)
// use rand::Rng;
// use rand::ErrorKind::Transient;
// use rand::CryptoRng;

use rand::{Rng, CryptoRng, ErrorKind::Transient};

// use std::io;
// use std::io::Write;

use std::io::{self, Write};
use std::io::*;
//all files in it

fn create_random() {
    let secret = rand::thread_rng().gen_range(1, 100);
}


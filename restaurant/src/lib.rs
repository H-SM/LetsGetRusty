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
mod front_of_house { 
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
    
    struct Breakfast {
        toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        fn summer (toast: &str) -> Breakfast {
            Breakfast {
                toast : String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant_2() {
    let mut meal = back_of_house::Breakfast::summer("TOAST");
}
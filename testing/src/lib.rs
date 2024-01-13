pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle { 
    w : u32, 
    h : u32,
}

impl Rectangle { 
    fn can_hold(&self,  other : &Rectangle) -> bool {
        self.w > other.w && self.h > other.h
    }
}

pub struct Guess {
    value : i32,
}

impl Guess {
    pub fn new(value : i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("The guessed numbber so out of bounds");
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4); //asserts equal
        // assert_ne!(result, 4); //asserts not equal
    }

    #[test]
    // fn failing_test() {
    //     panic!("this is a failing test");
    // }
    fn larger_can_hold_smaller() {
        let larger = Rectangle { 
            w : 7,
            h : 7
        };
        let smaller = Rectangle {
            w : 3, 
            h : 4
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn greeting_the_name() {
        let res = greeting("hsm");
        //custom failure message 
        assert!(res.contains("hsm"),
        "can't find the name `{}`",
        res);
        // can't find the name `hello hsm!`
    } 

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(1000);
    }

    //Returning result tests
    #[test]
    fn it_works2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        }else{
            Err(String::from("two plus two is not four!"))
        }
    }
}


pub fn greeting(name : &str) -> String {
    format!("hello {}!",name)
} 
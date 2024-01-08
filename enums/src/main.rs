enum IpAddressKind { 
    v4(u8, u8, u8, u8),
    v6(String),
}
// this represent a version 6 type of string (for ip address)
// this represent a version 4 type of 4 -> 8 bit int (for ip address)


enum Message { 
    Quit,
    Move { x: i32, y : i32 }, //this is a struct (vector in cpp)
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message {
    fn some_func(){
        println!("Hi this is HSM!");
        
    }
}
struct IpAddr { 
    kind : IpAddressKind,
    address : String,
}
fn main() {
    let four = IpAddressKind::v4;
    let six = IpAddressKind::v6;

    let localhost = IpAddressKind::v4(127,0,0,1);

    value_in_cents(Coin::Quater(UsState::Arizona));
}

fn route(ip_kind: IpAddressKind) { 

}

// we have an option enum for having a null value to a param
fn option_enum_here() {
    // enum Option<T>{
    //     Some(T),
    //     None,
    // }

    let some_number = Some(5);
    //optional int

    let some_string = Some("a string");
    let abs_number : Option<i32> = None;
    
    let string : &str = "this is good"; //this is a string literal 
    let string2 = String::from("hsm"); //this is a string 
    let string3 = &string2; //this is a string slice 

    let x : i8 = 5;
    let y : Option<i8> = Some(5);
    let sum = x + y.unwrap_or(0); //if the unwrapping value is none over the option, use default value '0' here
}

//matching cases over enum 
#[derive(Debug)]
enum UsState { 
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    // ...
}

enum Coin { 
    Penny,
    Nickel, 
    Dime,
    Quater(UsState),
}

fn value_in_cents(coin: Coin) -> u8 { 
    match coin { 
        Coin::Penny => {
            println!("this is a penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater(state) => {
            println!("State quater from {:?}!",state);
            25
        },
    }
}

fn plus_one(x : Option<i32>) -> Option<i32> { 
    match x { 
        // None => None,
        Some(i) => Some(i + 1),
        _ => None //for any other values in the match tree than the defied path go to these functions (more like a default condition over multiple cases of matching)
    }
}
fn another_func() { 
    // its a lot like 'if let' function as below (less time taking and more robust as we look over on the conditions which actually matters)
    let some_variable = Some(3);
    match some_variable { 
        Some(3) => println!("Three"),
        _ => (),
    }

    if let Some(3) = some_variable {
        println!("Three");
    }
}
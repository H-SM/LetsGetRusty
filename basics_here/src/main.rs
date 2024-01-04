fn main() {
    let mut x = 5;
    println!("the value : {}",x);
    x = 6; 
    println!("the value : {}",x);
    const NO_WAY: u32 = 10_00_00_000;
    println!("the constant : {}",NO_WAY);
    
    let y = 51;
    println!("the value : {}",y);
    //SHADOWING - both mutable & we can change the types
    let y = 6; 
    println!("the value : {}",y);

    //SCALER DATA TYPES
    // integers 
    // floating-point numbers
    // booleans 
    // characters 
    // https://doc.rust-lang.org/book/ch03-02-data-types.html
    let f: u8 = 255;
    println!("the value : {}",f);

    let g = 2.0;
    let h: f32 = 234.09;

    let character = 'a';

    // tuple
    let tup = ("getting rusty!", 120921);
    let (a,b) = tup;
    let counter = tup.1;
    println!("{}",counter);

    //array 
    let an_arr = [123,32,23,12];
    let a_arr_num = an_arr[2];
    let byte = [0; 8]; //make 8 0's in the array
    let sum  = a_func(12,23);
    println!("{}",sum);

    let cnd = true;
    let number_here = if cnd { 5 } else { 1 };

    //LOOPS
    let mut c =  6; 
    let r = loop { 
        c += 1;
        if c == 10 { 
            break c;
        }
        println!("LOOP");
    };
    println!("the result of loop: {}",r);
}

fn a_func(x : i32,y: i32) -> i32 {
    println!("some func here");
    x + y
}

fn while_here() {
    let mut num = 3;
    while num != 0{
        println!("{}",num);
        num -= 1;
    }
}

fn for_loop(){
    let a = [12,23,21,23];
    for ele in a.iter() {
        print!("{}",ele);
    }

    for num in 1..4{
        print!("{}",num);
    }
}
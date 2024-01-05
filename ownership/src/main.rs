fn main() {
    // THE OWNERSHIP RULES 
    // 1. each value in rust has a variable that's the owners for the data 
    // 2. There can only be one owner at a time 
    // 3. When the owner goes out of scope, the value will be dropped
    {
        let s = "hello"; // this is immutable 
        //for making it mutable 
        let str = String::from("hello"); //"hello" on the heap 

    }

}

fn copy_it() { 
    let x = 5;
    let y = x; //copied

    let s1 = String::from("hello");
    let s2 = s1;//here s1 is shadowed and not copied but moved 
    let s3 = s2.clone();//to copy a string or a memory in the heap we clone it  
}

fn looker_own() {
    //taking the ownership
    let s = String::from("hello");
    // takes_ownership(s);
    takes_ownership(s.clone()); //this will do the job
    println!("{}",s);// s cant be borrowed from a moved value

    //giving ownership 
    let s2 = gives_ownership();
    println!("{}",s2);
    let s4 = String::from("hello");

    let s3 = takes_and_gives_back(s4);
}

fn takes_ownership(str: String) {
    println!("{}",str);
}

fn gives_ownership() -> String {
    let str = String::from("hello");
    str
}

fn takes_and_gives_back(str : String)-> String { 
    str
}


//ok the ownership is a lot hell for big codes, thus we can use references -> &s this ref is immutable by default

fn ref_from_here() {
    let s = String::from("hello");
    let  len = calculate_len(&s);
    println!("length of the '{}' is {}.",s,len); 
}

fn calculate_len(s: &String)-> usize { 
    let length = s.len();
    length
}


fn example_here() {
    let mut s1 = String::from("Hello");
    change(&mut s1);
}

fn change(some_str: &mut String){
    some_str.push_str(", world"); // we can change the val by giving a mutable reference to it (WE CAN ONLY HAVE OEN MUTABLE REF FOR A SINGLE VALUE IN A SINGLE SCOPE)
    // EG BELOW 
}
fn ref_eg_here(){
    let mut s = String::from("Hello");

    let r1 = &mut s;
    // let r2 = &mut s; //we cant do this

    print!("{}",r1);
    //WE CANNOT HAVE A MUTABLE REFERENCE IF WE HAVE AN IMMUTABLE REFERENCE TO THE DATA/VARI (we can have multiple immutable refs)
}

//A DANGLING REFERENCE 

fn dangle_ref_from_here() { 
    // let ref_to_here = dangle();
    //s gets deallocated in this stage (we could set the "lifetime" for the data here - extending the scope for the variable referred to here)
}

// fn dangle() -> &String {
//     let s = String::from("Hello");

//     &s
// }

//THE RULES  OF REFERENCES -> 
// 1. At any given time, you can have either one mutable ref on any number of immutable ref.
// 2. Ref must always be valid. (not dangling)

//SLICES 
// refer a countigous sequence of data/elements of a collection instead refering to entire collection 
// this doesn't take the ownershifind_firstp of the data

fn find_first() { 
    let mut s = String::from("Hello World");
    let str = "Hello World"; // this is a string slice NOT A STRING HEAP DATA 

    // let hello = &s[0..5]; //0,1,2,3,4
    let hello = &s[..5]; //0,1,2,3,4
    // let world = &s[6..11]; //6,7,8,9,10
    let world = &s[6..]; //6,7,8,9,10

    let word = first_word(str);
    // s.clear(); // this removes the scope for the word ref 
    println!("the first word : {}", word);
}


// fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str { //for the "str" ref here now 
    let bytes = s.as_bytes();
    
    for(i, &item) in bytes.iter().enumerate() { 
        if item == b' '{
            return &s[..i];
        }
    }   
    &s[..]
}

//we can do slice on an array (any heap data)

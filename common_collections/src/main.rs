use core::f64;
use std::collections::hash_map;

fn main() {
    let a = [1,2,3];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    {
        let mut v2 = vec![1,2,3];
    }

    let mut v3 = vec![1,2,3,4,5];
    println!("the 3rd element in v3 : {}",&v3[2]);
    v3.push(6);    
    //if we wish to prevent the index out of bound here (if a wrong index is given to the input)

    match v.get(2){
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no such index here in the vector."),
    } 

    for i in &v3 {
         print!("{} ",i);
    }
    println!("");

    //OR we could take a mutable reference
    for i in &mut v3 {
        *i += 50;
        //this abv is a de-reference model 
    }

    for i in &v3 {
        print!("{} ",i);
    }
    println!("");

    //enum reference to a vector 

    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("blue")),
    ];

    match &row[1] {
        SpreadSheetCell::Int(i) => println!("{}",i),
        _ => println!("No Integer value!")
    }

    strings_here();
    hash_map_here();
}

fn strings_here() {
    // Strings are encoded as a collection ot UTF-8 encoded bytes
    let mut str = String::from("HS");
    str.push_str("M");
    str.push('!');
    
    let s1 = String::from("HSM");
    let s2 = String::from("HERE");
    let s3 = s1 + &s2 + &String::from(".ME");

    println!("{}",s3);
    //we will get an error here if we reuse "s1" as its ownership has been shifted to "s3" now [ref. to ownership dir for more...]

    // OR we can combine in such a followign manner
    let s4 = String::from("HSM");
    println!("{}{}{}",s4,s2,String::from(".ME"));

    //the character in the strins aren't 1 byte long thus we can't use the below convention to take out the characters (Rust uses UTF-8 convension, not ASCII)
    // let c = s4[1];

    // THERE ARE 3 WAYS TO REPRESENT THE STRINGS 
    // 1. Bytes (for each character)
    println!("1. Bytes (for each character)");
    for b in s3.bytes() { // HSMHERE.ME
        print!("{} ",b); //72 83 77 72 69 82 69 46 77 69
    }
    println!("");
    // 2. Scaler values (this is the "char type")
    println!("2. Scaler values (this is the 'char type')");
    for c in s3.chars() { // HSMHERE.ME
        print!("{} ",c); //H S M H E R E . M E 
    }
    println!("");
    // 3. Grapheme clusters
    // we need to import the unicode-segmentation module
    println!("3. Grapheme clusters");
    use unicode_segmentation::UnicodeSegmentation;

    for g in s3.graphemes(true) { // HSMHERE.ME
        print!("{} ",g); //H S M H E R E . M E (each is a seperate string not a char) 
    }
    println!("");

}

use std::collections::HashMap;

fn hash_map_here() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    // println!("{}",blue);
    //we cant borrow this "blue" a moved value to the "scores";

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // println!("{}",score);
    //there is no guarantty the key wil be referenced thus we cant directly use "score" do a match here

    for (key , value) in &scores {
         println!("{},{}",key,value);
    }

    let mut scores2 = HashMap::new();

    //this will override the value
    scores2.insert(String::from("Blue"), 10);
    scores2.insert(String::from("Blue"), 50);   

    //this wont override the value (or_insert)
    scores2.entry(String::from("Yellow")).or_insert(30);   
    scores2.entry(String::from("Yellow")).or_insert(40);   

    //updating a value in the hash map 

    let text = "hsm was here . I was here";

    let mut map = HashMap::new();
    //{"hsm", "was", "here", ".", "I", "was", "here"}
    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}",map); // {"I": 1, ".": 1, "was": 2, "here": 2, "hsm": 1}
}
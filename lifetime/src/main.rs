

fn main() {
    let str1 =String::from("abcd");
    let str2 = String::from("xy");

    let result = longest(str1.as_str(),str2.as_str());
    println!("the longest = {}",result);
}

// &i32 -> a reference
// &'a i32 -> a reference with an expilicit lifetime 
// &'a mut i32 -> a mutable reference with an expilicit lifetime 
fn longest<'a>(x : &'a str, y : &'a str) -> &'a str {
    if x.len() >= y.len(){
        x
    }else {
        y
    }
    //here x ,y and return value will have the same lifetime -> a 
    //there is a relationship between x, y, and return value (the lifetime of the return value will be the same to the smallest lifetime of the input params here)

}
//THIS WAS CHECKED OVER WITH THE LIFETIME OF THE DATA WITH HELP OF BORROW CHECKER  
// let x = 5;
// let r= &x;
// println!("{}", r);

// let r;
// {
//     let x = 5;
//     r = &x; //dangling reference
// } // x has lifetime till this line
// println!("{}", r);

// fn largest(x : &str, y : &str) -> &str {
//     if x.len() >= y.len(){
//         x
//     }else {
//         y
//     }
    
// }
// we dont know the lifetime for the result from the above function , which is dependent over the lifetime of either x or y at the moment (that too we dont know, as they are just placeholders here)

// THUS WE GET THIS ERROR HERE ->
// missing lifetime specifier
// this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`

// for fixing this we use generic lifetime notation -> describes the notation of lifetimes between multiple reference and how they relate (DOESNT CHANGE THE LIFETIME, BUT JUST EXPLAINS THE LIFETIME FOR THAT RELATIONSHIP) as below -> 
     
// fn longest<'a>(x : &'a str, y : &'a str) -> &'a str {
//     if x.len() >= y.len(){
//         x
//     }else {
//         y
//     }
//     //here x ,y and return value will have the same lifetime -> a 
//     //there is a relationship between x, y, and return value (the lifetime of the return value will be the same to the smallest lifetime of the input params here)
// }

fn example2() {
    let str = String::from("hsm");
    {
        let str1 = String::from("xyz");
        let result = longest(str.as_str(), str1.as_str());
        println!("the longest : {}",result);
        //here the 2 string are having two different lifetime
        // result lifetime = str1 lifetime here
    }
}

//this will raise an error as the lifetime of result has ended
// fn example3() {
//     let str = String::from("hsm");
//     {
//         let str1 = String::from("xyz");
//         let result = longest(str.as_str(), str1.as_str());
//         //here the 2 string are having two different lifetime
//         // result lifetime = str1 lifetime here
//     }
//     println!("the longest : {}",result);
// }

fn longest2<'a>(x : &'a str, y : &str) -> &'a str {
    x
}

//giving out a ref from the function wont help either (out of scope ref)
// fn longest3<'a>(x : &str, y : &str) -> &'a str {
//     let result = String::from("long here");
//     result.as_str()
// }

fn longest3<'a>(x : &str, y : &str) -> String {
    let result = String::from("long here");
    result
}

//STRUCTS WITH LIFETIME ANOTATIONS

struct ImportantExample<'a> {
    part : &'a str,
}

fn struct_here() {
    let novel = String::from("some novel. data here...");
    let first_sentence = novel.split(".").next().expect("Coudl not find out the sentence");
    let i = ImportantExample {
        part : first_sentence,
    };
}

fn first_word<'a>(s : &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item==b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// RULES OF LIFETIME 
// 1. Each param that is a reference gets its own lieftime parameter 
// 2. If there is exactly one input lifetime param, theat is the lifetime assigned to all the output lifetime params
// 3. If there are multiple input lifetimes, but one of them is &self or &mut self the lifetime of self is assigned to all the output parameters  

// 3. -> 
impl <'a> ImportantExample<'a> {
    fn return_part(&self, announcement: &str)-> &str {
        println!("Attention here : {}", announcement);
        self.part
    }
}

// STATIC LIFETIME
fn static_here(){
    let s: &'static str = "I am HSM";
}



// ****************************** 

use std::fmt::Display;

fn longest_with_an_announcement<'a, T> (
    x : &'a str,
    y : &'a str,
    ann: T
) -> &'a str
where 
    T : Display,
{
    println!("Annoucement : {}", ann);
    if x.len() > y.len(){
        x
    }else{
        y
    }
}


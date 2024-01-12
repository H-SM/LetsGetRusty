use std::fmt::Display;

pub struct NewsArticle { 
    pub author : String,
    pub headline : String,
    pub content : String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }

    fn summarize_author(&self) -> String {
        format!("{}",self.author)
    }
}
pub struct Tweet { 
    pub username : String, 
    pub content : String, 
    pub reply : bool, 
    pub retweet : bool,    
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("{}",self.username)
    }
}

pub trait Summary {
    // fn summarize(&self) -> String;
    fn summarize(&self) -> String{
        // String::from("(read more ...)")
        //this inact as a default condition for the trait if this trait function is implemented to a struct anywhere...

        format!("(read more from {}...)",self.summarize_author())
    }

    fn summarize_author(&self) -> String ; 
}
fn main() {
    let tweet = Tweet {
        username : String::from("h-sm"), 
        content : String::from("Hello this is HSM"), 
        reply : false, 
        retweet : false, 
    };

    let article = NewsArticle { 
        author : String::from("HSM"),
        headline : String::from("This is a great tragedy!"),
        content : String::from("There is a earthquake, having a magnitude of 9.1 !!"),
    };

    println!("Tweet Summary {}", tweet.summarize());
    println!("Article Summary {}", article.summarize());
    notify(&article);

    println!("{}",returns_summary().summarize());
}

// pub fn notify(item: &impl Summary){
//     println!("NEWS -> {}",item.summarize());
// }
//the above has a trait bound syntax looking like -> 

pub fn notify<T: Summary>(item: &T){
    println!("NEWS -> {}",item.summarize());
}

// pub fn notify(item1: &impl Summary, item2: &impl Summary){
//     // .... 
// }


// pub fn notify<T: Summary>(item1: &T, item2: &T){
//     // .... 
// }


// pub fn notify(item1: &(impl Summary + Display), item2: &impl Summary){
//     // .... 
// }


// pub fn notify<T: Summary + Display>(item1: &T, item2: &T){
//     // .... 
// }

// having multiple trait bounds could hender readability as -> 
// pub fn notify<T: Display + Clone, U : Clone + Debug>(item1: &T, item2: &U) -> i32{
//     // .... 
// }

// we could use a where clause for the above as below -> 
// fn func<T, U >(item1: &T, item2: &U) -> i32 
//     where   T : Display + Clone,
//             U : Clone + Debug
// {
//     //...
// }

//this is a impl trait for Summary
// WE CAN ONLY RETURN ONE TYPE HERE (CANT BOOL OVER NEWARTICLE AND TWEET IN THE SAME FUNCTION)
fn returns_summary() -> impl Summary {
    Tweet {
        username : String::from("HSM2"),
        content : String::from("Hello this is HSM"), 
        reply : false, 
        retweet : false, 
    }
}

//conditionally implemented method 

struct Pair<T> {
    x : T,
    y : T,
}

impl <T> Pair<T> {
    //creates a new pair
    fn new(x: T,y: T) -> Self {
        Self {x, y}
    }
}

impl <T : Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}",self.x);
        }else{
            println!("The largest number is y = {}",self.y);
        }
    }
}

//blanket implementation
// we implement another trait over the trait given right here
// impl<T : Display> ToString for T {
//     // implement ToString trait to any type T implementing the Display trait
// }



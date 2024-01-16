use std::thread;
use std::time::Duration;

fn simulated_expensive_calc(intensity : u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
fn main() {
    // yay -Syu --aur 
    //CLOSURES - anonymous functions (can have variables and pass around input params to a function or catch a variable in the scope)
    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);
}

//to get in the expensive function and not call it again and again (ref the first 'if' condition)
struct Cacher<T> // generic 
where
    T: Fn(u32) -> u32, //trait bound
{
    calc : T, //the type of data
    value : Option<u32>, //initially the value will be null 
}

impl <T> Cacher<T>
where
    T: Fn(u32) -> u32, 
{
    fn new(calc : T) -> Cacher<T> {
        Cacher {
            calc,
            value : None,
        }
    }

    fn value(&mut self, arg : u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calc)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(simulated_intensity : u32, simulated_random_number : u32 ) {
    // let expensive_result = simulated_expensive_calc(simulated_intensity);
    // let expensive_closure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };
    // ^ doesn't need a input and output param like normal function
    // (just dont change the parameters from the closure in the scope)
    // eg -> 
    // let example_closure = |x| x;
    // let s = example_closure(String::from("hsm"));
    // let x = example_closure(5);
    
    let mut cached_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    
    if simulated_intensity < 25 {
        println!("Today, do {} pushups!",cached_result.value(simulated_intensity));
        println!("Today, do {} situps!",cached_result.value(simulated_intensity));
    }else{
        if simulated_random_number ==3 {
            println!("Take a break Today! Remember to be hydrated!");
        }else{
        println!("Today, run for {} minutes!",cached_result.value(simulated_intensity));
        }
    }
}

fn closure_here() {
    let x = 4;

    // fn equal_to_x(z : i32) -> bool {
    //     z == x
    // }

    let equal_to_x = |num| num == x;
    let y = 4;
    assert!(equal_to_x(y));

    let q = vec![1,2,3];

    let equal_to_q = move |z| z == q;
    //move -> take the ownership
    // println!("cant use x here : {:?}",q);

    let w = vec![1,2,3];

    assert!(equal_to_q(w));
}

// closure takes params in 3 ways -> 
// - by taking ownership [FnOnce]
// - by borrowing mutable params [FnMut]
// - by borrowing immutable params [Fn]
//TODO: look over these params once more
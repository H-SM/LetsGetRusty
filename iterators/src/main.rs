//ohw an iterator works -> 
pub trait Iterator {
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;

    //methods witt default implementation elided 
}


fn main() {
    //we can iterate over the elements regardless of how they are stored
    let v1 = vec![1,2,3];

    let v1_iter = v1.iter();

    for value in v1_iter {
        print!("{} ",value)
    }
    println!("");

    let v2 : Vec<_> = v1.iter().map(|x| x+1).collect();

    assert_eq!(v2, vec![2,3,4])
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size : u32, 
    style : String,
}

fn shoes_in_my_size(shoes : Vec<Shoe>, shoe_size : u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

//implementing iterators 

struct Counterhere {
    count: u32,
}

impl Counterhere {
    fn new() -> Counterhere {
        Counterhere { count: 0 }
    }
}

impl Iterator for Counterhere {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn iterator_demonstration() {
        let v1 = vec![1,2,3];
    
        let mut v1_iter = v1.into_iter();
        //muttable iterator -> iter_mut();
        //untypes iterator -> into_iter();
        assert_eq!(v1_iter.next(), Some(1));
        assert_eq!(v1_iter.next(), Some(2));
        assert_eq!(v1_iter.next(), Some(3));
        assert_eq!(v1_iter.next(), None);
    }
    
    #[test] 
    fn iterator_sum() {
        let v1 = vec![1,2,3];
    
        let v1_iter = v1.iter();
    
        let total : i32 = v1_iter.sum();
    
        assert_eq!(total, 6);
    }

    #[test] 
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size : 10,
                style : String::from("canvas"),
            },
            Shoe {
                size : 13,
                style : String::from("sandel"),
            },
            Shoe {
                size : 10,
                style : String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size, 
            vec![
                Shoe {
                    size : 10,
                    style : String::from("canvas"),
                }, 
                Shoe {
                    size : 10,
                    style : String::from("boot"),
                },
            ]
        )
    }
    
    #[test]
    fn calling_next_directly() {
        let mut counter = Counterhere::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counterhere::new()
        .zip(Counterhere::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
        //TODO: look over why this function isnt working
        assert_eq!(sum, 18);
    }
}
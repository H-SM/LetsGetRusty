fn main() {
    let number_list = vec![12,23,43,54,231];

    let largest = largest_here(number_list);

    println!("The largest number is : {}", largest);
}
// <Type: PartialOrd + Copy> Type -> generic type, (PartialOrd + Copy) -> traits for the type
fn largest_here<Type: PartialOrd + Copy>(number_list: Vec<Type>) -> Type {
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest { 
            largest = number;
        }
    }
    largest
}

struct Point <Type, U>{
    x : Type,
    y : U
}

fn structer() {
    let p1 = Point { x : 2, y : 5};
    let p2 = Point { x : 2.5, y : 5.6};
    let p3 = Point { x : 2, y : 5.6};
}

fn enums_here() { 
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}


struct Point_here <Type>{
    x : Type,
    y : Type
}

impl<Type> Point_here<Type> {
    fn x(&self) -> &Type {
        &self.x
    }
}

impl Point_here<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}

fn structer2() {
    let p1 = Point_here { x : 2, y : 5};
    p1.x();
    let p2 = Point_here { x : 5.0, y : 1.0};
    p2.y();
}

impl<Type, U> Point <Type, U> {
    fn mixup<V,W>(self, other : Point<V,W>) -> Point<Type,W> {
        Point { 
            x: self.x, // V -> Type change here 
            y: other.y, // W type gets passed 
        }  
    }
}

fn looker_here() {
    let p1 = Point { x : 5, y : 10.4};
    let p2 = Point { x : "Hello" , y : 'c'};
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}",p3.x , p3.y); // x : 5 , y : 'c'

}

// enum duplicates itself for different Type of Some params given to it.. eg -> one for i32 and one for f64 (if both are Some() values for the Option enum)


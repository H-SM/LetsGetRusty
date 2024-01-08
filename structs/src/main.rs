#[derive(Debug)]
struct User { 
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // here we will look over grouping of the data (structs)

    // ways to make an instance over the struct of data we have here
    let mut user1 = User { 
        email: String::from("hsmhere@gmail.com"),
        username: String::from("hsm"),
        active: true,
        sign_in_count: 1
    };

    let name = user1.username;
    user1.username = String::from("HSM");
    let user2 = builtuser(String::from("harmanhere@gmail.com"), String::from("harman"));

    //using the user2 params as default values to it
    let user3 = User { 
        email : String::from("hsm123@gmail.com"),
        username: String::from("harmanheretoo"),
        ..user2
    };

    // println!("{}",user3.username);
    // printing entire struct
    // println!("{:?}",user3);
    println!("{:#?}",user3);

    // now we also have TUPLE STRUCTS 
    struct color(i32, i32, i32);
    struct Point(i32, i32, i32);
    //THESE ARE STRUCTS W/O ANY FEILDS TO POINT DATA FROM 
    illfindthearea();
    // we could have also used strig slices but that requires slicing that needs lifetime over it (to do later)
}


fn builtuser(email : String, username : String) -> User {
    User { 
        // email: email,
        email,
        // username: username,
        username,
        active : true,
        sign_in_count : 1,
    }
}

//use case of tuple -> grouping the data that have a relation between them 
#[derive(Debug)]
struct Rectangle { 
    w : u32,
    h : u32,
} 

//suppose we have a function cloe to the struct above, we can add it in the struct only by "impl" (implementing the function over the structure)

impl Rectangle { 
    fn area(&self) -> u32 { 
        self.w * self.h
    }

    fn can_hold(&self, other: &Rectangle) -> bool { 
        self.w > other.w && self.h > other.h
    }
}

impl Rectangle { 
    // this is an associative function (doesn't have its own params in it)
    fn square(size : u32) -> Rectangle { 
        Rectangle { 
            w : size,
            h : size
        }
    }
}

fn illfindthearea() {
    // let w = 10;
    // let h = 50;

    // struct rect (u32, u32);
    // let rect = (10, 50); (will work w/o abv command)

    let rect = Rectangle { 
        w : 10,
        h : 50,
    };
    println!("The area for the rectangle is : {} px^2", rect.area());

    let rect1 = Rectangle { 
        w : 10,
        h : 10,
    };
    
    let rect2 = Rectangle { 
        w : 100,
        h : 100,
    };

    println!("{:#?}",rect1);
    println!("{:#?}",rect2);
    println!("");
    println!( "will rect1 be in rect2 : {}", rect2.can_hold(&rect1));
    println!( "will rect2 be in rect1 : {}", rect1.can_hold(&rect2));

    // using the associative function 
    let rect3 = Rectangle::square(5);
    println!("{:#?}",rect3);

}

// fn area(rectangle : &Rectangle) -> u32 { 
// fn area(dimensions : (u32,u32)) -> u32 { 
//     dimensions.0 * dimensions.1
// }

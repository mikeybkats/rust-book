#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// implementation block
// Everything within this impl block will be associated with the Rectangle type.
impl Rectangle {
    // within an implementation block self is an alias for the type that the impl block is for (in this case rectangle)
    // the self abbreviation can only exist in the first parameter spot (like python)
    fn area(&self) -> u32 {
        // remember use the reference (&someName) when you don't want to take ownership
        self.width * self.height
    }
    // example of a getter in rust:
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated functions don't use self as the first parameter
    // Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct.
    fn square(size: u32) -> Self {
        Self {
            // self is an alias for the name of the impl (rectangle in this case)
            // in other words this method returns an instance of the rectangle like a constructor
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // notice the syntax for calling the associated function below:
    let sq = Rectangle::square(3);
    println!("{:#?}", sq);
}

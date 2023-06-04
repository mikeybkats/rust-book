use std::io;

fn main() {
    // Mutables
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // Constants
    // const THREE_HOURSE_IN_SECONDS: u32 = 60 * 60 * 3;

    // Shadowing
    let x = 5;
    let x = x + 1;

    {
        let x = x *2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // using mutables correctly
    let spaces = "  ";
    let spaces = spaces.len(); // can redefine a string as a number with muiltiple let statements
    println!("spaces: {spaces}");

    // let mut spaces = "  ";
    // spaces = spaces.len(); // can't mutate a variables type

    // arrays
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

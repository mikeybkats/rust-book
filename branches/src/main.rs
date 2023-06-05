fn main() {
    let number = 3;

    // the condition must be a boolean
    // rust unlike other languages will not try to convert non boolean types to a boolean.
    // if statements require a boolean
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    // When this program executes, it checks each if expression in turn and executes the first body for which the condition evaluates to true
    let number = 6;

    // rust will only print the first block that's true
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    // note!! all related/arms of if statements in a block must result in the same data type. In other words, you can't do this:
    //     let number = if condition { 5 } else { "six" };

    // Using too many else if expressions can clutter your code, so if you have more than one consider refactoring with the match branching construct

    // Using if in a let statement (ternary operator):

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

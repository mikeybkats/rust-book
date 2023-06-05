fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeled_measurement(5, 'h');

    scoped_expression();

    let x = five();
    println!("The value of x is: {x}");

    let x_plus_one: i32 = plus_one(x);
    println!("The value x plus 1: {x_plus_one}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}.");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is : {value}{unit_label}");
}

// Statements and expressions:
/*
- Statements are instructions that perform some action and do not return a value.
- Expressions evaluate to a resultant value. Let’s look at some examples.
 */

fn scoped_expression() {
    // sometimes an expression can look like a statement if you don't know the syntax:
    // this expression is kind of like an immediately invoked function expression without the return statement
    let y = {
        let x = 3;
        x + 1
        // Note that the x + 1 line doesn’t have a semicolon at the end, which is unlike most of the lines you’ve seen so far. Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value
    };

    println!("The value of y is: {y}");
}

// Functions with return values:
fn five() -> i32 {
    5 // notice no semi-colon - meaning this is an expression we want to return (not a statment)
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // if a semi colon is added here an error like this will appear at runtime:
    /*
    $ cargo run
    Compiling functions v0.1.0 (file:///projects/functions)
         error[E0308]: mismatched types
         --> src/main.rs:7:24
         |
         7 | fn plus_one(x: i32) -> i32 {
         |    --------            ^^^ expected `i32`, found `()`
         |    |
         |    implicitly returns `()` as its body has no tail or `return` expression
         8 |     x + 1;
         |          - help: remove this semicolon to return this value

         For more information about this error, try `rustc --explain E0308`.
         error: could not compile `functions` due to previous error

      */
}

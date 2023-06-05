fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // break can be used to stop the loop
        }
    };
    println!("The result is {result}");

    loop_labels_to_disambiguate();
    conditional_loops_with_while();
    looping_through_collection_with_for();
}

fn loop_labels_to_disambiguate() {
    let mut count = 0;

    'counting_up: loop {
        println!("Count = {count}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn conditional_loops_with_while() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!");

    // same thing with a for loop:
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}

fn looping_through_collection_with_for() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        // SLOW: the compiler adds runtime code to perform the conditional check of whether the index is within the bounds of the array on every iteration through the loop
        // ERROR Prone: if the array changes or a different array is reassigned to a there could be a panic
        println!("The value is {}", a[index]);
        index += 1;
    }

    // the for loop solves the above problems
    // SAFER
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

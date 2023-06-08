fn main() {
    // String::from manages data allocated on the heap and stores the amount of text at compile time
    // the double colon operator :: namespaces the from function under the String type
    let mut s = String::from("hello");
    // this string type lives on the heap
    // this can be mutated

    s.push_str(", world!");
    // push_str appends a literal to a string
    // this literal lives on the stack
    // and cannot be mutated

    println!("{}", s);

    // in rust when a variable that's using memory goes out of scope the memory is freed up.
    {
        let s = String::from("hello");
        println!("{}", s);
    }
    // scope is over now. rust calls drop and frees the memory here

    // in the scope below s1 is only valid on line 2
    // if access to s1 is attempted (line 31) - println!("{}, world!", s1);
    // it will yeild the error:
    // error[E0382]: borrow of moved value: `s1`
    {
        let s1 = String::from("hello");
        // when s2 is defined as s1, rust "moves" the variable from s1 to s2.
        // it's a move because rust invalidates the first variable
        let s2 = s1;
        // rust invalidates the first variable to prevent memory problems "double free" error when the program attempts to free memory twice

        // println!("{}, world!", s1);
        println!("{}, world!", s2);
    }

    // if an actual duplication of a string is required (expensive! arbitrary), there is the clone method:
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {}, s2 = {}", s1, s2);
    }

    // stack-only data: copy
    {
        let x = 5;
        let y = x;

        // this code works because there's no reason to prevent x from being valid after creating y because the data is a known size and stored on the stack and is effectively a clone. It's not like a string where the copy is a copy of the pointer. This is a copy of the actual integer here.
        println!("x = {}, y = {}", x, y);
    }

    {
        let s = String::from("hello"); // s comes into scope
        takes_ownership(s); // s's value moves into the function scope
                            // s is no longer valid here

        let x = 5;
        makes_copy(x); // x would move into the makes_copy scope, but it lives on the stack so it doesn't

        println!("value of x after makes_copy() {}", x);
    }

    {
        let _s1 = gives_ownership(); // gives_ownership moves its return
                                     // value into s1
        let s2 = String::from("hello"); // s2 comes into scope

        let _s3 = takes_and_gives_back(s2); // s2 is moved into
                                            // takes_and_gives_back, which also
                                            // moves its return value into s3
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
      // happens. s1 goes out of scope and is dropped.
}

// ownership_and_functions
// The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does. Listing 4-3 has an example with some annotations showing where variables go into and out of scope.
fn takes_ownership(some_string: String) {
    println!("takes_ownership of: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("makes_copy of: {}", some_integer);
}

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn main() {
    let s1 = String::from("hello");

    // ampersand represents a reference to something
    // the pointer can live on the stack
    // s --> s1 --> value on heap
    let len = calculate_length(&s1);

    // passing by reference means that variable len is still in scope here
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s; // this won't work. only one mutable reference can exist
    /*
        No more than one mutable reference:
        The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these three behaviors occur:

        - Two or more pointers access the same data at the same time.
        - At least one of the pointers is being used to write to the data.
        - There’s no mechanism being used to synchronize access to the data.
    */
    // https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

    println!("{}", r1);
}

// ampersand = reference to a string
// the action of creating a reference is called "borrowing"
fn calculate_length(s: &String) -> usize {
    // s is a reference to a string
    s.len() // because it's a reference modifying it will not work
} // here, s goes out of scope. because it does not have ownership of what it refers to, it is not dropped.

fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

// references can be changed if they are marked as mutable references
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// dangling pointer—a pointer that references a location in memory that may have been given to someone else—by freeing some memory while preserving a pointer to that memory.
fn dangle() -> &String {
    // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

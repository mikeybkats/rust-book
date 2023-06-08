fn main() {
    let s = String::from("hello deary.");

    let s1 = first_word(&s);
    println!("first word ends at index: {}", s1);

    string_slice();

    let s2 = first_word_two(&s);
    println!("first word: {}", s2);
}

// this is an example of what not to do. it doesn't make a lot of sense to search for
// words in this manner.
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // converts the string reference into an array of bytes

    for (i, &item) in bytes
        .iter() // iterates over the array with the iter() method. it provides references to the items (&items)
        .enumerate()
    // enumerate returns an index (i)
    {
        // this is called byte literal syntax. it gives us the byte value of what's in the template literal
        // in this case the template literal has a blank space.
        if item == b' ' {
            return i;
        }
        // if it were me I would be building the word one
        // letter at a time with this iterator until I hit the space.
    }

    s.len()
}

fn string_slice() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("string slice: {} {}", hello, world);
}

// fn first_word_two(s: &String) -> &str {
fn first_word_two(s: &str) -> &str {
    //Defining a function to take a string slice instead of a reference to a String makes our API more general and useful without losing any functionality:

    // a type of &str will allow both &String and &str
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // use the string slice range to return the reference to the first word
        }
    }

    &s[..]
}

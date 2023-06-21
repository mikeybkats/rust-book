// use crate::averages::averages::median;
// use crate::averages::averages::mode;
// use crate::pig_latin::pig_latin::convert_word;
// use crate::pig_latin::pig_latin::convert_word_gpt;
// use std::time::Instant;
use crate::employees::employees::start_cli;
// tells root about the directory averages
pub mod averages;
pub mod employees;
pub mod pig_latin;

// vectors
// strings
// hash maps
fn main() {
    start_cli();
    // let numbers = vec![333, 22, 100, 200, 320, 16, 450];
    // median(numbers);

    // let numbers = vec![300, 299, 304];
    // median(numbers);

    // let numbers = vec![16, 1, 30, 450];
    // median(numbers);

    // let numbers = vec![10, 30, 40, 10, 20, 30, 10, 30, 40, 10];
    // let numbers_mode = mode(numbers);
    // println!("the mode number is: {numbers_mode}");

    // let numbers = vec![11, 31, 40, 10, 20, 31, 13, 30, 40, 10];
    // let numbers_mode = mode(numbers);
    // println!("the mode number is: {numbers_mode}");

    // let word = convert_word("boom");
    // println!("converted word: {word}");

    // let word = convert_word("oomff");
    // println!("converted word: {word}");

    // let word = convert_word("creamsicle");
    // println!("converted word: {word}");

    // let start_a = Instant::now();
    // let word_a = convert_word_gpt("popsicle");
    // let duration_a = start_a.elapsed();

    // let start_b = Instant::now();
    // let word_b = convert_word("popsicle");
    // let duration_b = start_b.elapsed();

    // if duration_a < duration_b {
    //     println!(
    //         "Method A is faster. A: {},  B: {}",
    //         duration_a.as_nanos(),
    //         duration_b.as_nanos()
    //     );
    // } else {
    //     println!("Method B is faster");
    // }

    // println!("converted words: {word_a} {word_b}");
}

pub fn vectors() {
    // creating a new vector
    // Vec<T> is provided by the standard library and can hold any type
    // let _v: Vec<i32> = Vec::new();

    // // the vec! macro will create a new vector and hold the values given:
    // let _v = vec![1, 2, 3];

    // // create a vector and then add elements
    // let mut v = Vec::new();

    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);

    // // reference to the element at index value 2
    // let third = &v[2];
    // println!("The third el: {}", third);

    // // .get returns an Option<&T>
    // let third = v.get(2);
    // // The reason Rust provides these two ways to reference an element is so you can choose how the program behaves when you try to use an index value outside the range of existing elements
    // match third {
    //     Some(third) => println!("The third el: {third}"),
    //     None => println!("There is no third el"),
    // }

    // let v = vec![1, 2, 3, 4, 5];

    // // let does_not_exist = &v[100];
    // // let does_not_exist = v.get(100);

    // // iterating through vectors
    // for i in &v {
    //     println!("{i}");
    // }

    // let mut v = vec![100, 200, 300];
    // for i in &mut v {
    //     *i += 50;
    // }
}

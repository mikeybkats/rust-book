// vectors
// strings
// hash maps
fn main() {
    // creating a new vector
    // Vec<T> is provided by the standard library and can hold any type
    let _v: Vec<i32> = Vec::new();

    // the vec! macro will create a new vector and hold the values given:
    let _v = vec![1, 2, 3];

    // create a vector and then add elements
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // reference to the element at index value 2
    let third = &v[2];
    println!("The third el: {}", third);

    // .get returns an Option<&T>
    let third = v.get(2);
    // The reason Rust provides these two ways to reference an element is so you can choose how the program behaves when you try to use an index value outside the range of existing elements
    match third {
        Some(third) => println!("The third el: {third}"),
        None => println!("There is no third el"),
    }

    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);

    // iterating through vectors
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 200, 300];
    for i in &mut v {
        *i += 50;
    }
}

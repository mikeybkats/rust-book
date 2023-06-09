struct Dimensions(u32, u32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rectangle_tuple = Dimensions(30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuples(rectangle_tuple)
    );

    let rectangle_struct = Rectangle {
        width: 30,
        height: 50,
    };
    // passing dimensions_struct as a reference allows the ownership to remain
    // in the scope of main
    println!(
        "The area of the rectangle is {} square pixels.",
        area_structs(&rectangle_struct)
    );

    // allows pretty print with derive debug
    println!("rectangle struct is {:#?}", rectangle_struct);
}

// fn area(width: u32, height: u32) -> u32 {
fn area_tuples(dimensions: Dimensions) -> u32 {
    // width * height
    dimensions.0 * dimensions.1
}

// remember making this parameter to the function a reference allows the ownership to stay
// in the scope of the caller
fn area_structs(dimensions: &Rectangle) -> u32 {
    dimensions.height * dimensions.width
}

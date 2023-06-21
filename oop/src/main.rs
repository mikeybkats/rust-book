use crate::my_class::my_class::MyClass;

pub mod my_class;

fn main() {
    println!("Hello, world!");

    let mut my_object = MyClass::new(42);
    my_object.do_something(); // Output: Doing something with data: 42

    my_object.modify_data(99);
    my_object.do_something(); // Output: Doing something with data: 99
}

pub mod my_class {
    pub struct MyClass {
        data: i32,
    }

    impl MyClass {
        pub fn new(data: i32) -> Self {
            MyClass { data }
        }

        // Method of MyClass
        pub fn do_something(&self) {
            println!("Doing something with data: {}", self.data);
        }

        // Method that modifies the data
        pub fn modify_data(&mut self, new_data: i32) {
            self.data = new_data;
        }
    }
}

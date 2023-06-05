use std::io;

fn main() {
    let farenheit = get_farenheit_input();
    let celsius = convert_to_celsius(farenheit);
    println!("Farenheit temperature: {farenheit} converted to celsius is: {celsius}");
}

fn get_farenheit_input() -> i32 {
    loop {
        println!("Enter a farenheit temperature to convert to celsius:");

        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        return match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}

fn convert_to_celsius(farenheit: i32) -> i32 {
    (farenheit - 32) * 5 / 9
}

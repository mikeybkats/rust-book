use std::io;
// use std::u32::MAX;

/*
returns the number from the fibonacci sequence at index number
*/
fn main() {
    loop {
        let number = get_user_input();
        let fibonacci_number = make_fibonacci(number);

        println!("The fibonacci number at index {number} is: {fibonacci_number}\n");

        println!("Would you like to enter another number? y/n");

        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("Falied to read line");

        match response.trim().to_lowercase().as_ref() {
            "yes" => continue,
            "y" => continue,
            "no" => break,
            "n" => break,
            _ => println!("Invalid response"),
        }
    }
}

fn get_user_input() -> u128 {
    loop {
        println!("Enter an index less than 151 from fibonacci and that value will be returned:");

        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        return match temp.trim().parse() {
            Ok(num) => {
                if num < 151 {
                    num
                } else {
                    continue;
                }
            }
            Err(_) => continue,
        };
    }
}

/*
generates nth number of a Fibonacci sequence

F0 = 0 (applies only to the first integer)
F1 = 1 (applies only to the second integer)
Fn = Fn-1 + Fn-2 (applies to all other integers)
*/
fn make_fibonacci(number: u128) -> u128 {
    let mut count = 2;
    let mut fn_1 = 0;
    let mut fn_2 = 1;

    loop {
        count += 1;

        if number == 0 {
            break 0;
        }

        if number == 1 {
            break 1;
        }

        if number == 2 {
            break 1;
        }

        let fib_n = fn_1 + fn_2;
        fn_1 = fn_2;
        fn_2 = fib_n;

        if count == number {
            break fib_n;
        }
    }
}

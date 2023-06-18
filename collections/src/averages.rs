pub mod averages {
    use std::collections::HashMap;

    pub fn median(numbers: Vec<i32>) -> i32 {
        let length = numbers.len();

        let mut sorted_numbers = numbers;
        sorted_numbers.sort();

        if length % 2 > 0 {
            // if length is odd return the length/2 index
            let median = sorted_numbers[length / 2];
            println!("Odd length. Median is: {median}");
            return median;
        } else {
            // if length is even return the length/2 - 1 index
            let median = sorted_numbers[length / 2 - 1];
            println!("Even length. Median is: {median}");
            return median;
        }
    }

    pub fn mode(numbers: Vec<i32>) -> i32 {
        let mut repeats = HashMap::new();

        for number in numbers {
            let count = repeats.entry(number).or_insert(0);
            *count += 1;
        }

        let mut mode = 0;
        let mut max = 0;

        for (key, value) in &repeats {
            if value > &max {
                mode = *key;
            }
            max = *value;
        }
        return mode;
    }
}

const GIFTS: [&str; 12] = [
    "a partridge in a pear tree",
    "two turtle doves",
    "three French hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "seven swans a-swimming",
    "eight mades a-milking",
    "nine ladies dancing",
    "ten lords a-leaping",
    "eleven pipers piping",
    "twelve drummers drumming",
];

fn main() {
    let mut count = 0;
    while count != 12 {
        println!("On the first day of");
        println!("Christmas, my true love");
        println!("sent to me");

        let mut inner_count = count;
        loop {
            if (inner_count == 0) && (count == 0) {
                println!("{}", GIFTS[inner_count]);
                break;
            }
            if inner_count == 0 {
                println!("And {}", GIFTS[inner_count]);
                break;
            }

            println!("{}", GIFTS[inner_count]);
            inner_count -= 1;
        }

        println!("\n");

        count += 1;
    }
}

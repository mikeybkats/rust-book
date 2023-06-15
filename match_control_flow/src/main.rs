#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
}
enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter,
    StateQuarter(UsState),
}

fn main() {
    #[derive(Debug)]
    struct Wallet {
        penny: u8,
        quarter: u8,
    }
    let my_worths = Wallet {
        penny: value_in_cents(Coin::Penny),
        quarter: value_in_cents(Coin::Quarter),
    };

    println!("the value of a penny is {}", my_worths.penny);
    println!("the value of a quarter is {}", my_worths.quarter);

    println!("{:#?}", Coin::Dime);
    println!("{:#?}", Coin::Nickel);
    println!("{:#?}", my_worths);

    let quarter = value_in_cents2(Coin2::StateQuarter(UsState::Arkansas));
    println!("my quarter: {}", quarter);
    let quarter = value_in_cents2(Coin2::Quarter);
    println!("my second quarter: {}", quarter);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?} {:?} {:?}", five, six, none);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
fn value_in_cents2(coin: Coin2) -> u8 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter => 25,
        Coin2::StateQuarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// You’ll see this pattern a lot in Rust code: match against an enum, bind a variable to the data inside, and then execute code based on it.
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// the catch all pattern "_"
fn catch_all_pattern() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // _ => reroll(),
        _ => (), // the empty tuple type will act doing nothing
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
// fn reroll() {}

use std::cmp::Ordering;

#[derive(Debug)]
enum USState {
    Alabama,
    Georgia,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(maybe_num: Option<i32>) -> Option<i32> {
    match maybe_num {
        None => None,
        Some(num) => Some(num + 1),
    }
}

fn relative_to_ten(num: i32) -> String {
    let output = match num.cmp(&10) {
        Ordering::Less => "less than",
        Ordering::Equal => "equal to",
        Ordering::Greater => "greater than",
    };

    output.to_string()
}

fn nums_name(num: i32) -> String {
    let output = match num {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "unknown",
    };

    output.to_string()
}

fn main() {
    println!("*** Coins ***");

    println!("Penny is {} cents", value_in_cents(Coin::Penny));
    println!("Nickel is {} cents", value_in_cents(Coin::Nickel));
    println!("Dime is {} cents", value_in_cents(Coin::Dime));
    println!(
        "Quarter is {} cents",
        value_in_cents(Coin::Quarter(USState::Alaska))
    );

    println!("*** Options ***");
    println!("Some(5) plus one is {:?}", plus_one(Some(5)));
    println!("None plus one is {:?}", plus_one(None));

    println!("Ints");
    println!("5 is {} 10", relative_to_ten(5));
    println!("10 is {} 10", relative_to_ten(10));
    println!("15 is {} 10", relative_to_ten(15));

    println!("Ints with wildcard");
    println!("1's name is {}", nums_name(1));
    println!("3's name is {}", nums_name(3));
    println!("10's name is {}", nums_name(10));
}

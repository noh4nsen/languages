use std::num;

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    NewYork
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter(UsState::NewYork);

    println!("Penny = {:?}", value_in_cents(penny));
    println!("Nickel = {:?}", value_in_cents(nickel));
    println!("Dime = {:?}", value_in_cents(dime));
    println!("Quarter = {:?}", value_in_cents(quarter));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?} {:?} {:?}", five, six, none);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() { println!("add fancy hat") }
    fn remove_fancy_hat() { println!("remove fancy hat") }
    fn move_player(num_spaces: u8) { println!("move {:?} spaces", num_spaces) }

    let dice_roll=10;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn reroll() { println!("reroll the dice") }

    let dice_roll = 30;
    match dice_roll {
        3 => add_fancy_hat(),
        8 => remove_fancy_hat(),
        _ => ()
    }

}

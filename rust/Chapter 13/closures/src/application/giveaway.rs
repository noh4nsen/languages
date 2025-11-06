use crate::domain::store::{Inventory, ShirtColor};

pub fn run(shirt_option: &Option<ShirtColor>, inventory: &Inventory) {
    let result = inventory.giveaway(*shirt_option);

    println!(
        "The user with preference {:?} gets {:?}",
        shirt_option, result
    );
}

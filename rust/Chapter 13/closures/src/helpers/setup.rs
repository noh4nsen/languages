use crate::domain::store::{Inventory, ShirtColor};

pub fn store() -> Inventory {
    Inventory {
        shirts: vec![
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Blue,
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Red,
            ShirtColor::Red,
        ],
    }
}

pub fn shirts() -> Vec<Option<ShirtColor>> {
    vec![
        Some(ShirtColor::Red),
        Some(ShirtColor::Blue),
        Some(ShirtColor::Red),
        None,
        None,
        Some(ShirtColor::Blue),
    ]
}

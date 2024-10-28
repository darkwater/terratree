#![allow(clippy::module_inception)]

pub mod item;

pub use self::item::Item;

// use self::item::enums::{DamageType, ItemType, Rarity};
// pub fn items() -> Vec<Item> {
//     include!("_items.rs")
// }

lazy_static::lazy_static! {
    pub static ref ITEMS: Vec<Item> = rmp_serde::from_slice(include_bytes!("items.bin")).unwrap();
}

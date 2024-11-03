#![allow(clippy::module_inception)]

pub mod image;
pub mod item;

pub use self::image::{Image, ImageLocation, ImageRef};
pub use self::item::Item;

// use self::item::enums::{DamageType, ItemType, Rarity};
// pub fn items() -> Vec<Item> {
//     include!("_items.rs")
// }

#[cfg(feature = "items")]
lazy_static::lazy_static! {
    pub static ref ITEMS: Vec<Item> = rmp_serde::from_slice(include_bytes!("items.bin")).unwrap();
}

#[cfg(feature = "images")]
lazy_static::lazy_static! {
    pub static ref IMAGES: Vec<ImageRef<'static>> = rmp_serde::from_slice(include_bytes!("images.bin")).unwrap();
}

use serde::Serialize;

use super::{
    enums::{DamageType, ItemType, Rarity},
    raw::RawItem,
    utils::{parse_leading_number, parse_opt_leading_number, parse_rarity},
};

#[derive(Debug, Serialize)]
pub struct Item {
    pub name: String,
    pub types: Vec<ItemType>,
    pub damage: Option<i32>,
    pub damage_type: Option<DamageType>,
    pub autoswing: Option<bool>,
    pub velocity: Option<u32>,
    pub knockback: Option<f32>,
    pub rarity: Rarity,
}

impl Item {
    pub fn from_raw(item: &RawItem) -> Option<Self> {
        Some(Self {
            name: item.name().to_owned(),
            types: item.r#type(),
            damage: parse_opt_leading_number(&item.damage()),
            damage_type: item
                .damagetype()
                .map(|s| s.parse().expect("invalid damage type")),
            autoswing: item.autoswing(),
            velocity: parse_opt_leading_number(&item.velocity()),
            knockback: item
                .knockback()
                .as_ref()
                .and_then(|s| parse_leading_number(s).ok()),
            rarity: parse_rarity(&item.rare())?,
        })
    }
}

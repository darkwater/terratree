use serde::{Deserialize, Serialize};

use super::{
    raw::RawItem,
    types::{DamageType, ItemType, Rarity},
    utils::{parse_leading_number, parse_opt_leading_number, parse_rarity},
    WikiText,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub item_id: i32,
    pub name: String,
    pub types: Vec<ItemType>,
    pub damage: Option<i32>,
    pub damage_type: Option<DamageType>,
    pub autoswing: Option<bool>,
    pub velocity: Option<u32>,
    pub knockback: Option<f32>,
    pub rarity: Rarity,
    pub tooltip: Option<WikiText>,
}

impl Item {
    pub fn from_raw(item: &RawItem) -> Option<Self> {
        Some(Self {
            item_id: item.itemid()?,
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
            tooltip: item.tooltip().map(|s| WikiText::new(&s)),
        })
    }
}

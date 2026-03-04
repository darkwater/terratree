use serde::{Deserialize, Serialize};

use crate::ImageLocation;

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
    pub image_location: Option<ImageLocation>,
    pub types: Vec<ItemType>,
    pub damage: Option<i32>,
    pub damage_type: Option<DamageType>,
    pub defense: Option<i32>,
    pub autoswing: Option<bool>,
    pub velocity: Option<u32>,
    pub knockback: Option<f32>,
    pub critical: Option<i32>,
    pub usetime: Option<i32>,
    pub mana: Option<i32>,
    pub hheal: Option<i32>,
    pub mheal: Option<i32>,
    pub buy: Option<String>,
    pub sell: Option<String>,
    pub stack: Option<i32>,
    pub consumable: Option<bool>,
    pub hardmode: Option<bool>,
    pub rarity: Rarity,
    pub tooltip: Option<WikiText>,
}

impl Item {
    pub fn from_raw(item: &RawItem, image_location: Option<ImageLocation>) -> Option<Self> {
        Some(Self {
            item_id: item.itemid()?,
            name: item.name().to_owned(),
            image_location,
            types: item.r#type(),
            damage: parse_opt_leading_number(&item.damage()),
            damage_type: item
                .damagetype()
                .and_then(|s| s.parse().ok()),
            defense: parse_opt_leading_number(&item.defense()),
            autoswing: item.autoswing(),
            velocity: parse_opt_leading_number(&item.velocity()),
            knockback: item
                .knockback()
                .as_ref()
                .and_then(|s| parse_leading_number(s).ok()),
            critical: parse_opt_leading_number(&item.critical()),
            usetime: parse_opt_leading_number(&item.usetime()),
            mana: parse_opt_leading_number(&item.mana()),
            hheal: parse_opt_leading_number(&item.hheal()),
            mheal: parse_opt_leading_number(&item.mheal()),
            buy: item.buy(),
            sell: item.sell(),
            stack: parse_opt_leading_number(&item.stack()),
            consumable: item.consumable(),
            hardmode: item.hardmode(),
            rarity: parse_rarity(&item.rare())?,
            tooltip: item.tooltip().map(|s| WikiText::new(&s)),
        })
    }
}

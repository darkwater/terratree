use std::str::FromStr;

use super::{
    utils::{parse_leading_number, parse_opt_leading_number, parse_rarity},
    ItemType, Rarity, RawItem,
};

#[derive(Debug)]
pub struct Weapon {
    pub name: String,
    pub damage: Option<i32>,
    pub damage_type: Option<DamageType>,
    pub autoswing: bool,
    pub velocity: Option<u32>,
    pub knockback: Option<f32>,
    pub rarity: Rarity,
}

#[derive(Debug)]
pub enum DamageType {
    Melee,
    Ranged,
    Summon,
    Magic,
    Other,
}

impl FromStr for DamageType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "melee" => Ok(Self::Melee),
            "ranged" => Ok(Self::Ranged),
            "summon" => Ok(Self::Summon),
            "magic" => Ok(Self::Magic),
            "-" => Ok(Self::Other),
            _ => Err(()),
        }
    }
}

impl Weapon {
    pub fn from_raw(item: &RawItem) -> Option<Self> {
        if !item.types().contains(&ItemType::Weapon) {
            return None;
        }

        Some(Self {
            name: item.name.clone(),
            damage: parse_opt_leading_number(&item.damage),
            damage_type: item.damagetype.clone().map(|s| s.parse().unwrap()),
            autoswing: match item.autoswing.as_deref() {
                Some("1") => true,
                Some("0") | None => false,
                other => panic!("Unexpected autoswing value: {:?}", other),
            },
            velocity: parse_opt_leading_number(&item.velocity),
            knockback: item
                .knockback
                .as_ref()
                .and_then(|s| parse_leading_number(s).ok()),
            rarity: parse_rarity(&item.rare),
        })
    }
}

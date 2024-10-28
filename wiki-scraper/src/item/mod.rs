use std::str::FromStr;

use derive_more::derive::TryFrom;
use serde::{Deserialize, Serialize};

use self::weapon::Weapon;

mod utils;
pub mod weapon;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RawItem {
    pub itemid: Option<String>,
    pub name: String,
    pub internalname: Option<String>,
    pub image: Option<String>,
    pub imagefile: Option<String>,
    pub imageplaced: Option<String>,
    pub imageequipped: Option<String>,
    pub autoswing: Option<String>,
    pub stack: Option<String>,
    pub consumable: Option<String>,
    pub hardmode: Option<String>,
    pub r#type: Option<String>,
    pub listcat: Option<String>,
    pub tag: Option<String>,
    pub damage: Option<String>,
    pub damagetype: Option<String>,
    pub defense: Option<String>,
    pub velocity: Option<String>,
    pub knockback: Option<String>,
    pub research: Option<String>,
    pub rare: Option<String>,
    pub buy: Option<String>,
    pub sell: Option<String>,
    pub axe: Option<String>,
    pub pick: Option<String>,
    pub hammer: Option<String>,
    pub fishing: Option<String>,
    pub bait: Option<String>,
    pub bonus: Option<String>,
    pub toolspeed: Option<String>,
    pub usetime: Option<String>,
    pub unobtainable: Option<String>,
    pub critical: Option<String>,
    pub tooltip: Option<String>,
    pub placeable: Option<String>,
    pub placedwidth: Option<String>,
    pub placedheight: Option<String>,
    pub mana: Option<String>,
    pub hheal: Option<String>,
    pub mheal: Option<String>,
    pub bodyslot: Option<String>,
    pub buffs: Option<String>,
    pub debuffs: Option<String>,
}

impl RawItem {
    pub fn fields() -> Vec<String> {
        serde_json::to_value(RawItem::default())
            .unwrap()
            .as_object()
            .unwrap()
            .keys()
            .map(|k| k.to_string())
            .collect()
    }

    pub fn types(&self) -> Vec<ItemType> {
        self.r#type
            .as_deref()
            .unwrap_or_default()
            .split('^')
            .filter_map(|t| t.parse().ok())
            .collect()
    }

    pub fn parse(&self) -> Option<Item> {
        Weapon::from_raw(self).map(Item::Weapon)
    }
}

#[derive(Debug, PartialEq)]
pub enum ItemType {
    Weapon,
}

impl FromStr for ItemType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "weapon" => Ok(ItemType::Weapon),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum Item {
    Weapon(Weapon),
}

#[derive(Debug, TryFrom)]
#[try_from(repr)]
#[repr(i32)]
pub enum Rarity {
    Gray = -1,
    White = 0,
    Blue = 1,
    Green = 2,
    Orange = 3,
    LightRed = 4,
    Pink = 5,
    LightPurple = 6,
    Lime = 7,
    Yellow = 8,
    Cyan = 9,
    Red = 10,
    Purple = 11,
    Expert = -12,
    Master = -13,
    Quest = -11,
}

impl Rarity {
    pub fn color(&self) -> u32 {
        match self {
            Rarity::Gray => 0x828282,
            Rarity::White => 0xffffff,
            Rarity::Blue => 0x9696ff,
            Rarity::Green => 0x96ff96,
            Rarity::Orange => 0xffc896,
            Rarity::LightRed => 0xff9696,
            Rarity::Pink => 0xff96ff,
            Rarity::LightPurple => 0xd2a0ff,
            Rarity::Lime => 0x96ff0a,
            Rarity::Yellow => 0xffff0a,
            Rarity::Cyan => 0x05c8ff,
            Rarity::Red => 0xff2864,
            Rarity::Purple => 0xb428ff,
            Rarity::Expert => 0xffaf00,
            Rarity::Master => 0xff0000,
            Rarity::Quest => 0xffaf00,
        }
    }
}

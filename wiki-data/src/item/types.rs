use std::str::FromStr;

use derive_more::derive::TryFrom;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ItemType {
    Weapon,
    Tool,
}

impl FromStr for ItemType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "weapon" => Ok(ItemType::Weapon),
            "tool" => Ok(ItemType::Tool),
            _ => Err(()),
        }
    }
}

#[derive(Debug, TryFrom, Serialize, Deserialize)]
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

impl FromStr for Rarity {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "quest" => Ok(Rarity::Quest),
            other => {
                let rarity = other.parse::<i32>().map_err(|_| ())?;
                Rarity::try_from(rarity).map_err(|_| ())
            }
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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

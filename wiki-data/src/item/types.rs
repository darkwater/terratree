use core::fmt::{Display, Formatter};
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

impl Display for ItemType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, TryFrom, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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
    pub fn color(&self) -> RarityColor {
        match self {
            Rarity::Gray => RarityColor::Static { r: 0x82, g: 0x82, b: 0x82 },
            Rarity::White => RarityColor::Static { r: 0xff, g: 0xff, b: 0xff },
            Rarity::Blue => RarityColor::Static { r: 0x96, g: 0x96, b: 0xff },
            Rarity::Green => RarityColor::Static { r: 0x96, g: 0xff, b: 0x96 },
            Rarity::Orange => RarityColor::Static { r: 0xff, g: 0xc8, b: 0x96 },
            Rarity::LightRed => RarityColor::Static { r: 0xff, g: 0x96, b: 0x96 },
            Rarity::Pink => RarityColor::Static { r: 0xff, g: 0x96, b: 0xff },
            Rarity::LightPurple => RarityColor::Static { r: 0xd2, g: 0xa0, b: 0xff },
            Rarity::Lime => RarityColor::Static { r: 0x96, g: 0xff, b: 0x0a },
            Rarity::Yellow => RarityColor::Static { r: 0xff, g: 0xff, b: 0x0a },
            Rarity::Cyan => RarityColor::Static { r: 0x05, g: 0xc8, b: 0xff },
            Rarity::Red => RarityColor::Static { r: 0xff, g: 0x28, b: 0x64 },
            Rarity::Purple => RarityColor::Static { r: 0xb4, g: 0x28, b: 0xff },
            Rarity::Expert => RarityColor::Expert,
            Rarity::Master => RarityColor::Master,
            Rarity::Quest => RarityColor::Static { r: 0xff, g: 0xaf, b: 0x00 },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RarityColor {
    Static { r: u8, g: u8, b: u8 },
    Expert,
    Master,
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "melee" => Ok(Self::Melee),
            "ranged" => Ok(Self::Ranged),
            "summon" => Ok(Self::Summon),
            "magic" => Ok(Self::Magic),
            "" => Ok(Self::Other),
            other => Err(other.to_owned()),
        }
    }
}

impl Display for DamageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

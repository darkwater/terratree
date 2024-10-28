use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::{enums::ItemType, item::Item};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RawItem {
    itemid: Option<String>,        // Integer
    name: String,                  // String
    internalname: String,          // String
    image: Option<String>,         // Wikitext
    imagefile: Option<String>,     // String
    imageplaced: Option<String>,   // Wikitext
    imageequipped: Option<String>, // Wikitext
    autoswing: Option<String>,     // Boolean
    stack: Option<String>,         // Wikitext
    consumable: Option<String>,    // Boolean
    hardmode: Option<String>,      // Boolean
    r#type: Option<String>,        // List of String, delimiter: ^
    listcat: Option<String>,       // List of String, delimiter: ^
    tag: Option<String>,           // List of String, delimiter: ^
    damage: Option<String>,        // Wikitext
    damagetype: Option<String>,    // String
    defense: Option<String>,       // Wikitext
    velocity: Option<String>,      // Wikitext
    knockback: Option<String>,     // Wikitext
    research: Option<String>,      // Wikitext
    rare: Option<String>,          // Wikitext
    buy: Option<String>,           // Wikitext
    sell: Option<String>,          // Wikitext
    axe: Option<String>,           // Wikitext
    pick: Option<String>,          // Wikitext
    hammer: Option<String>,        // Wikitext
    fishing: Option<String>,       // Wikitext
    bait: Option<String>,          // Integer
    bonus: Option<String>,         // Wikitext
    toolspeed: Option<String>,     // Wikitext
    usetime: Option<String>,       // Wikitext
    unobtainable: Option<String>,  // Boolean
    critical: Option<String>,      // Wikitext
    tooltip: Option<String>,       // Wikitext
    placeable: Option<String>,     // Boolean
    placedwidth: Option<String>,   // Integer
    placedheight: Option<String>,  // Integer
    mana: Option<String>,          // Wikitext
    hheal: Option<String>,         // Wikitext
    mheal: Option<String>,         // Wikitext
    bodyslot: Option<String>,      // String
    buffs: Option<String>,         // List of String, delimiter: ^
    debuffs: Option<String>,       // List of String, delimiter: ^
}

macro_rules! getters {
    ($helper:ident ($wrapper:ident): $($field:ident: $ty:ty,)*) => {
        $(
            pub fn $field(&self) -> $wrapper<$ty> {
                Self::$helper(&self.$field)
            }
        )*
    };
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

    pub fn parse(&self) -> Option<Item> {
        Item::from_raw(self)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn internalname(&self) -> &str {
        &self.internalname
    }

    fn list<T: FromStr>(field: &Option<String>) -> Vec<T> {
        field
            .as_deref()
            .map(|s| s.split('^').filter_map(|s| s.parse().ok()).collect())
            .unwrap_or_default()
    }

    getters! {
        list (Vec):
            r#type: ItemType, listcat: String, tag: String, buffs: String,
            debuffs: String,
    }

    fn integer<N: FromStr>(field: &Option<String>) -> Option<N> {
        field.as_deref().and_then(|s| s.parse().ok())
    }

    getters! {
        integer (Option):
            itemid: i32, bait: i32, placedwidth: i32, placedheight: i32,
    }

    fn boolean(field: &Option<String>) -> Option<bool> {
        field.as_deref().map(|s| s == "1")
    }

    getters! {
        boolean (Option):
            autoswing: bool, consumable: bool, hardmode: bool, unobtainable: bool,
            placeable: bool,
    }

    fn wikitext(field: &Option<String>) -> Option<String> {
        field.clone()
    }

    getters! {
        wikitext (Option):
            image: String, imageplaced: String, imageequipped: String,
            stack: String, damage: String, defense: String, velocity: String,
            knockback: String, research: String, rare: String, buy: String,
            sell: String, axe: String, pick: String, hammer: String,
            fishing: String, bonus: String, toolspeed: String, usetime: String,
            critical: String, tooltip: String, mana: String, hheal: String,
            mheal: String,
    }

    fn string(field: &Option<String>) -> Option<String> {
        field.clone()
    }

    getters! {
        string (Option):
            damagetype: String, bodyslot: String,
    }
}

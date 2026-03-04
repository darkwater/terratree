use serde::{Deserialize, Deserializer, Serializer};
use wiki_data::{ITEMS, Item};

pub fn serialize<S>(nodes: &Item, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&nodes.name)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<&'static Item, D::Error>
where
    D: Deserializer<'de>,
{
    let name = String::deserialize(deserializer)?;

    ITEMS
        .iter()
        .find(|item| item.name == name)
        .ok_or_else(|| serde::de::Error::custom(format!("Item not found: {name}")))
}

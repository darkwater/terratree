use std::sync::Arc;

use serde::Deserialize;

pub mod tree;

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Data {
    pub milestones: Vec<Arc<Milestone>>,
    pub items: Vec<Arc<Item>>,
}

#[derive(Debug, Deserialize)]
pub struct Milestone {
    pub name: String,
    #[serde(skip)]
    pub items: Vec<Arc<Item>>,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    pub name: String,
    pub tags: Vec<String>,
    pub milestone: String,
}

impl Data {
    pub fn load() -> Result<Self, toml::de::Error> {
        [
            include_str!("../../assets/tdata/milestones.toml"),
            include_str!("../../assets/tdata/items.toml"),
        ]
        .iter()
        .map(|s| toml::from_str(s))
        .collect::<Result<Data, _>>()
    }
}

impl FromIterator<Data> for Data {
    fn from_iter<I: IntoIterator<Item = Data>>(iter: I) -> Self {
        let mut data = Data::default();
        for item in iter {
            data.milestones.extend(item.milestones);
            data.items.extend(item.items);
        }
        data
    }
}

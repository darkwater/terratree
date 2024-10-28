use std::sync::Arc;

use super::Item;

pub struct TDataTree {
    pub milestones: Vec<TreeMilestone>,
}

pub struct TreeMilestone {
    pub name: String,
    pub items: Vec<TreeItem>,
}

pub type TreeItem = Arc<Item>;

impl TDataTree {
    pub fn from_tdata(tdata: &super::Data) -> Self {
        let mut milestones = tdata
            .milestones
            .iter()
            .map(|m| TreeMilestone { name: m.name.clone(), items: vec![] })
            .collect::<Vec<_>>();

        for item in &tdata.items {
            let milestone = milestones
                .iter_mut()
                .find(|m| m.name == item.milestone)
                .unwrap();

            milestone.items.push(item.clone());
        }

        Self { milestones }
    }
}

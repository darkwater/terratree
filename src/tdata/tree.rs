use std::sync::Arc;

pub struct TDataTree {
    pub milestones: Vec<TreeMilestone>,
}

pub struct TreeMilestone {
    pub name: String,
    pub items: Vec<TreeItem>,
}

pub struct TreeItem {
    pub tdata: Arc<super::Item>,
    pub wiki_data: &'static wiki_data::Item,
}

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

            let wiki_data = wiki_data::ITEMS
                .iter()
                .find(|i| i.name == item.name)
                .expect("missing wiki data");

            milestone
                .items
                .push(TreeItem { tdata: item.clone(), wiki_data });
        }

        Self { milestones }
    }
}

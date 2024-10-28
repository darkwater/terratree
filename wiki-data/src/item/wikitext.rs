use std::ops::Deref;

use scraper::{node::Node, Html};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiText {
    pub text: String,
}

impl WikiText {
    pub fn new(text: &str) -> Self {
        Self {
            text: Html::parse_fragment(&unescape(text))
                .root_element()
                .descendants()
                .filter_map(|node| match node.value() {
                    Node::Text(text) => Some(text.deref()),
                    Node::Element(el) if el.name() == "br" => Some("\n"),
                    _ => None,
                })
                .collect::<String>(),
        }
    }
}

fn unescape(text: &str) -> String {
    text.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
}

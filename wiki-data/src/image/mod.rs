use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub name: String,
    pub data: Vec<u8>,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageRef<'a> {
    pub name: &'a str,
    pub data: &'a [u8],
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageLocation {
    pub name: String,
    pub url: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WikiImageInfoPage {
    pub title: String,
    pub pageid: Option<i32>,
    pub ns: i32,
    pub imagerepository: Option<String>,
    #[serde(default)]
    pub imageinfo: Vec<WikiImageInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WikiImageInfo {
    pub url: String,
    pub descriptionurl: String,
    pub descriptionshorturl: String,
    pub width: i32,
    pub height: i32,
    pub size: i32,
}

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Response {
    pub data: Vec<Module>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Module {
    pub id: usize,
    pub manufacturer: String,
    pub name: String,
    pub rack_rash: bool,
    pub first_owner: bool,
    pub box_included: bool,
    pub price: usize,
    pub condition: String,
    pub modulargrid: Option<String>,
    pub pictures: Vec<Picture>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Picture {
    pub directus_files_id: String,
}

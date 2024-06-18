use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct TxtPath {
    textures: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ItemTxt {
    resource_pack_name: String,
    texture_name: String,
    texture_data: HashMap<String, TxtPath>,
}
impl ItemTxt {
    pub fn from_string(str: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&str)
    }
    pub fn new(resource: String, texture: String) -> Self {
        Self {
            resource_pack_name: resource,
            texture_name: texture,
            texture_data: HashMap::new(),
        }
    }
    pub fn concat(&mut self, other: ItemTxt) {
        for (name, txt) in other.texture_data.iter() {
            self.texture_data.insert(name.to_string(), txt.clone());
        }
    }
    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }
}

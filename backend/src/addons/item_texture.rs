use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Texture {
    String(String),
    Vec(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextureData {
    textures: Texture,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ItemTxt {
    resource_pack_name: Option<String>,
    texture_name: Option<String>,
    texture_data: HashMap<String, TextureData>,
}
impl ItemTxt {
    pub fn from_text(text: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(text)
    }
    pub fn new(name: &str) -> Self {
        Self {
            texture_name: Some("atlas.items".to_string()),
            resource_pack_name: Some(name.to_string()),
            texture_data: HashMap::new(),
        }
    }
    pub fn set(&mut self, key: String, data: TextureData) -> Option<TextureData> {
        self.texture_data.insert(key, data)
    }
    pub fn concat(&mut self, other: Self) {
        for (key, data) in other.texture_data {
            self.set(key, data);
        }
    }
    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
pub struct BlockTxt {
    resource_pack_name: Option<String>,
    texture_name: Option<String>,
    padding: u8,
    num_mip_levels: u8,
    texture_data: HashMap<String, TextureData>,
}
impl BlockTxt {
    pub fn from_text(text: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(text)
    }
    pub fn new(name: &str) -> Self {
        Self {
            texture_name: Some("atlas.items".to_string()),
            resource_pack_name: Some(name.to_string()),
            num_mip_levels: 4,
            padding: 8,
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

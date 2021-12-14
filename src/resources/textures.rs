use std::collections::HashMap;

use bevy::asset::{AssetServer, HandleUntyped};
use bevy::sprite::TextureAtlas;

use crate::generation::Soil;

#[derive(Default, Clone)]
pub struct Textures {
    textures: HashMap<&'static str, usize>,
    pub handles: Vec<HandleUntyped>,
    pub loaded: bool,
}

fn index(server: &AssetServer, atlas: &TextureAtlas, name: &str) -> usize {
    let path = format!("textures/{}.png", name);
    atlas
        .get_texture_index(&server.get_handle(path.as_str()))
        .expect(format!("Texture doesn't exist: {}", path).as_str())
}

impl Textures {
    pub fn load(&mut self, server: &AssetServer, atlas: &TextureAtlas) {
        let labels = vec![
            "water",
            "water_deep",
            "water_shallow",
            "grass1",
            "grass2",
            "grass3",
            "grass4",
            "clay",
            "sand",
            "silt",
            "peat",
            "chalk",
            "loam",
            "blank",
            "trees",
            "marker",
            "snow",
            "unit",
            "unit_lifted",
        ];

        self.textures = labels
            .iter()
            .map(|&l| (l, index(server, atlas, l)))
            .collect();
    }

    pub fn get(&self, label: &str) -> usize {
        self.textures[label]
    }

    pub fn soil(&self, soil: &Soil) -> usize {
        match soil {
            Soil::Clay => self.get("clay"),
            Soil::Sand => self.get("sand"),
            Soil::Silt => self.get("silt"),
            Soil::Peat => self.get("peat"),
            Soil::Chalk => self.get("chalk"),
            Soil::Loam => self.get("loam"),
            Soil::None => self.get("blank"),
        }
    }
}

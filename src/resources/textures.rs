use std::collections::HashMap;

use bevy::asset::{AssetServer, HandleUntyped};
use bevy::sprite::TextureAtlas;

use crate::generation::Soil;

pub enum Label {
    ShallowWater,
    DeepWater,
    Snow,
    Grass1,
    Grass2,
    Grass3,
    Grass4,
    Trees,
    Clay,
    Sand,
    Silt,
    Peat,
    Chalk,
    Loam,
    Blank,
    Unit,
    Marker,
}

impl Label {
    /// keep the mapping in same location as 
    /// the load function so that when label names
    /// change they can be updated.
    pub fn as_str(&self) -> &'static str {
        use Label::*;
        match self {
            ShallowWater => "water_shallow",
            DeepWater => "water_deep",
            Snow => "snow",
            Grass1 => "grass1",
            Grass2 => "grass2",
            Grass3 => "grass3",
            Grass4 => "grass4",
            Trees => "trees",
            Clay => "clay",
            Sand => "sand",
            Silt => "silt",
            Peat => "peat",
            Chalk => "chalk",
            Loam => "loam",
            Blank => "blank",
            Unit => "units/infantry",
            Marker => "marker",
        }
    }
}

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
            "units/veteran/veteran_0",
            "units/veteran/veteran_1",
            "units/veteran/veteran_2",
            "units/veteran/veteran_3",
            "units/armor",
            "units/armor/armor_blue",
            "units/armor/armor_green",
            "units/armor/armor_red",
            "units/armor/armor_yellow",
            "units/infantry",
            "units/infantry/infantry_blue",
            "units/infantry/infantry_green",
            "units/infantry/infantry_red",
            "units/infantry/infantry_yellow",
            "units/militia",
            "units/militia/militia_blue",
            "units/militia/militia_green",
            "units/militia/militia_red",
            "units/militia/militia_yellow",
        ];

        self.textures = labels
            .iter()
            .map(|&l| (l, index(server, atlas, l)))
            .collect();
    }

    pub fn get(&self, texture: Label) -> usize {
        self.textures[texture.as_str()]
    }

    pub fn soil(&self, soil: &Soil) -> usize {
        match soil {
            Soil::Clay => self.get(Label::Clay),
            Soil::Sand => self.get(Label::Sand),
            Soil::Silt => self.get(Label::Silt),
            Soil::Peat => self.get(Label::Peat),
            Soil::Chalk => self.get(Label::Chalk),
            Soil::Loam => self.get(Label::Loam),
            Soil::None => self.get(Label::Blank),
        }
    }
}

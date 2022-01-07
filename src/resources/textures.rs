use std::collections::HashMap;

use bevy::asset::{AssetServer, HandleUntyped};
use bevy::sprite::TextureAtlas;

use crate::generation::Soil;
use crate::generation::Specialty;

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
    Marker,
    Infantry,
    Armor,
    Militia,
    Medical,
    Logistics,
    Mechanic,
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
            Marker => "marker",
            Infantry => "units/infantry/infantry",
            Armor => "units/armor/armor",
            Militia => "units/militia/militia",
            Medical => "units/medical/medical",
            Logistics => "units/logistics/logistics",
            Mechanic => "units/mechanic/mechanic",
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
            "units/armor/armor_0",
            "units/armor/armor_1",
            "units/armor/armor_2",
            "units/armor/armor_3",
            "units/armor/armor_4",
            "units/infantry/infantry_0",
            "units/infantry/infantry_1",
            "units/infantry/infantry_2",
            "units/infantry/infantry_3",
            "units/infantry/infantry_4",
            "units/militia/militia_0",
            "units/militia/militia_1",
            "units/militia/militia_2",
            "units/militia/militia_3",
            "units/militia/militia_4",
        ];

        self.textures = labels
            .iter()
            .map(|&l| (l, index(server, atlas, l)))
            .collect();
    }

    pub fn get(&self, texture: Label) -> usize {
        self.textures[texture.as_str()]
    }

    pub fn variant(&self, texture: Label, v: u8) -> usize {
        let key = format!("{}_{}",texture.as_str(),v);
        self.textures[key.as_str()]
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

    pub fn unit(&self, unit: &Specialty, v: u8) -> usize {
        match unit {
            Specialty::Infantry => self.variant(Label::Infantry,v),
            Specialty::Armor => self.variant(Label::Armor,v),
            Specialty::Militia => self.variant(Label::Militia,v),
            Specialty::Medical => self.variant(Label::Medical,v),
            Specialty::Logistics => self.variant(Label::Logistics,v),    
            Specialty::Mechanic => self.variant(Label::Mechanic,v),
        }
    }
}

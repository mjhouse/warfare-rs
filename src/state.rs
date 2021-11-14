use std::collections::hash_map::HashMap;
use bevy::asset::HandleUntyped;

use crate::area::{Location,Area,Attribute,bounds};
use crate::spectrum::Spectrum;
use crate::generator::Factors;

#[derive(Default, Clone)]
pub struct Terrain {
    pub selected: Area,
    pub overlay: Attribute,
    pub seed: String,
    pub size: String,
    pub update: bool,
}

#[derive(Default, Clone)]
pub struct Icons {
    pub blank: usize,
    pub mark: usize,
}

#[derive(Default, Clone)]
pub struct Resources {
    pub textures: Vec<HandleUntyped>,
    pub fonts: Vec<HandleUntyped>,
    pub loaded_textures: bool,
    pub loaded_fonts: bool,
}

#[derive(Default, Clone)]
pub struct State {
    pub resources: Resources,
    pub areas: HashMap<Location,Area>,
    pub overlay: HashMap<Attribute,Spectrum>,
    pub terrain: Terrain,
    pub factors: Factors,
    pub icons: Icons,
    pub loaded: bool,
}

impl State {
    pub fn add(&mut self, area: Area) {
        self.areas.insert(area.location(),area);
    }

    pub fn add_all(&mut self, areas: Vec<Area>) {
        for area in areas.into_iter() {
            self.add(area);
        }
    }

    pub fn get_texture(&self, loc: &Location) -> usize {
        match self.areas.get(loc) {
            Some(a) => a.texture(),
            None => self.icons.blank,
        }
    }

    pub fn get_attribute(&self, loc: &Location, attr: &Attribute) -> f32 {
        match self.areas.get(loc) {
            Some(a) => match attr {
                Attribute::Biome => self.biome_scaled(a),
                Attribute::Soil => self.soil_scaled(a),
                Attribute::Elevation => self.elevation_scaled(a),
                Attribute::Temperature => self.temperature_scaled(a),
                Attribute::Fertility => self.fertility_scaled(a),
                Attribute::Rocks => self.rocks_scaled(a),
                Attribute::Moisture => self.moisture_scaled(a),
                Attribute::None => 0.0,
            },
            None => 0.0
        }
    }

    fn biome_scaled(&self, area: &Area) -> f32 {
        let s = area.biome() as u8;
        s as f32 / 5.0
    }

    fn soil_scaled(&self, area: &Area) -> f32 {
        let s = area.soil() as u8;
        s as f32 / 5.0
    }

    fn elevation_scaled(&self, area: &Area) -> f32 {
        use bounds::*;
        let m = MIN_ELEV.abs();
        let s = area.elevation();
        (s + m) / (MAX_ELEV + m)
    }

    fn temperature_scaled(&self, area: &Area) -> f32 {
        use bounds::*;
        let m = MIN_TEMP.abs();
        let s = area.temperature();
        (s + m) / (MAX_TEMP + m)
    }

    fn fertility_scaled(&self, area: &Area) -> f32 {
        let s = area.fertility() as f32;
        s / 100.0
    }

    fn rocks_scaled(&self, area: &Area) -> f32 {
        let s = area.rocks() as f32;
        s / 100.0
    }

    fn moisture_scaled(&self, area: &Area) -> f32 {
        let s = area.moisture() as f32;
        s / 100.0
    }
}
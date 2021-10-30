use std::sync::atomic::{AtomicUsize,Ordering};
use bevy_tilemap::{Tile,point::Point3};
use bevy::prelude::Color;

static ID: AtomicUsize = AtomicUsize::new(0);

pub type Location = (i32,i32);

#[derive(Debug,Clone)]
pub enum Biome {
    Grassland,// high movement, low cover, med forage
    Forest,   // low movement, provides cover
    Desert,   // med move, heatstroke?
    Tundra,   // med move, frostbite?
    Aquatic,  // freshwater or marine, very low move
}

#[derive(Debug,Clone)]
pub enum Soil {
    Clay,  // holds water, bad fertility
    Sand,  // low nutrients, low moisture, drain quickly
    Silt,  // erodes in rain, med moisture, med fertility
    Peat,  // high moisture, med-high fert
    Chalk, // low fertility, alkaline soil
    Loam,  // high fert, med moisture
}

impl Default for Biome {
    fn default() -> Self { Self::Grassland }
}

impl Default for Soil {
    fn default() -> Self { Self::Clay }
}

impl std::fmt::Display for Biome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::fmt::Display for Soil {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug,Default,Clone)]
pub struct Area {
    /// A unique id for each tile
    id: usize,

    /// The base texture id (ground) for this area
    texture: usize,

    /// The texture layer
    order: usize,

    /// The location of the area in the map
    location: Location,

    /// The biome that the area is in
    biome: Biome,

    /// The type of the soil in the area
    soil: Soil,

    /// The moisture content of the soil (0-100%)
    moisture: u8,

    /// the rockyness of the soil (0-100%)
    rocks: u8,

    /// The rate at which plants grow (0-100%)
    fertility: u8,

    /// The elevation relative to sea level (in meters)
    elevation: f32,

    /// The current temperature of the area (in celsius)
    temperature: f32,
}

impl Area {
    pub fn create() -> Self {
        Self {
            id: ID.fetch_add(1, Ordering::SeqCst),
            ..Default::default()
        }
    }

    pub fn with_texture<T: Into<usize>>(mut self, v: T) -> Self {
        self.texture = v.into();
        self
    }
    
    pub fn with_location(mut self, v: Location) -> Self {
        self.location = v;
        self
    }
    
    pub fn with_biome(mut self, v: Biome) -> Self {
        self.biome = v;
        self
    }
    
    pub fn with_soil(mut self, v: Soil) -> Self {
        self.soil = v;
        self
    }
    
    pub fn with_moisture<T: Into<u8>>(mut self, v: T) -> Self {
        self.moisture = v.into();
        self
    }
    
    pub fn with_rocks<T: Into<u8>>(mut self, v: T) -> Self {
        self.rocks = v.into();
        self
    }
    
    pub fn with_fertility<T: Into<u8>>(mut self, v: T) -> Self {
        self.fertility = v.into();
        self
    }
    
    pub fn with_elevation<T: Into<f32>>(mut self, v: T) -> Self {
        self.elevation = v.into();
        self
    }
    
    pub fn with_temperature<T: Into<f32>>(mut self, v: T) -> Self {
        self.temperature = v.into();
        self
    }

    pub fn build(mut self) -> Self {
        self.moisture = self.moisture.min(100);
        self.rocks = self.rocks.min(100);
        self.fertility = self.fertility.min(100);
        self
    }

    pub fn id(&self) -> usize {
        self.id.clone()
    }

    pub fn texture(&self) -> usize {
        self.texture.clone()
    }

    pub fn location(&self) -> Location {
        self.location.clone()
    }

    pub fn biome(&self) -> Biome {
        self.biome.clone()
    }

    pub fn soil(&self) -> Soil {
        self.soil.clone()
    }

    pub fn moisture(&self) -> u8 {
        self.moisture.clone()
    }

    pub fn rocks(&self) -> u8 {
        self.rocks.clone()
    }

    pub fn fertility(&self) -> u8 {
        self.fertility.clone()
    }

    pub fn elevation(&self) -> f32 {
        self.elevation.clone()
    }

    pub fn temperature(&self) -> f32 {
        self.temperature.clone()
    }

    pub fn tile(&self) -> Tile<Point3> {
        Tile {
            point: self.location.into(),
            sprite_order: self.order,
            sprite_index: self.texture,
            tint: Color::WHITE,
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_generated() {
        // make sure ID is reset to '0'
        ID.store(0, Ordering::SeqCst);

        let a1 = Area::create();
        let a2 = Area::create();

        assert_eq!(a1.id(), 0);
        assert_eq!(a2.id(), 1);
    }

    #[test]
    fn test_percentages_are_capped() {
        let a1 = Area::create()
            .with_moisture(110)
            .with_rocks(110)
            .with_fertility(110)
            .build();

        assert_eq!(a1.moisture(), 100);
        assert_eq!(a1.rocks(), 100);
        assert_eq!(a1.fertility(), 100);
    }
}
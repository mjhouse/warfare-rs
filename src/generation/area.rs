use bevy_tilemap::{Tile,point::Point3};
use bevy::prelude::Color;

use crate::generation::{Soil,Biome,id};
use std::fmt::{Display,Formatter,Result,Debug};

pub type Location = (i32,i32);

pub mod bounds {
    pub const MAX_ELEV: f32 = 4000.0;
    pub const MIN_ELEV: f32 = 0000.0;
    pub const MAX_TEMP: f32 =  50.0;
    pub const MIN_TEMP: f32 = -40.0;
}

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
pub enum Attribute {
    None,
    Biome,
    Soil,
    Elevation,
    Temperature,
    Fertility,
    Rocks,
    Moisture,
}

impl Default for Attribute {
    fn default() -> Self { Self::None }
}

impl Display for Attribute {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(self, f)
    }
}

#[derive(Debug,Default,Clone)]
pub struct Area {
    /// A unique id for each tile
    id: usize,

    /// The texture stack for this area
    textures: Vec<usize>,

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

    /// The degree to which this tile impedes movement
    impedance: u8,
}

impl Area {
    pub fn create() -> Self {
        Self {
            id: id::get(),
            ..Default::default()
        }
    }

    pub fn with_textures(mut self, v: Vec<usize>) -> Self {
        self.textures = v;
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

    pub fn with_impedance<T: Into<u8>>(mut self, v: T) -> Self {
        self.impedance = v.into();
        self
    }

    pub fn build(mut self) -> Self {
        use bounds::*;

        self.moisture = self.moisture
            .min(100);
            
        self.rocks = self.rocks
            .min(100);

        self.fertility = self.fertility
            .min(100);

        self.impedance = self.impedance
            .min(100);

        self.elevation = self.elevation
            .min(MAX_ELEV)
            .max(MIN_ELEV);

        self.temperature = self.temperature
            .min(MAX_TEMP)
            .max(MIN_TEMP);

        assert!(self.textures.len() > 0);

        self
    }

    pub fn id(&self) -> usize {
        self.id.clone()
    }

    pub fn texture(&self) -> Option<usize> {
        self.textures.get(0).cloned()
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

    pub fn impedance(&self) -> u8 {
        self.impedance.clone()
    }

    pub fn tiles(&self, max: usize) -> Vec<Tile<Point3>> {
        assert!(self.textures.len().saturating_sub(1) <= max);
        self.textures
            .iter()
            .enumerate()
            .map(|(i,t)| Tile {
                point: self.location.into(),
                sprite_order: i,
                sprite_index: *t,
                tint: Color::WHITE,
            }).collect()
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_generated() {
        // TODO: sometimes FAILS
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
            .with_textures(vec![0])
            .build();

        assert_eq!(a1.moisture(), 100);
        assert_eq!(a1.rocks(), 100);
        assert_eq!(a1.fertility(), 100);
    }

    #[test]
    fn test_values_are_capped() {
        let a1 = Area::create()
            .with_temperature(99999.999)
            .with_elevation(99999.999)
            .with_textures(vec![0])
            .build();

        let a2 = Area::create()
            .with_temperature(-9999.999)
            .with_elevation(-9999.999)
            .with_textures(vec![0])
            .build();

        assert!(a1.elevation() <= bounds::MAX_ELEV);
        assert!(a1.temperature() <= bounds::MAX_TEMP);

        assert!(a2.elevation() >= bounds::MIN_ELEV);
        assert!(a2.temperature() >= bounds::MIN_TEMP);
    }
}
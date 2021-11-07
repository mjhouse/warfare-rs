use std::sync::atomic::{AtomicUsize,Ordering};
use bevy_tilemap::{Tile,point::Point3};
use bevy::prelude::Color;

use std::fmt::{Display,Formatter,Result,Debug};
use rand::distributions::{Distribution, Standard};
use rand::Rng;

static ID: AtomicUsize = AtomicUsize::new(0);

pub type Location = (i32,i32);

pub mod bounds {
    pub const MAX_ELEV: f32 = 8848.0; // Mt Everest
    pub const MIN_ELEV: f32 = -414.0; // Dead Sea
    
    pub const MAX_TEMP: f32 = 56.6;
    pub const MIN_TEMP: f32 = -89.2;
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

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub enum Biome {
    None,     // no biome value
    Grassland,// high movement, low cover, med forage
    Forest,   // low movement, provides cover
    Desert,   // med move, heatstroke?
    Tundra,   // med move, frostbite?
    Aquatic,  // freshwater or marine, very low move
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub enum Soil {
    None,  // no soil value
    Clay,  // holds water, bad fertility
    Sand,  // low nutrients, low moisture, drain quickly
    Silt,  // erodes in rain, med moisture, med fertility
    Peat,  // high moisture, med-high fert
    Chalk, // low fertility, alkaline soil
    Loam,  // high fert, med moisture
}

impl Default for Attribute {
    fn default() -> Self { Self::None }
}

impl Default for Biome {
    fn default() -> Self { Self::Grassland }
}

impl Default for Soil {
    fn default() -> Self { Self::Clay }
}

impl Display for Biome {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(self, f)
    }
}

impl Display for Attribute {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(self, f)
    }
}

impl Display for Soil {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(self, f)
    }
}

impl From<u8> for Biome {
    fn from(v: u8) -> Self {
        match v {
            0 => Biome::Grassland,
            1 => Biome::Forest,
            2 => Biome::Desert,
            3 => Biome::Tundra,
            4 => Biome::Aquatic,
            _ => Biome::None,
        }
    }
}

impl From<u8> for Soil {
    fn from(v: u8) -> Self {
        match v {
            0 => Soil::Clay,
            1 => Soil::Sand,
            2 => Soil::Silt,
            3 => Soil::Peat,
            4 => Soil::Chalk,
            5 => Soil::Loam,
            _ => Soil::None,
        }
    }
}

impl Distribution<Biome> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Biome {
        rng.gen_range::<u8,_>(0..5).into()
    }
}

impl Distribution<Soil> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Soil {
        rng.gen_range::<u8,_>(0..6).into()
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

/*
conceptually, area attributes are  considered to be part of different "layers" where 
later attributes (in "higher" layers) depend on earlier, more basic attributes. At the 
base is location, which is always fixed. Attributes that rely on location are considering
the attributes of neighboring regions during generation.

Second-layer attributes are determined with some sort of mathematical noise calculation and
optionally neighboring tiles.

Third-layer attributes are set using some combination of second layer attribute values.

layer #1: 
    location:    fixed

layer #2:
    elevation:   noise + location
    biome:       noise + location
    
layer #3:
    moisture:    elevation
    soil:        biome + elevation
    temperature: elevation + noise

layer #4:
    rocks:       elevation + location
    fertility:   soil + moisture - elevation
*/

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
        use bounds::*;

        self.moisture = self.moisture
            .min(100);
        self.rocks = self.rocks
            .min(100);
        self.fertility = self.fertility
            .min(100);
        self.elevation = self.elevation
            .min(MAX_ELEV)
            .max(MIN_ELEV);
        self.temperature = self.temperature
            .min(MAX_TEMP)
            .max(MIN_TEMP);

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

    #[test]
    fn test_values_are_capped() {
        let a1 = Area::create()
            .with_temperature(99999.999)
            .with_elevation(99999.999)
            .build();

        let a2 = Area::create()
            .with_temperature(-9999.999)
            .with_elevation(-9999.999)
            .build();

        assert!(a1.elevation() <= bounds::MAX_ELEV);
        assert!(a1.temperature() <= bounds::MAX_TEMP);

        assert!(a2.elevation() >= bounds::MIN_ELEV);
        assert!(a2.temperature() >= bounds::MIN_TEMP);
    }
}
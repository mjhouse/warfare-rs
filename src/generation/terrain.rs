#![allow(unused)]

use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::fmt::{Debug, Display, Formatter, Result};

use crate::generation::WeatherType;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Biome {
    None,      // no biome value
    Grassland, // high movement, low cover, med forage
    Forest,    // low movement, provides cover
    Desert,    // med move, heatstroke?
    Tundra,    // med move, frostbite?
    Aquatic,   // freshwater or marine, very low move
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Soil {
    None,  // no soil value
    Clay,  // holds water, bad fertility
    Sand,  // low nutrients, low moisture, drain quickly
    Silt,  // erodes in rain, med moisture, med fertility
    Peat,  // high moisture, med-high fert
    Chalk, // low fertility, alkaline soil
    Loam,  // high fert, med moisture
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Foliage {
    Grass,
    Trees,
    Brush,
    Crops,
    Rocks,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Structure {
    None,
    Trenches,
    Base, // FOB, FSB, Arsenal, etc.
    Village,
    Town,
    City,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Time {
    Day,
    Night,
}

/// stack of enums for everything that a tile contains
pub struct Terrain((Biome, Soil, Foliage, Structure, WeatherType, Time));

impl Default for Biome {
    fn default() -> Self {
        Self::None
    }
}

impl Default for Soil {
    fn default() -> Self {
        Self::None
    }
}

impl Display for Biome {
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
        rng.gen_range::<u8, _>(0..5).into()
    }
}

impl Distribution<Soil> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Soil {
        rng.gen_range::<u8, _>(0..6).into()
    }
}

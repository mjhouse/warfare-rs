use std::collections::hash_map::HashMap;
use bevy::asset::HandleUntyped;
use bevy::sprite::TextureAtlas;
use bevy::asset::AssetServer;
use bevy_tilemap::point::Point3;
use bevy_tilemap::chunk::LayerKind;
use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::objects::Location;
use crate::resources::{Spectrum,Textures};
use crate::error::{Error,Result};
use crate::state::{
    Events,
    Calendar,
    traits::{
        Moveable,
        Positioned,
    },
};

use crate::generation::{
    bounds,
    Area,
    Attribute,
    Soil,
    Factors,
    Generator,
    Layers,
    Unit,
    Marker,
};

static CONTEXT: Lazy<Mutex<Context>> = Lazy::new(|| Mutex::new(Context::default()));

#[derive(Debug,Clone)]
pub struct Context {
    pub height: u32,
    pub width: u32,
}

#[derive(Default, Clone)]
pub struct Terrain {
    pub selected: Area,
    pub overlay: Attribute,
    pub seed: String,
    pub size: String,
}

#[derive(Clone)]
pub struct State {
    /// tile icon resources
    pub textures: Textures,

    /// the map layers to build
    pub layers: Layers,

    /// user-supplied factors that control generator
    pub factors: Factors,

    /// the map generator to use
    pub generator: Generator,

    /// current selection of areas
    pub areas: HashMap<Location,Area>,

    /// current points and layers of graphics
    pub tiles: Vec<(Point3,usize)>,

    /// available overlay displays
    pub overlay: HashMap<Attribute,Spectrum>,

    /// terrain flags and information
    pub terrain: Terrain,

    /// resource loaded flag
    pub loaded: bool,

    /// events for systems
    pub events: Events,

    /// in-game date and turn count
    pub calendar: Calendar,

    /// all units on the board
    pub units: Vec<Unit>,

    pub marker: Marker,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            height: 30,
            width: 30,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            textures: Default::default(),
            layers: Default::default(),
            factors: Default::default(),
            generator: Default::default(),
            areas: Default::default(),
            tiles: Default::default(),
            overlay: Default::default(),
            terrain: Default::default(),
            loaded: Default::default(),
            events: Default::default(),
            calendar: Default::default(),
            units: Default::default(),
            marker: Default::default(),
        }
    }
}

impl Context {
    pub fn width() -> u32 {
        CONTEXT.lock().expect("No context").width
    }

    pub fn height() -> u32 {
        CONTEXT.lock().expect("No context").height
    }

    pub fn size() -> (i32,i32) {
        ( Context::width()  as i32, 
          Context::height() as i32)
    }
}

impl State {

    pub fn set_map_size(&self, width: u32, height: u32) {
        let mut c = CONTEXT.lock().expect("Could not update context");
        c.width = width;
        c.height = height;
    }

    pub fn context(&self) -> Context {
        CONTEXT
            .lock()
            .expect("No context")
            .clone()
    }

    pub fn add(&mut self, area: Area) {
        self.areas.insert(area.location(),area);
    }

    pub fn add_all(&mut self, areas: Vec<Area>) {
        for area in areas.into_iter() {
            self.add(area);
        }
    }

    pub fn has_unit(&self, location: &Location) -> bool {
        for unit in self.units.iter() {
            if unit.get_position() == *location {
                return true;
            }
        }
        false
    }

    pub fn find_unit(&mut self, location: &Location) -> Option<&mut Unit> {
        for unit in self.units.iter_mut() {
            if unit.get_position() == *location {
                return Some(unit);
            }
        }
        None
    }

    pub fn get_texture(&self, loc: &Location) -> usize {
        let blank = self.textures.get("blank");
        match self.areas.get(loc) {
            Some(a) => a.texture().unwrap_or(blank),
            None => blank,
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
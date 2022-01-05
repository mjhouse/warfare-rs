use bevy_tilemap::point::Point3;
use once_cell::sync::Lazy;
use std::collections::hash_map::HashMap;
use std::sync::Mutex;
use log::*;

use crate::objects::Location;
use crate::objects::Point;
use crate::objects::Map;
use crate::resources::{Spectrum, Textures, Label};
use crate::state::Action;
use crate::state::Flags;

use crate::state::{traits::*, Calendar, Events};
use crate::networking::messages::*;
use crate::generation::{bounds, Area, Attribute, Cursor, Factors, Generator, Layers, Unit};

static CONTEXT: Lazy<Mutex<Context>> = Lazy::new(|| Mutex::new(Context::default()));

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum StateFlag {
    Loaded,
}

#[derive(Debug, Clone)]
pub struct Context {
    pub width: u32,
    pub height: u32,
    pub tile_width: u32,
    pub tile_height: u32,
}

#[derive(Default, Clone)]
pub struct Terrain {
    pub selected: Area,
    pub overlay: Attribute,
    pub seed: String,
    pub size: String,
}

pub struct State {
    flags: Flags<StateFlag>,

    /// tile icon resources
    pub textures: Textures,

    /// the map layers to build
    pub layers: Layers,

    /// user-supplied factors that control generator
    pub factors: Factors,

    /// the map generator to use
    pub generator: Generator,

    /// current selection of areas
    pub areas: HashMap<Location, Area>,

    /// current points and layers of graphics
    pub tiles: Vec<(Point3, usize)>,

    /// available overlay displays
    pub overlay: HashMap<Attribute, Spectrum>,

    /// terrain flags and information
    pub terrain: Terrain,

    /// events for systems
    pub events: Events,

    /// in-game date and turn count
    pub calendar: Calendar,

    /// all units on the board
    pub units: Map,

    pub cursor: Cursor,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            width: 30,
            height: 30,
            tile_width: 175,
            tile_height: 200,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            flags: Flags::new(),
            textures: Default::default(),
            layers: Default::default(),
            factors: Default::default(),
            generator: Default::default(),
            areas: Default::default(),
            tiles: Default::default(),
            overlay: Default::default(),
            terrain: Default::default(),
            events: Default::default(),
            calendar: Default::default(),
            units: Map::new(),
            cursor: Default::default(),
        }
    }
}

impl Terrain {
    pub fn seed(&self) -> u32 {
        self.seed
            .parse::<u32>()
            .unwrap_or(0)
    }
}

macro_rules! context {
    () => {
        CONTEXT.lock().expect("Could not lock context")
    };
}

impl Context {
    pub fn total() -> usize {
        let c = context!();
        c.width as usize * 
        c.height as usize
    }

    pub fn width() -> u32 {
        context!().width
    }

    pub fn height() -> u32 {
        context!().height
    }

    pub fn tile_width() -> u32 {
        context!().tile_width
    }

    pub fn tile_height() -> u32 {
        context!().tile_height
    }

    pub fn size() -> (i32, i32) {
        let c = context!();
        (c.width as i32, c.height as i32)
    }

    pub fn tile_size() -> (i32, i32) {
        let c = context!();
        (c.tile_width as i32, c.tile_height as i32)
    }

    pub fn clone() -> Context {
        context!().clone()
    }

    pub fn set_size(w: u32, h: u32) {
        let mut c = context!();
        c.width = w;
        c.height = h;
    }

    pub fn set_tile_size(w: u32, h: u32) {
        let mut c = context!();
        c.tile_width = w;
        c.tile_height = h;
    }

    pub fn init(w: u32, h: u32, tw: u32, th: u32) -> Context {
        Self::set_size(w, h);
        Self::set_tile_size(tw, th);
        Self::clone()
    }
}

impl State {
    pub fn is_loaded(&self) -> bool {
        self.flags.get(StateFlag::Loaded)
    }

    pub fn set_loaded(&mut self) {
        self.flags.set(StateFlag::Loaded);
    }

    pub fn sync(&mut self, data: TerrainData) {
        self.terrain.seed = format!("{}",data.seed);
        self.calendar = Calendar::from_turn(data.turn);
        self.factors = data.factors;
        self.events.send(Action::UpdateTerrain);
    }

    pub fn seed(&self) -> u32 {
        self.terrain.seed()
    }

    pub fn turn(&self) -> u32 {
        self.calendar.turn()
    }

    pub fn factors(&self) -> Factors {
        self.factors.clone()
    }

    pub fn end_turn(&mut self) {
        self.calendar.advance();
        for unit in self.units.units_mut() {
            unit.reset_actions()
        }
    }

    pub fn impedance_map(&self) -> HashMap<Point, f32> {
        self.areas
            .iter()
            .map(|(l, a)| (Point::from(*l), a.impedance() as f32))
            .collect()
    }

    pub fn add(&mut self, area: Area) {
        self.areas.insert(area.location(), area);
    }

    pub fn add_all(&mut self, areas: Vec<Area>) {
        for area in areas.into_iter() {
            self.add(area);
        }
    }

    pub fn has_unit(&self, point: &Point) -> bool {
        self.units.count_units(point) > 0
    }

    pub fn find_units(&self, location: &Location) -> Vec<&Unit> {
        self.units.get_units(&(*location).into())
    }

    pub fn get_texture(&self, loc: &Location) -> usize {
        let blank = self.textures.get(Label::Blank);
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
            None => 0.0,
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

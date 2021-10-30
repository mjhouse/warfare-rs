use std::collections::hash_map::HashMap;
use crate::area::{Location,Area};

#[derive(Default, Clone)]
pub struct State {
    pub areas: HashMap<Location,Area>,
    pub loaded: bool,
}

impl State {
    pub fn add(&mut self, area: Area) {
        self.areas.insert(area.location(),area);
    }

    pub fn add_all(&mut self, areas: Vec<Area>) {
        for area in areas.into_iter() {
            self.areas.insert(area.location(),area);
        }
    }
}
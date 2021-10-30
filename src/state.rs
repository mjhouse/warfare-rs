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
}
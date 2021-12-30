use serde::{Deserialize, Serialize};
use crate::generation::{id, Id, LayerUse, Marker};
use crate::objects::Name;
use crate::objects::Point;
use crate::state::demographics::{Demographics, Sex};
use crate::state::traits::*;
use crate::state::State;
use crate::resources::Label;
use rand_pcg::Pcg64;

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum Specialty {
    Infantry,
    Medical,
    Logistics,
    Tanks,
    Driver,
    Mechanic,
    // ... etc
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Soldier {
    skill: Specialty,  // occupation
    name: Name,        // name
    sex: Sex,          // sex
    age: u8,           // age in years
    weight: u16,       // weight in kg
    height: u16,       // height in cm
    actions: (u8, u8), // (value,max)
    health: (u8, u8),  // (value,max)
    veteran: (u8, u8), // (value,max)
    morale: (u8, u8),  // (value,max)
    defense: (u8, u8), // (value,max)
    attack: (u8, u8),  // (value,max)
}

impl Soldier {
    pub fn new(skill: &Specialty) -> Self {
        let mut demo = Demographics::new();
        let sex = demo.sex();
        let name = demo.name(&sex);
        Self {
            skill: skill.clone(),
            name: name,
            sex: sex.clone(),
            age: demo.age(&sex),
            weight: demo.weight(&sex),
            height: demo.height(&sex),
            actions: (100, 100),
            health: (100, 100),
            veteran: (0, 100),
            morale: (100, 100),
            defense: (100, 100),
            attack: (100, 100),
        }
    }

    pub fn reset_actions(&mut self) {
        self.actions.0 = self.actions.1;
    }

    pub fn use_actions(&mut self, v: u8) {
        let value = &mut self.actions.0;
        *value = value.saturating_sub(v);
    }

    pub fn actions(&self) -> u8 {
        self.actions.0
    }

    pub fn max_actions(&self) -> u8 {
        self.actions.1
    }

    pub fn name(&self) -> String {
        self.name.full()
    }

    pub fn age(&self) -> u8 {
        self.age
    }

    pub fn sex(&self) -> Sex {
        self.sex
    }

    pub fn weight(&self) -> u16 {
        self.weight
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn specialty(&self) -> Specialty {
        self.skill.clone()
    }

    pub fn health(&self) -> (u8, u8) {
        self.health
    }

    pub fn morale(&self) -> (u8, u8) {
        self.morale
    }

    pub fn defense(&self) -> (u8, u8) {
        self.defense
    }

    pub fn attack(&self) -> (u8, u8) {
        self.attack
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Unit {
    /// globally unique id
    id: Id,

    /// display information
    marker: Marker,

    /// the unit specialty
    specialty: Specialty,

    /// soldiers in this unit
    soldiers: Vec<Soldier>,
}

impl Default for Unit {
    fn default() -> Self {
        Self {
            id: Id::new(),
            marker: Marker {
                layer: 0,
                texture: 0,
                position: (0, 0).into(),
            },
            specialty: Specialty::Infantry,
            soldiers: vec![],
        }
    }
}

impl Unit {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_specialty(mut self, specialty: Specialty) -> Self {
        self.specialty = specialty;
        self
    }

    pub fn with_soldiers(mut self, count: usize) -> Self {
        self.soldiers.clear();
        self.soldiers = Vec::with_capacity(count);
        self
    }

    pub fn with_position(mut self, point: Point) -> Self {
        self.marker.position = point;
        self
    }

    pub fn build(mut self, state: &State) -> Self {
        self.marker.layer = state
            .layers
            .get(&LayerUse::Units)
            .expect("Must have unit layer");

        self.marker.texture = state.textures.get(Label::Unit);

        for _ in 0..self.soldiers.capacity() {
            self.soldiers.push(
                Soldier::new(&self.specialty));
        }

        self
    }

    pub fn reset_actions(&mut self) {
        for soldier in self.soldiers.iter_mut() {
            soldier.reset_actions();
        }
    }

    pub fn use_actions(&mut self, actions: u8) {
        for soldier in self.soldiers.iter_mut() {
            soldier.use_actions(actions);
        }
    }

    pub fn actions(&self) -> u8 {
        let s = &self.soldiers;
        let v: usize = s
            .iter()
            .map(Soldier::actions)
            .map(|v| v as usize)
            .sum::<usize>()
            / s.len();
        v.min(255) as u8
    }

    pub fn max_actions(&self) -> u8 {
        let s = &self.soldiers;
        let v: usize = s
            .iter()
            .map(Soldier::max_actions)
            .map(|v| v as usize)
            .sum::<usize>()
            / s.len();
        v.min(255) as u8
    }

    pub fn soldiers(&self) -> &Vec<Soldier> {
        &self.soldiers
    }

    pub fn specialty(&self) -> Specialty {
        self.specialty.clone()
    }
}

impl HasMarker for Unit {
    fn marker(&self) -> &Marker {
        &self.marker
    }

    fn marker_mut(&mut self) -> &mut Marker {
        &mut self.marker
    }
}

impl HasId for Unit {
    fn id(&self) -> &Id {
        &self.id
    }
}

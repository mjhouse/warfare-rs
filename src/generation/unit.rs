use serde::{Deserialize, Serialize};
use crate::generation::{Id, PlayerId, LayerUse, Marker};
use crate::objects::Name;
use crate::objects::Point;
use crate::state::demographics::{Demographics, Sex};
use crate::state::traits::*;
use crate::state::State;
use crate::resources::Label;
use rand_pcg::Pcg64;
use crate::networking::messages::PlayerData;
use rand::{Rng,rngs::ThreadRng};

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum Specialty {
    Infantry,
    Armor,
    Militia,
    Medical,
    Logistics,    
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

    pub fn actions(&self) -> (u8, u8) {
        self.actions
    }

    pub fn current_actions(&self) -> u8 {
        self.actions.0
    }

    pub fn maximum_actions(&self) -> u8 {
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

    pub fn set_health(&mut self, v: i16) {
        self.health.0 = (self.health.0 as i16)
            .saturating_add(v)
            .max(0)
            .min(255) as u8;
    }

    pub fn current_health(&self) -> u8 {
        self.health.0
    }

    pub fn maximum_health(&self) -> u8 {
        self.health.1
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

    pub fn current_attack(&self) -> u8 {
        self.attack.0
    }

    pub fn maximum_attack(&self) -> u8 {
        self.attack.1
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Unit {
    /// globally unique id
    id: Id,

    /// the name of this unit
    name: String,

    /// the player that owns this unit
    player_id: PlayerId,

    /// the name of the owning player
    player_name: String,

    /// the join order of the player
    player_order: u8,

    /// display information
    marker: Marker,

    /// the unit specialty
    specialty: Specialty,

    /// soldiers in this unit
    soldiers: Vec<Soldier>,
}

/// Combined units for unit-to-unit interactions:
///     * combat
///     * merging
///     * transfer
///     * divide
///     * etc.
pub struct Units<'a> {
    /// aggregated units
    units: Vec<&'a Unit>,
}

/// Collection of changes to apply to units
pub struct Changes {
    changes: Vec<Change>,
}

/// A single change for a particular unit
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Change {
    pub id: Id,
    pub point: Point,
    pub action: ChangeType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ChangeType {
    Health(i16),
    Morale(i16),
    Attack(i16),
}

impl Unit {
    pub fn new(id: PlayerId) -> Self {
        Self {
            id: Id::new(),
            name: "".into(),

            player_id: id,
            player_name: "".into(),
            player_order: 0,

            marker: Marker {
                layer: 0,
                texture: 0,
                position: (0, 0).into(),
            },
            specialty: Specialty::Infantry,
            soldiers: vec![],
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn with_player(mut self, player: PlayerData) -> Self {
        self.player_id = player.id;
        self.player_name = player.name.clone();
        self.player_order = player.order as u8;
        self
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

        self.marker.texture = state.textures.unit(&self.specialty,self.player_order);

        for _ in 0..self.soldiers.capacity() {
            self.soldiers.push(
                Soldier::new(&self.specialty));
        }

        self
    }

    pub fn rebuild(mut self, state: &State) -> Self {
        self.marker.layer = state
            .layers
            .get(&LayerUse::Units)
            .expect("Must have unit layer");

        self.marker.texture = state.textures.unit(&self.specialty,self.player_order);
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

    pub fn set_health(&mut self, v: i16) {
        for soldier in self.soldiers.iter_mut() {
            soldier.set_health(v);
        }
    }

    pub fn health(&self) -> u8 {
        let s = &self.soldiers;
        let v: usize = s
            .iter()
            .map(Soldier::current_health)
            .map(|v| v as usize)
            .sum::<usize>()
            / s.len();
        v.min(255) as u8
    }

    pub fn attack(&self) -> u8 {
        let s = &self.soldiers;
        let v: usize = s
            .iter()
            .map(Soldier::current_attack)
            .map(|v| v as usize)
            .sum::<usize>()
            / s.len();
        v.min(255) as u8
    }

    pub fn actions(&self) -> u8 {
        let s = &self.soldiers;
        let v: usize = s
            .iter()
            .map(Soldier::current_actions)
            .map(|v| v as usize)
            .sum::<usize>()
            / s.len();
        v.min(255) as u8
    }

    pub fn max_actions(&self) -> u8 {
        let s = &self.soldiers;
        let v: usize = s
            .iter()
            .map(Soldier::maximum_actions)
            .map(|v| v as usize)
            .sum::<usize>()
            / s.len();
        v.min(255) as u8
    }

    pub fn soldiers(&self) -> &Vec<Soldier> {
        &self.soldiers
    }

    pub fn specialty(&self) -> &Specialty {
        &self.specialty
    }

    pub fn player_id(&self) -> &PlayerId {
        &self.player_id
    }

    pub fn player_name(&self) -> String {
        self.player_name.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl<'a> Units<'a> {
    pub fn aggregate(units: Vec<&'a Unit>) -> Self {
        assert!(units.len() > 0);
        Self { 
            units: units,
        }
    }

    pub fn attack(&self, other: &Units<'_>) -> Vec<Change> {
        let mut atk1 = self.current_attack();
        let mut atk2 = other.current_attack();
        let var1 = rand::thread_rng().gen::<u8>() / 10;
        let var2 = rand::thread_rng().gen::<u8>() / 10;

        atk1 = atk1.saturating_add(var1);
        atk2 = atk2.saturating_add(var2);
        
        if atk1 > atk2 {
            let mut changes1 = self
                .units()
                .iter()
                .map(|u| Change::health(u,-50))
                .collect::<Vec<Change>>();
            let mut changes2 = other
                .units()
                .iter()
                .map(|u| Change::health(u,-100))
                .collect::<Vec<Change>>();
            changes1.append(&mut changes2);
            changes1
        }
        else if atk1 < atk2 {
            let mut changes1 = self
                .units()
                .iter()
                .map(|u| Change::health(u,-100))
                .collect::<Vec<Change>>();
            let mut changes2 = other
                .units()
                .iter()
                .map(|u| Change::health(u,-50))
                .collect::<Vec<Change>>();
            changes1.append(&mut changes2);
            changes1
        }
        else {
            let mut changes1 = self
                .units()
                .iter()
                .map(|u| Change::health(u,-50))
                .collect::<Vec<Change>>();
            let mut changes2 = other
                .units()
                .iter()
                .map(|u| Change::health(u,-50))
                .collect::<Vec<Change>>();
            changes1.append(&mut changes2);
            changes1
        }
    }

    pub fn current_attack(&self) -> u8 {
        let s = &self.units;
        let v: usize = s
            .iter()
            .map(|u| u.attack())
            .map(|v| v as usize)
            .sum::<usize>()
            / s.len();
        v.min(255) as u8
    }

    pub fn units(&self) -> Vec<&Unit> {
        self.units.clone()
    }
}

impl Change {
    pub fn health(unit: &Unit, change: i16) -> Self {
        Self {
            id: *unit.id(),
            point: *unit.position(),
            action: ChangeType::Health(change),
        }
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

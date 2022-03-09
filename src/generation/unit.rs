use serde::{Deserialize, Serialize};
use crate::generation::{Id, PlayerId, LayerUse, Marker};
use crate::objects::{Name,Point,Property};
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
    actions: Property,
    health: Property,
    veteran: Property,
    morale: Property,
    defense: Property,
    attack: Property,
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
            actions: Property::new(100, 0, 100),
            health: Property::new(100, 0, 100),
            veteran: Property::new(0, 0, 100),
            morale: Property::new(100, 0,100),
            defense: Property::new(100, 0, 100),
            attack: Property::new(100, 0, 100),
        }
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

    pub fn actions(&self) -> &Property {
        &self.actions
    }

    pub fn actions_mut(&mut self) -> &mut Property {
        &mut self.actions
    }

    pub fn health(&self) -> &Property {
        &self.health
    }

    pub fn health_mut(&mut self) -> &mut Property {
        &mut self.health
    }

    pub fn veteran(&self) -> &Property {
        &self.veteran
    }

    pub fn veteran_mut(&mut self) -> &mut Property {
        &mut self.veteran
    }
    
    pub fn morale(&self) -> &Property {
        &self.morale
    }

    pub fn morale_mut(&mut self) -> &mut Property {
        &mut self.morale
    }

    pub fn defense(&self) -> &Property {
        &self.defense
    }

    pub fn defense_mut(&mut self) -> &mut Property {
        &mut self.defense
    }

    pub fn attack(&self) -> &Property {
        &self.attack
    }

    pub fn attack_mut(&mut self) -> &mut Property {
        &mut self.attack
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
            soldier.actions_mut().reset();
        }
    }

    pub fn use_actions(&mut self, actions: u8) {
        for soldier in self.soldiers.iter_mut() {
            soldier.actions_mut().update(actions as i64 * -1);
        }
    }

    pub fn set_health(&mut self, v: i16) {
        for soldier in self.soldiers.iter_mut() {
            soldier.health_mut().set(v);
        }
    }

    pub fn health(&self) -> u8 {
        let s = &self.soldiers;
        let v: usize = s
            .iter()
            .map(|s| s.health().val())
            .map(|v| v as usize)
            .sum::<usize>()
            / s.len();
        v.min(255) as u8
    }

    pub fn attack(&self) -> u8 {
        let s = &self.soldiers;
        let v: usize = s
            .iter()
            .map(|s| s.attack().max())
            .map(|v| v as usize)
            .sum::<usize>()
            / s.len();
        v.min(255) as u8
    }

    pub fn actions(&self) -> u8 {
        let s = &self.soldiers;
        let v: usize = s
            .iter()
            .map(|s| s.actions().val())
            .map(|v| v as usize)
            .sum::<usize>()
            / s.len();
        v.min(255) as u8
    }

    pub fn max_actions(&self) -> u8 {
        let s = &self.soldiers;
        let v: usize = s
            .iter()
            .map(|s| s.actions().max())
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

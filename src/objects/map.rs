use crate::generation::id::Id;
use crate::generation::Unit;
use crate::objects::Point;
use crate::state::traits::HasId;
use crate::state::Context;

use indexmap::IndexMap;

/// max units at a given position
const MAX: usize = 5;

#[derive(Debug, Clone)]
pub struct Position {
    units: IndexMap<Id, Unit>,
}

pub struct Map {
    positions: Vec<Position>,
    selected: Vec<Unit>,
}

impl Position {
    pub fn new() -> Self {
        Self {
            units: IndexMap::with_capacity(MAX),
        }
    }

    pub fn full(&self) -> bool {
        self.units.len() >= MAX
    }

    pub fn list(&self) -> Vec<&Unit> {
        self.units.iter().map(|(_, v)| v).collect()
    }

    pub fn take(&mut self, id: Id) -> Option<Unit> {
        self.units.shift_remove(&id)
    }

    pub fn take_all(&mut self, ids: Vec<Id>) -> Vec<Unit> {
        ids.into_iter()
            .map(|i| self.take(i))
            .filter_map(|u| u)
            .collect()
    }

    pub fn push(&mut self, unit: Unit) {
        if !self.full() {
            self.units.insert(unit.id().clone(), unit);
        }
    }

    pub fn pop(&mut self) -> Option<Unit> {
        self.units.pop().map(|p| p.1)
    }
}

impl Map {
    pub fn new() -> Self {
        let size = Context::total();
        Self {
            positions: vec![Position::new(); size],
            selected: vec![],
        }
    }
    
    pub fn select_top(&mut self, point: Point) -> Option<&Unit> {
        let unit = self
            .get_mut(point)
            .map(|p| p.pop())
            .flatten()?;
        self.add(unit)
    }

    pub fn select_id(&mut self, point: Point, id: Id) -> Option<&Unit> {
        let unit = self
            .get_mut(point)
            .map(|p| p.take(id))
            .flatten()?;
        self.add(unit)
    }

    pub fn select_ids(&mut self, point: Point, ids: Vec<Id>) -> Vec<&Unit> {
        let units = self
            .get_mut(point)
            .map(|p| p.take_all(ids))
            .unwrap_or(Vec::new());
        self.add_all(units)
    }

    fn add(&mut self, unit: Unit) -> Option<&Unit> {
        self.selected.push(unit);
        self.selected.last()
    }

    fn add_all(&mut self, mut units: Vec<Unit>) -> Vec<&Unit> {
        let l = self.selected.len() - units.len();
        self.selected.append(&mut units);
        self.selected[l..].iter().collect()
    }

    fn get_mut(&mut self, point: Point) -> Option<&mut Position> {
        let index = point.to_index() as usize;
        self.positions.get_mut(index)
    }

    fn get(&self, point: Point) -> Option<&Position> {
        let index = point.to_index() as usize;
        self.positions.get(index)
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_succeeds() {}
}

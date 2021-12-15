use crate::generation::id::Id;
use crate::generation::Unit;
use crate::objects::Point;
use crate::state::traits::HasId;
use crate::state::Context;
use crate::error::{Result,Error};

use indexmap::IndexMap;

/// alias for map index position
type Index = usize;

/// max units at a given position
const MAX: usize = 5;

#[derive(Default, Debug, Clone, Copy)]
pub struct Selection {
    pub index: Index,
    pub id:    Id,
}

#[derive(Debug, Clone)]
pub struct Position {
    units: IndexMap<Id, Unit>,
}

#[derive(Debug, Clone)]
pub struct Map {
    positions: Vec<Position>,
    selected: Vec<Selection>,
}

impl Selection {
    pub fn new(point: &Point, unit: &Unit) -> Self {
        Self {
            index: point.as_index() as usize,
            id: *unit.id(),
        }
    }
    pub fn update(&mut self, point: &Point) {
        self.index = point.as_index() as usize;
    }
}

impl Position {
    pub fn new() -> Self {
        Self {
            units: IndexMap::with_capacity(MAX),
        }
    }

    /// remaining space in this position
    pub fn space(&self) -> usize {
        MAX.saturating_sub(self.units.len())
    }

    /// check if position is full
    pub fn full(&self) -> bool {
        self.units.len() > MAX
    }

    /// get a reference to all units
    pub fn list(&self) -> Vec<&Unit> {
        self.units.iter().map(|(_, v)| v).collect()
    }

    /// get a mutable reference to all units
    pub fn list_mut(&mut self) -> Vec<&mut Unit> {
        self.units.iter_mut().map(|(_, v)| v).collect()
    }

    /// give a unit to this position
    pub fn give(&mut self, unit: Unit) {
        if !self.full() {
            self.units.insert(unit.id().clone(), unit);
        }
    }

    /// give a unit to this position
    pub fn give_all(&mut self, units: Vec<Unit>) {
        for unit in units.into_iter() {
            self.give(unit);
        }
    }

    /// take a unit from this position
    pub fn take(&mut self, id: Id) -> Option<Unit> {
        self.units.shift_remove(&id)
    }

    /// check if position contains unti
    pub fn contains(&self, id: Id) -> bool {
        self.units.contains_key(&id)
    }

    /// get a reference to the top unit
    pub fn top(&self) -> Option<&Unit> {
        self.units.last().map(|p| p.1)
    }

    /// get a reference to a particular unit
    pub fn id(&self, id: &Id) -> Option<&Unit> {
        self.units.get(id)
    }

    /// get references to multiple units
    pub fn ids(&self, ids: &Vec<Id>) -> Vec<&Unit> {
        ids.iter()
            .map(|i| self.id(i))
            .filter_map(|u| u)
            .collect()
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

    pub fn count(&self) -> usize {
        self.selected.len()
    }

    pub fn select_none(&mut self) {
        self.selected.clear();
    }

    pub fn select_top(&mut self, point: Point) {
        if let Some(unit) = self.get_top(&point) {
            self.selected.push(Selection::new(&point,unit));
        }
    }

    pub fn select_id(&mut self, point: Point, id: Id) {
        if let Some(unit) = self.get_id(&point,&id) {
            self.selected.push(Selection::new(&point,unit));
        }
    }

    pub fn select_ids(&mut self, point: Point, ids: Vec<Id>) {
        self.selected
            .append(&mut self
                .get_ids(&point,&ids)
                .iter()
                .map(|u| Selection::new(&point,u))
                .collect());
    }

    pub fn get_top(&self, point: &Point) -> Option<&Unit> {
        self.get(point)
            .map(|p| p.top())
            .flatten()
    }

    pub fn get_id(&self, point: &Point, id: &Id) -> Option<&Unit> {
        self.get(point)
            .map(|p| p.id(id))
            .flatten()
    }

    pub fn get_ids(&self, point: &Point, ids: &Vec<Id>) -> Vec<&Unit> {
        self.get(point)
            .map(|p| p.ids(ids))
            .unwrap_or(Vec::new())
    }

    fn get_mut(&mut self, point: &Point) -> Option<&mut Position> {
        let index = point.as_index() as usize;
        self.positions.get_mut(index)
    }

    fn get(&self, point: &Point) -> Option<&Position> {
        let index = point.as_index() as usize;
        self.positions.get(index)
    }

    /// move selection to a new position
    pub fn move_selection(&mut self, point: &Point) -> Result<()> {
        let count = self.count();
        let space = self.get(point)
            .ok_or(Error::TargetNotFound)?
            .space();

        // fail if no units are selected
        if count == 0 {
            return Err(Error::NoSelection);
        }

        // fail if target doesn't have enough space
        if space < count {
            return Err(Error::TargetTooSmall);
        }

        // find all selected units
        let units = self.selected
            .clone()
            .into_iter()
            .filter_map(|s| self
                .positions
                .get_mut(s.index)
                .map(|p| p.take(s.id))
                .flatten())
            .collect();

        // give units to new location
        if let Some(target) = self.get_mut(point) {
            target.give_all(units);
        }

        // update selection to the new position
        self.selected
            .iter_mut()
            .map(|p| p.update(point));

        Ok(())
    }

    pub fn units(&self) -> Vec<&Unit> {
        self.positions
            .iter()
            .map(|p| p.list())
            .flatten()
            .collect()
    }

    pub fn units_mut(&mut self) -> Vec<&mut Unit> {
        self.positions
            .iter_mut()
            .map(|p| p.list_mut())
            .flatten()
            .collect()
    }

    pub fn add(&mut self, point: Point, unit: Unit) {
        if let Some(target) = self.get_mut(&point) {
            target.give(unit);
        }
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

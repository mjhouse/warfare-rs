use crate::generation::id::Id;
use crate::generation::Unit;
use crate::objects::Point;
use crate::state::traits::HasId;
use crate::state::Context;
use crate::error::{Result,Error};
use crate::state::traits::{HasPosition,AsTile,HasLayer};

use bevy_tilemap::Tilemap;
use bevy_tilemap::Tile;

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

    pub fn count(&self) -> usize {
        self.units.len()
    }

    /// remaining space in this position
    pub fn space(&self) -> usize {
        MAX.saturating_sub(self.count())
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

    /// check if position contains unit
    pub fn contains(&self, id: &Id) -> bool {
        self.units.contains_key(id)
    }

    /// get a reference to the top unit
    pub fn top(&self) -> Option<&Unit> {
        self.units.last().map(|p| p.1)
    }

    /// get a reference to a particular unit
    pub fn id(&self, id: &Id) -> Option<&Unit> {
        self.units.get(id)
    }

    /// get a mutablereference to a particular unit
    pub fn id_mut(&mut self, id: &Id) -> Option<&mut Unit> {
        self.units.get_mut(id)
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

    pub fn count_units(&self, point: &Point) -> usize {
        self.get(point)
            .map(|p| p.count())
            .unwrap_or(0)
    }

    pub fn has_unit(&self, point: &Point, id: &Id) -> bool {
        self.get(point)
            .map(|p| p.contains(id))
            .unwrap_or(false)
    }

    pub fn has_units(&self, point: &Point) -> bool {
        self.count_units(point) > 0
    }

    pub fn has_selection(&self) -> bool {
        self.selected.len() > 0
    }

    pub fn select_none(&mut self) {
        self.selected.clear();
    }

    pub fn select_top(&mut self, point: &Point) {
        if let Some(unit) = self.get_top(&point) {
            self.selected = vec![Selection::new(&point,unit)];
        }
    }

    pub fn select_id(&mut self, point: &Point, id: Id) {
        if let Some(unit) = self.get_id(point,&id) {
            self.selected.push(Selection::new(&point,unit));
        }
    }

    pub fn select_ids(&mut self, point: &Point, ids: Vec<Id>) {
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

    /// move selection to a new position
    pub fn moveto(&mut self, point: &Point) -> Result<()> {
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
            .map(|mut u| {
                u.set_position(point);
                u
            })
            .collect::<Vec<Unit>>();

        // give units to new location
        if let Some(target) = self.get_mut(point) {
            target.give_all(units);
        }

        // update selection to the new position
        for selection in self.selected.iter_mut() {
            selection.update(point);
        }

        Ok(())
    }

    pub fn update(&mut self, map: &mut Tilemap, point: &Point) -> Result<()> {

        let moved: Vec<(usize,usize)> = self
            .selected
            .iter()
            .filter_map(|s| self
                .get_unit(s)
                .map(|u| (s.index,*u.layer())))
            .collect();

        self.moveto(point)?;

        // get points for positions left empty
        let points: Vec<((i32,i32),usize)> = moved
            .into_iter()
            .filter(|(i,l)| self.is_empty(i))
            .map(Point::tuple_index)
            .collect();

        // remove empty tiles after move
        if let Err(e) = map.clear_tiles(points) {
            log::warn!("{:?}", e);
        }

        // get tiles for new unit positions
        let mut after: Vec<Tile<_>> = self
            .selected
            .iter()
            .filter_map(|s| self.get_unit(s))
            .map(|u| u.as_tile())
            .collect();
        
        // remove duplicate tiles
        after.sort_by(|a,b| a.point.cmp(&b.point));
        after.dedup();

        // insert tiles after unit move
        if let Err(e) = map.insert_tiles(after) {
            log::warn!("{:?}", e);
        }


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
            println!("-- adding unit at {:?}",point);
            target.give(unit);
        }
    }

    fn get(&self, point: &Point) -> Option<&Position> {
        self.get_idx(point.as_index() as usize)
    }

    fn get_mut(&mut self, point: &Point) -> Option<&mut Position> {
        self.get_idx_mut(point.as_index() as usize)
    }

    fn get_idx(&self, index: Index) -> Option<&Position> {
        self.positions.get(index)
    }

    fn get_idx_mut(&mut self, index: Index) -> Option<&mut Position> {
        self.positions.get_mut(index)
    }

    fn get_unit(&self, s: &Selection) -> Option<&Unit> {
        self.get_idx(s.index)
            .map(|p| p.id(&s.id))
            .flatten()
    }

    fn get_unit_mut(&mut self, s: &Selection) -> Option<&mut Unit> {
        self.get_idx_mut(s.index)
            .map(|p| p.id_mut(&s.id))
            .flatten()
    }

    fn is_empty(&self, i: &Index) -> bool {
        self.get_idx(*i)
            .map(|p| p.count() == 0)
            .unwrap_or(false)
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

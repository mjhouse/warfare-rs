use crate::generation::{Unit,Change,ChangeType,Id,PlayerId};
use crate::objects::Point;
use crate::state::traits::HasId;
use crate::state::Context;
use crate::error::{Result,Error};
use crate::behavior::Pathfinder;
use crate::state::traits::{HasPosition,AsTile,HasLayer};

use std::collections::HashMap;
use bevy::render::color::Color;
use bevy_tilemap::Tilemap;
use bevy_tilemap::Tile;

use indexmap::IndexMap;
use log::*;

macro_rules! point {
    ($i:expr) => { Point::from_index($i as i32) }
}

/// alias for map index position
type Index = usize;

/// max units at a given position
const MAX: usize = 5;

#[derive(Default, Debug, Clone, Copy)]
pub struct Selection {
    pub id: Id,
    pub start: Index,
    pub end: Index,
    pub actions: (u8,u8),
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
            id: *unit.id(),
            start: point.as_index() as usize,
            end: point.as_index() as usize,
            actions: (
                unit.actions(), // initial actions
                unit.actions(), // current actions
            ),
        }
    }

    pub fn update(&mut self, point: &Point) {
        self.end = point.as_index() as usize;
    }

    pub fn start_point(&self) -> Point {
        Point::from_index(self.start as i32)
    }

    pub fn end_point(&self) -> Point {
        Point::from_index(self.end as i32)
    }

    pub fn current(&self) -> u8 {
        self.actions.1
    }

    pub fn unit(&self) -> Id {
        self.id.clone()
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

    /// check if the position contains enemies
    pub fn enemies(&self, id: &PlayerId) -> bool {
        self.units
            .iter()
            .any(|(_,v)| v.player_id() != id)
    }

    pub fn targeted_units(&self, id: &PlayerId) -> Vec<&Unit> {
        self.units
            .iter()
            .map(|(_,v)| v)
            .filter(|v| v.player_id() != id)
            .collect()
    }

    /// get a reference to the top unit
    pub fn top(&self) -> Option<&Unit> {
        self.units.last().map(|p| p.1)
    }

    /// get a reference to a particular unit
    pub fn id(&self, id: &Id) -> Option<&Unit> {
        self.units.get(id)
    }

    /// get a mutable reference to a particular unit
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

    pub fn selected(&self) -> Vec<Selection> {
        self.selected.clone()
    }

    pub fn selected_units(&self) -> Vec<&Unit> {
        self.selected
            .iter()
            .filter_map(|s| self.get_unit(s))
            .collect()
    }

    pub fn has_enemy(&self, point: &Point, player: &PlayerId) -> bool {
        self.get(point)
            .map(|p| p.enemies(player))
            .unwrap_or(false)
    }

    pub fn targeted_units(&self, point: &Point, player: &PlayerId) -> Vec<&Unit> {
        self.get(point)
            .map(|p| p.targeted_units(player))
            .unwrap_or(Vec::new())
    }

    pub fn find(&mut self, id: &Id) -> Option<&mut Unit> {
        self.positions
            .iter_mut()
            .map(|p| p
                .list_mut()
                .into_iter())
            .flatten()
            .find(|u| u.id() == id)
    }

    pub fn execute(&mut self, map: &mut Tilemap, changes: &Vec<Change>) {
        println!("------------------------------------------------------");
        println!("applying {} changes",changes.len());
        let mut remove: Vec<(Id,Point)> = vec![];
        for change in changes {
            if let Some(unit) = self.find(&change.id) {
                match change.action {
                    ChangeType::Health(v) => {
                        println!("unit \"{}\" health changed: {}",unit.name(), v);
                        unit.set_health(v)
                    },
                    _ => ()
                };
                if unit.health() == 0 {
                    println!("unit \"{}\" destroyed",unit.name());
                    remove.push((
                        *unit.id(),
                        *unit.position(),
                    ))
                }
            }
        }

        println!("selecting tiles");
        let tiles: Vec<_> = remove
            .into_iter()
            .filter_map(|(i,p)| self
                .get_mut(&p)
                .map(|o| o.take(i))
                .flatten()
                .map(|u| (p,u)))
            .map(|(p,u)| (p.integers(),*u.layer()))
            .collect();

        println!("{} tiles selected",tiles.len());
        // clear the tile locations
        if let Err(e) = map.clear_tiles(tiles) {
            log::warn!("{:?}", e);
        }

        println!("------------------------------------------------------");
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
        let selected = self.selected.clone();
        for s in selected.iter() {
            if s.start != s.end {
                if let Some(unit) = self.get_unit_mut(s) {
                    let used = s.actions.0.saturating_sub(s.actions.1);
                    unit.use_actions(used);
                }
            }
        }
        self.selected.clear();
    }

    pub fn select_top(&mut self, owner: PlayerId, point: &Point) {
        if let Some(unit) = self.get_top(&point) {
            if unit.player_id() == &owner {
                self.selected = vec![Selection::new(&point,unit)];
            }
        }
    }

    pub fn select_all(&mut self, point: &Point) {
        self.selected
            .append(&mut self
                .get_all(&point)
                .iter()
                .map(|u| Selection::new(&point,u))
                .collect());
        log::info!("selected: {}",self.selected.len());
    }

    pub fn select_id(&mut self, point: &Point, id: Id) {
        if let Some(unit) = self.get_id(point,&id) {
            self.selected.push(Selection::new(&point,unit));
        }
    }

    pub fn select(&mut self, ids: &Vec<Id>) {
        self.selected = self
            .get_all_ids(ids)
            .iter()
            .map(|u| Selection::new(u.position(),u))
            .collect()
    }

    pub fn select_ids(&mut self, owner: &PlayerId, point: &Point, ids: Vec<Id>) {
        self.selected
            .append(&mut self
                .get_ids(&point,&ids)
                .iter()
                .filter(|u| u.player_id() == owner)
                .map(|u| Selection::new(&point,u))
                .collect());
    }

    pub fn transfer(&mut self, map: &mut Tilemap, moves: Vec<(Id,u8)>, point: Point) {
        let ids = moves
            .iter()
            .map(|m| m.0)
            .collect();

        self.select(&ids);
        self.move_selection(map,&point);

        let zipped = self
            .selected
            .iter_mut()
            .zip(moves
                .iter()
                .map(|m| m.1));

        for (selection,current) in zipped {
            selection.actions.1 = current;
        }

        self.select_none();
    }

    pub fn select_return(&mut self, map: &mut Tilemap) {
        let mut previous = self.selected.clone();
        self.hide(map, &self.selected);

        // move to each starting position and leave the
        // unit(s) that started there.
        for p in previous.iter_mut() {
            self.moveto(&Point::from_index(p.start as i32));
            self.selected.retain(|s| s.start != p.start);
            p.end = p.start;
        }

        self.show(map, &previous);
    }

    pub fn move_selection(&mut self, map: &mut Tilemap, point: &Point) {
        warn!("moving to {:?}",point);
        self.hide(map, &self.selected);
        self.moveto(point);
        self.show(map, &self.selected);
    }

    pub fn get_top(&self, point: &Point) -> Option<&Unit> {
        self.get(point)
            .map(|p| p.top())
            .flatten()
    }

    pub fn get_all(&self, point: &Point) -> Vec<&Unit> {
        self.get(point)
            .map(|p| p.list())
            .unwrap_or(Vec::new())
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

    pub fn get_all_ids(&self, ids: &Vec<Id>) -> Vec<&Unit> {
        self.units()
            .into_iter()
            .filter(|u| ids.contains(u.id()))
            .collect()
    }

    pub fn moveto(&mut self, point: &Point) -> Result<()> {
        self.is_valid(point)?;

        // find all selected units
        let mut units = self.selected
            .clone()
            .into_iter()
            .filter_map(|s| self
                .positions
                .get_mut(s.end)
                .map(|p| p.take(s.id))
                .flatten())
            .collect::<Vec<Unit>>();

        // update the unit to the new position
        for unit in units.iter_mut() {
            unit.set_position(point);
        }

        // update selection to the new position
        for selection in self.selected.iter_mut() {
            selection.update(point);
        }

        // give units to new location
        if let Some(target) = self.get_mut(point) {
            target.give_all(units);
        }

        Ok(())
    }

    fn space(&self, point: &Point) -> usize {
        self.get(point)
            .map(|p| p.space())
            .unwrap_or(0)
    }

    fn draw(&self, map: &mut Tilemap, points: &Vec<Point>, layer: usize, sprite: usize) {
        let tiles: Vec<Tile<_>> = points
            .iter()
            .map(|p| Tile {
                point: p.integers(),
                sprite_order: layer,
                sprite_index: sprite,
                tint: Color::rgba(1., 1., 1., 0.25),
            })
            .collect();

        if let Err(e) = map.insert_tiles(tiles) {
            log::warn!("{:?}", e);
        }
    }

    pub fn pathto(&mut self, map: &mut Tilemap, impedance: &HashMap<Point, f32>, point: &Point, layer: usize, sprite: usize) -> Vec<Point> {
        self.hide(map, &self.selected);

        // find all selected units
        let mut units = self.selected
            .clone()
            .into_iter()
            .filter_map(|s| self
                .positions
                .get_mut(s.end)
                .map(|p| p.take(s.id))
                .flatten())
            .map(|u| (*u.id(),u))
            .collect::<HashMap<Id,Unit>>();

        let mut paths: IndexMap<Id,Vec<Point>> = IndexMap::new();

        for s in self.selected.iter_mut() {
            let finder = Pathfinder::new(&impedance, point!(s.start), *point);

            // init actions to initial values
            let ( i, _ ) = s.actions;
            let mut c = i;

            let path = finder
                .find_weighted()
                .into_iter()
                .filter(|(_, n)| {
                    let cost = n.max(0.).min(100.) as u8;
                    c = c.saturating_sub(cost);
                    c > 0
                })
                .map(|(p, _)| p)
                .collect::<Vec<Point>>();
            
            s.actions.1 = c;
            paths.insert(s.id,path);
        }

        let mut i = 0;
        while i < paths.len() {
            let maybe_last = paths
                .get_index(i)
                .map(|(k,v)| v
                    .last())
                .flatten()
                .cloned();

            if let Some(last) = maybe_last {
                let mut count = self
                    .get(&last)
                    .map(|p| p.count())
                    .unwrap_or(MAX);

                count += paths
                    .iter()
                    .filter(|(_,p)| p
                        .last() == Some(&last))
                    .count();

                // check if point is overcrowded
                if count > MAX {

                    let maybe_path = paths
                        .get_index_mut(i)
                        .map(|(k,v)| v);

                    if let Some(path) = maybe_path {
                        // remove last point from path
                        path.retain(|p| p != &last);

                        // skip incrementing 'i'
                        continue;
                    }
                }
            }

            i += 1;
        }


        for s in self.selected.iter_mut() {
            if let Some(unit) = units.get_mut(&s.id) {
                if let Some(path) = paths.get_mut(&s.id) {
                    if let Some(last) = path.last().cloned() {
                        // get all but the last position
                        path.retain(|&p| p != last);
                        
                        // update the selection to the last valid point
                        s.end = last.to_index() as usize;
        
                        // update the unit location to the last point
                        unit.set_position(&last);
                    }
                }
            }
        }

        // move units to the new position
        for (_,unit) in units.into_iter() {
            match self.get_mut(unit.position()) {
                Some(pos) => pos.give(unit),
                None => panic!("Unit placed on nonexistant tile"),
            };
        }

        self.show(map, &self.selected);

        let points = paths
            .into_iter()
            .map(|(i,p)| p)
            .map(Vec::into_iter)
            .flatten()
            .collect();

        self.draw(map,&points,layer,sprite);

        points
    }

    pub fn get_units(&self, point: &Point) -> Vec<&Unit> {
        self.get(point)
            .map(|p| p.list())
            .unwrap_or(Vec::new())
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
        self.get_idx(s.end)
            .map(|p| p.id(&s.id))
            .flatten()
    }

    fn get_unit_mut(&mut self, s: &Selection) -> Option<&mut Unit> {
        self.get_idx_mut(s.end)
            .map(|p| p.id_mut(&s.id))
            .flatten()
    }

    fn is_empty(&self, i: &Index) -> bool {
        self.get_idx(*i)
            .map(|p| p.count() == 0)
            .unwrap_or(false)
    }

    fn is_valid(&self, point: &Point) -> Result<()> {
        // get units to move
        let count = self.count();

        // get space at target position
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

        Ok(())
    }

    fn hide(&self, map: &mut Tilemap, units: &Vec<Selection>) {
        let tiles: Vec<_> = units
            .iter()
            .filter_map(|s| self
                .get_idx(s.end)
                .map(|p| (s,p)))
            .map(|(s,p)| ( s, units
                .iter()
                .filter(|u| u.end == s.end)
                .fold(p.count(),|a,_| a
                    .saturating_sub(1))))
            .filter(|&(_,c)| c == 0)
            .filter_map(|(s,_)| self
                .get_unit(s)
                .map(|u| (s.end,*u.layer())))
            .map(Point::tuple_index)
            .collect();

        // clear the tile locations
        if let Err(e) = map.clear_tiles(tiles) {
            log::warn!("{:?}", e);
        }
    }

    fn show(&self, map: &mut Tilemap, units: &Vec<Selection>) {
        // get tile positions to insert unit graphics
        let mut tiles: Vec<Tile<_>> = units
            .iter()
            .filter_map(|s| self
                .get_unit(s))
            .map(|u| u
                .as_tile())
            .collect();
        
        // remove duplicate locations so that we don't insert
        // multiple overwritten graphics
        tiles.sort_by(|a,b| a.point.cmp(&b.point));
        tiles.dedup();

        // insert tiles at locations
        if let Err(e) = map.insert_tiles(tiles) {
            log::warn!("{:?}", e);
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

use crate::generation::Unit;
use crate::objects::Point;
use crate::generation::id::Id;
use crate::state::traits::HasId;
use hashbrown::HashMap;

/// max units at a given position
const MAX: usize = 5;

#[derive(Debug,Clone)]
pub struct Position {
    units: HashMap<Id,Unit>,
    ids:   Vec<Id>
}

pub struct Map {
    positions: Vec<Position>,
}

impl Position {
    pub fn new() -> Self {
        Self { 
            units: HashMap::with_capacity(MAX), 
            ids: Vec::with_capacity(MAX),
        }
    }

    pub fn full(&self) -> bool {
        self.units.len() >= MAX
    }

    pub fn list(&self) -> Vec<&Unit> {
        let mut units = self.units
            .iter()
            .map(|(_,v)| v)
            .collect::<Vec<&Unit>>();
            
        units.sort_by(|a, b| {
            let bd = self.depth(b.id());
            let ad = self.depth(a.id());
            bd.cmp(&ad)
        });
        
        units
    }

    pub fn depth(&self, id: &Id) -> usize {
        let max = self.ids.len();
        max.saturating_sub(self.ids
            .iter()
            .position(|r| r == id)
            .unwrap_or(max))
    }

    pub fn take(&mut self, id: &Id) -> Option<Unit> {
        self.ids.retain(|i| i != id);
        self.units.remove(id)
    }

    pub fn push(&mut self, unit: Unit) {
        if self.full() { return; }
        let id = unit.id().clone();
        self.units.insert(id,unit);
        self.ids.push(id);
    }

    pub fn pop(&mut self) -> Option<Unit> {
        let id = self.ids.pop()?;
        self.units.remove(&id)
    }
}

impl Map {
    pub fn new(size: usize) -> Self {
        Self { 
            positions: vec![Position::new(); size]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_succeeds() {

    }
}
use std::collections::HashSet;

/// Actions for systems to perform
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Action {
    UpdateTerrain,
    UpdateOverlay,
    PlaceUnit,
}

#[derive(Default, Clone)]
pub struct Events {
    events: HashSet<Action>,
}

impl Events {
    pub fn send(&mut self, action: Action) -> bool {
        self.events.insert(action)
    }

    pub fn receive(&self, action: Action) -> bool {
        self.events.contains(&action)
    }

    pub fn clear(&mut self, action: Action) -> bool {
        self.events.remove(&action)
    }
}

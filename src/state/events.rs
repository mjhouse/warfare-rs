use std::collections::HashMap;

/// The target system. There should be one 
/// flag for each system in the game.
#[derive(Clone,Eq,PartialEq,Hash)]
pub enum Target {
    Camera,
    Icon,
    Overlay,
    Selection,
    Generate,
}

/// Action for target system to perform
#[derive(Clone)]
pub enum Action {
    Update,
}

#[derive(Default,Clone)]
pub struct Events {
    events: HashMap<Target,Vec<Action>>,
}

impl Events {

    pub fn send(&mut self, target: Target, action: Action) {
        if let Some(e) = self.events.get_mut(&target) {
            // if a queue of events already exists for target,
            // add action to queue.
            e.push(action);
        }
        else {
            // if not, insert new queue
            self.events.insert(target,vec![ action ]);
        }
    }

    pub fn receive(&mut self, target: Target) -> Vec<Action> {
        let mut result = vec![];
        if let Some(e) = self.events.get_mut(&target) {
            // if a queue exists for the target, then drain
            // all events and return.
            result = e.drain(..).collect();
        }
        result
    }

    pub fn size(&mut self, target: Target) -> usize {
        let mut result = 0;
        if let Some(e) = self.events.get_mut(&target) {
            // get length of queue
            result = e.len();
        }
        result
    }

}
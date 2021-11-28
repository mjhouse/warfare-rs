use bevy::input::mouse::MouseButton;
use bevy_tilemap::{Tilemap,Tile};
use bevy::prelude::*;

use crate::systems::selection::Selection;
use crate::generation::{LayerUse,Unit};
use crate::state::{State,Action,traits::Moveable};

pub struct ControlPlugin;

fn control_place_system(
    mut state: ResMut<State>,
    mut map_query: Query<&mut Tilemap>,
    mut sel_query: Query<&mut Selection>,
) {
    if !state.loaded {
        return;
    }

    if state.events.receive(Action::PlaceUnit) && state.events.receive(Action::SelectionChanged) {
        let mut tilemap = map_query.single_mut().expect("Need tilemap");
        let mut selection = sel_query.single_mut().expect("Need selection");
        
        let i = state.layers
            .get(&LayerUse::Units)
            .expect("Must have selection layer");
            
        let m = state.textures.get("unit");

        if let Some(_) = state.areas.get(&selection.selected) {
            // add unit token to map
            let result = tilemap.insert_tile(Tile {
                point: selection.selected.clone(),
                sprite_order: i,
                sprite_index: m,
                tint: Color::WHITE,
            });

            if let Err(e) = result {
                log::warn!("{:?}",e);
            }

            // add unit to state
            state.units.push(Unit::new(i,m,selection.selected.clone()));
        }

        state.events.clear(Action::PlaceUnit);
        state.events.clear(Action::SelectionChanged);
    }
    else {
        // clear and wait for PlaceUnit + SelectionChanged
        state.events.clear(Action::SelectionChanged);
    }

}

fn control_movement_system(
    mut state: ResMut<State>,
    mut map_query: Query<&mut Tilemap>,
    mut sel_query: Query<&mut Selection>,
) {
    if !state.loaded {
        return;
    }

    let mut tilemap = map_query.single_mut().expect("Need tilemap");
    let mut selection = sel_query.single_mut().expect("Need selection");
    let layer = state.layers.get(&LayerUse::Units).expect("Need layer");
    let index = state.textures.get("unit");

    if let Some(old_point) = selection.unit {
        let new_point = selection.selected.clone();

        if old_point != new_point {
            if state.areas.get(&new_point).is_some() {
                if let Some(unit) = state.find_unit(&old_point) {
                    unit.moved(&mut tilemap, new_point);
                    selection.unit = Some(new_point);
                }
            }
        }
    }

}

impl Plugin for ControlPlugin {
	fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(control_place_system.system())
            .add_system(control_movement_system.system());
	}
}
use bevy::input::mouse::MouseButton;
use bevy_tilemap::{Tilemap,Tile};
use bevy::prelude::*;

use crate::systems::selection::Selection;
use crate::generation::LayerUse;
use crate::state::{State,Action};

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
            state.units.push(selection.selected.clone());
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

    if let Some(unit) = selection.unit {
        if unit != selection.selected {
            if state.areas.get(&selection.selected).is_some() {
                // remove old unit token from map
                tilemap.clear_tile(unit,layer);
                
                // add new unit token to map
                let result = tilemap.insert_tile(Tile {
                    point: selection.selected.clone(),
                    sprite_order: layer,
                    sprite_index: index,
                    tint: Color::WHITE,
                });
    
                if let Err(e) = result {
                    log::warn!("{:?}",e);
                }
    
                // update positions in state and selection
                state.units.retain(|&p| p != unit);
                state.units.push(selection.selected.clone());
                selection.unit = Some(selection.selected);
                
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
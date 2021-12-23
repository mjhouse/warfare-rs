use bevy::prelude::*;
use bevy_tilemap::{Tile, Tilemap};

use crate::generation::{LayerUse, Specialty, Unit};
use crate::state::{traits::*, Action, State};
use crate::systems::selection::Selection;

pub struct ControlPlugin;

use crate::behavior::Pathfinder;
use crate::objects::Point;

fn control_place_system(
    mut state: ResMut<State>,
    inputs: Res<Input<MouseButton>>,
    mut map_query: Query<&mut Tilemap>,
    mut sel_query: Query<&mut Selection>,
) {
    if !state.loaded {
        return;
    }

    if state.events.receive(Action::PlaceUnit) {
        let mut tilemap = map_query.single_mut().expect("Need tilemap");
        let selection = sel_query.single_mut().expect("Need selection");

        if inputs.just_pressed(selection.button) {
            if let Some(_) = state.areas.get(&selection.hovered) {
                let point: Point = selection.hovered.into();
                
                // add a unit to the map
                let unit = Unit::new()
                    .with_specialty(Specialty::Infantry)
                    .with_soldiers(100)
                    .with_position(point.clone())
                    .build(&state);
    
                unit.insert(&mut tilemap);
                state.units.add(point,unit);
            }
            state.events.clear(Action::PlaceUnit);
        }
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
    let layer = state
        .layers
        .max(&LayerUse::Selection)
        .expect("Need selection layer");

}

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(control_place_system.system())
            .add_system(control_movement_system.system());
    }
}

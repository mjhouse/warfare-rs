use bevy::input::mouse::MouseButton;
use bevy_tilemap::{Tilemap,Tile};
use bevy::prelude::*;

use crate::systems::selection::Selection;
use crate::generation::{LayerUse,Unit};
use crate::state::{State,Action,traits::Moveable};

pub struct ControlPlugin;

use crate::objects::Point;
use crate::behavior::Pathfinder;

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
    let layer = state.layers.max(&LayerUse::Selection).expect("Need selection layer");
    let index = state.textures.get("unit");
    let blank = state.textures.get("blank");

    let mut new_path: Vec<((i32,i32),usize)> = vec![];
    let mut old_path: Vec<((i32,i32),usize)> = selection.path
        .iter()
        .map(|p| (p.integers(),layer))
        .collect();

    if let Some(old_point) = selection.unit {
        let new_point = selection.selected.clone();

        if old_point != new_point {
            if state.areas.get(&new_point).is_some() {
                if let Some(unit) = state.find_unit(&old_point) {
                    unit.moved(&mut tilemap, new_point);
                    selection.unit = Some(new_point);
                }

                if let Some(start) = selection.start {
                    if let Some(current) = selection.unit {
                        let map = state.impedance_map();

                        // construct a path finder
                        let finder = Pathfinder::new(
                            map, 
                            start.into(), 
                            current.into());

                        // find the shortest path from start to current
                        let path = finder.find();
                        let n = path.len().saturating_sub(1);

                        selection.path = path.clone();

                        // update the new path for selection
                        new_path = path
                            .iter()
                            .map(|p| (p.integers(),layer))
                            .collect();
                        
                        // build the tiles to display
                        let new_tiles: Vec<Tile<(i32,i32)>> = path
                            .iter()
                            .enumerate()
                            .filter(|&(i, _)| i != n)
                            .map(|(_, p)| Tile {
                                point: p.integers(),
                                sprite_order: layer,
                                sprite_index: blank,
                                tint: Color::rgba(1.,1.,1.,0.25),
                            })
                            .collect();

                        // clear the old path and display the new path
                        tilemap.clear_tiles(old_path.clone());
                        tilemap.insert_tiles(new_tiles);
                    }
                }

            }
        }
    }

    // if the path wasn't updated (no route or done moving)
    // then clear the old path here.
    // if new_path.is_empty() && !old_path.is_empty() {
    //     tilemap.clear_tiles(old_path);
    // }
}

impl Plugin for ControlPlugin {
	fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(control_place_system.system())
            .add_system(control_movement_system.system());
	}
}
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
    mut map_query: Query<&mut Tilemap>,
    mut sel_query: Query<&mut Selection>,
) {
    if !state.loaded {
        return;
    }

    if state.events.receive(Action::PlaceUnit) && state.events.receive(Action::SelectionChanged) {
        let mut tilemap = map_query.single_mut().expect("Need tilemap");
        let selection = sel_query.single_mut().expect("Need selection");

        if let Some(_) = state.areas.get(&selection.selected) {
            let point: Point = selection.selected.into();
            
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
        state.events.clear(Action::SelectionChanged);
    } else {
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
    let layer = state
        .layers
        .max(&LayerUse::Selection)
        .expect("Need selection layer");
    let blank = state.textures.get("blank");

    if let Some(old_point) = selection.unit {
        let new_point = selection.selected.clone();

        if old_point != new_point {
            if state.areas.get(&new_point).is_some() {
                let impedance = state.impedance_map();

                let initial = state
                    .find_unit(&old_point)
                    .map(|u| u.actions())
                    .unwrap_or(0) as i32;

                let mut actions = initial;

                let start: Point = selection.start.unwrap_or(old_point).into();
                let end: Point = new_point.into();

                // find a path and shorten until actions
                // is negative (no more action points)
                let finder = Pathfinder::new(impedance, start.clone(), end.clone());

                let mut path = finder
                    .find_weighted()
                    .into_iter()
                    .filter(|(_, c)| {
                        actions -= (*c) as i32;
                        actions >= 0
                    })
                    .map(|(p, _)| p)
                    .collect::<Vec<Point>>();

                // if there is at least one point in the path,
                // then display the path and allow the unit to move.
                if let Some(end_point) = path.last().cloned() {
                    path = path.into_iter().filter(|&p| p != end_point).collect();

                    let points = selection
                        .path
                        .iter()
                        .map(|p| (p.integers(), layer))
                        .collect::<Vec<((i32, i32), usize)>>();

                    let tiles = path
                        .iter()
                        .map(|p| Tile {
                            point: p.integers(),
                            sprite_order: layer,
                            sprite_index: blank,
                            tint: Color::rgba(1., 1., 1., 0.25),
                        })
                        .collect::<Vec<Tile<(i32, i32)>>>();

                    if let Err(e) = tilemap.clear_tiles(points) {
                        log::warn!("{:?}", e);
                    }

                    if let Err(e) = tilemap.insert_tiles(tiles) {
                        log::warn!("{:?}", e);
                    }

                    selection.path = path;
                    selection.actions = Some(initial - actions.max(0));

                    if let Some(unit) = state.find_unit(&old_point) {
                        if let Err(e) = unit.moveto(&mut tilemap, end_point.clone()) {
                            log::warn!("{:?}", e);
                        }
                        selection.unit = Some(end_point.integers());
                    }
                }
            }
        }
    }
}

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(control_place_system.system())
            .add_system(control_movement_system.system());
    }
}

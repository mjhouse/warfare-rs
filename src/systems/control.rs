use bevy_tilemap::{Tilemap,Tile};
use bevy::prelude::*;

use crate::systems::selection::Selection;
use crate::generation::{LayerUse,Unit};
use crate::state::{State,Action,traits::*};

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
        let selection = sel_query.single_mut().expect("Need selection");
        
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
            state.units.push(Unit::new(i,m,selection.selected.into()));
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
    let blank = state.textures.get("blank");

    if let Some(old_point) = selection.unit {
        let new_point = selection.selected.clone();

        if old_point != new_point {
            if state.areas.get(&new_point).is_some() {
                let impedance = state
                    .impedance_map();
                
                let mut actions = state
                    .find_unit(&old_point)
                    .map(|u| u.actions)
                    .unwrap_or(0) as i32;

                let start: Point = selection.start.unwrap_or(old_point).into();
                let end: Point = new_point.into();

                let finder = Pathfinder::new(
                    impedance, 
                    start.clone(), 
                    end.clone());

                let path = finder
                    .find_weighted()
                    .into_iter()
                    .filter(|(_,c)| {
                        actions -= (*c) as i32;
                        actions >= 0
                    })
                    .map(|(p,_)| p)
                    .filter(|p| p != &end)
                    .collect::<Vec<Point>>();

                let points = selection.path
                    .iter()
                    .map(|p| (p.integers(),layer))
                    .collect::<Vec<((i32,i32),usize)>>();

                let tiles = path
                    .iter()
                    .map(|p| Tile {
                        point: p.integers(),
                        sprite_order: layer,
                        sprite_index: blank,
                        tint: Color::rgba(1.,1.,1.,0.25),
                    })
                    .collect::<Vec<Tile<(i32,i32)>>>();

                if let Err(e) = tilemap.clear_tiles(points) {
                    log::warn!("{:?}",e);
                }

                if let Err(e) = tilemap.insert_tiles(tiles) {
                    log::warn!("{:?}",e);
                }

                selection.path = path;
                selection.actions = actions;

                if let Some(unit) = state.find_unit(&old_point) {
                    unit.moveto(&mut tilemap, new_point.into());
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
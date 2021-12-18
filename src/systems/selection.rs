use bevy::input::mouse::MouseButton;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use bevy_tilemap::{Tile, Tilemap};

use crate::generation::LayerUse;
use crate::math::MidRound;
use crate::state::{traits::*, Action, State};
use crate::systems::camera::Camera;
use crate::behavior::Pathfinder;

use crate::objects::Point;
pub struct SelectionPlugin;

pub struct Selection {
    /// the position of the pointer
    pub position: Vec2,

    /// the position of the hovered tile
    pub hovered: (i32, i32),

    /// false if pointer is over ui
    pub hovering: bool,

    /// the position of the selected tile
    pub selected: (i32, i32),

    // the position we're dragging to
    pub dragging: (i32, i32),

    pub actions: Option<i32>,

    /// the path from the initial position
    pub path: Vec<Point>,

    /// the button that triggers selection
    pub button: MouseButton,

    /// the key that cancels selection
    pub release: KeyCode,

    pub interacting: bool,
}

impl Selection {
    pub fn on_selected(&self) -> bool {
        self.selected == self.hovered
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            hovered: (0, 0),
            hovering: false,
            selected: (0, 0),
            dragging: (0, 0),
            actions: None,
            path: vec![],
            button: MouseButton::Left,
            release: KeyCode::Escape,
            interacting: false,
        }
    }
}

/// Find the current position of the mouse cursor
/// when it moves and update the selection.
fn selected_position_system(
    state: ResMut<State>,
    windows: Res<Windows>,
    camera: Query<&Transform, With<Camera>>,
    mut query: Query<&mut Selection>,
) {
    if !state.loaded {
        return;
    }

    let window = windows.get_primary().unwrap();
    let mut selection = query.single_mut().expect("Need selection");

    if selection.interacting {
        return;
    }

    if !selection.hovering {
        return;
    }

    if let Some(position) = window.cursor_position() {
        let size = Vec2::new(window.width() as f32, window.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let location = (position - size / 2.0).extend(0.0).extend(1.0);

        // apply the camera transform (assume single camera)
        let camera_transform = camera.single().unwrap();
        let world_position = camera_transform.compute_matrix() * location;

        let x = world_position.x;
        let y = world_position.y;

        selection.position = Vec2::new(x, y);
    }
}

/// Find the hovered tile when the mouse cursor moves
/// and update the selection.
fn selected_hovered_system(
    state: ResMut<State>,
    windows: Res<Windows>,
    mut sel_query: Query<&mut Selection>,
    mut map_query: Query<&mut Tilemap>,
) {
    if !state.loaded {
        return;
    }

    let window = windows.get_primary().unwrap();
    let tilemap = map_query.single_mut().expect("Need tilemap");
    let mut selection = sel_query.single_mut().expect("Need selection");

    if selection.interacting {
        return;
    }

    if !selection.hovering {
        return;
    }

    if window.cursor_position().is_some() {
        // cache the position of the hovered tile
        let x = selection.position.x;
        let y = selection.position.y;

        let point = Point::from_global(x,y).integers();
        
        if state.areas.get(&point).is_some() {
            selection.hovered = point;
        }
    }
}

fn pathfind() {
    // let impedance = state.impedance_map();

    // let initial = state
    //     .find_unit(&old_point)
    //     .map(|u| u.actions())
    //     .unwrap_or(0) as i32;

    // let mut actions = initial;

    // let start: Point = selection.start.unwrap_or(old_point).into();
    // let end: Point = new_point.into();

    // // find a path and shorten until actions
    // // is negative (no more action points)
    // let finder = Pathfinder::new(impedance, start.clone(), end.clone());

    // let mut path = finder
    //     .find_weighted()
    //     .into_iter()
    //     .filter(|(_, c)| {
    //         actions -= (*c) as i32;
    //         actions >= 0
    //     })
    //     .map(|(p, _)| p)
    //     .collect::<Vec<Point>>();

    // // if there is at least one point in the path,
    // // then display the path and allow the unit to move.
    // if let Some(end_point) = path.last().cloned() {
    //     path = path.into_iter().filter(|&p| p != end_point).collect();

    //     let points = selection
    //         .path
    //         .iter()
    //         .map(|p| (p.integers(), layer))
    //         .collect::<Vec<((i32, i32), usize)>>();

    //     let tiles = path
    //         .iter()
    //         .map(|p| Tile {
    //             point: p.integers(),
    //             sprite_order: layer,
    //             sprite_index: blank,
    //             tint: Color::rgba(1., 1., 1., 0.25),
    //         })
    //         .collect::<Vec<Tile<(i32, i32)>>>();

    //     if let Err(e) = tilemap.clear_tiles(points) {
    //         log::warn!("{:?}", e);
    //     }

    //     if let Err(e) = tilemap.insert_tiles(tiles) {
    //         log::warn!("{:?}", e);
    //     }

    //     selection.path = path;
    //     selection.actions = Some(initial - actions.max(0));

    //     if let Some(unit) = state.find_unit(&old_point) {
    //         if let Err(e) = unit.moveto(&mut tilemap, end_point.clone()) {
    //             log::warn!("{:?}", e);
    //         }
    //         selection.unit = Some(end_point.integers());
    //     }
    // }
}

fn selected_highlight_system(
    mut state: ResMut<State>,
    windows: Res<Windows>,
    inputs: Res<Input<MouseButton>>,
    keyboard: Res<Input<KeyCode>>,
    mut sel_query: Query<&mut Selection>,
    mut map_query: Query<&mut Tilemap>,
) {
    if !state.loaded {
        return;
    }

    let window = windows.get_primary().unwrap();
    let mut selection = sel_query.single_mut().expect("Need selection");
    let mut map = map_query.single_mut().expect("Need tilemap");

    let layer = state
        .layers
        .max(&LayerUse::Selection)
        .expect("Need selection layer");

    let blank = state.textures.get("blank");

    if selection.interacting {
        return;
    }

    if !selection.hovering {
        return;
    }

    if keyboard.just_pressed(selection.release) && state.units.has_selection() {
        state.units.select_return(&mut map);
    }

    if window.cursor_position().is_some() {
        // if the selection button has just been pressed, then select the
        // top unit at the position
        if inputs.just_pressed(selection.button) {
            selection.selected = selection.hovered;
            state.units.select_top(&selection.selected.into());
        }
        // if the selection button has just been released, then deselect
        // whatever units are selected
        else if inputs.just_released(selection.button) {
            state.units.select_none();

            // clear out any old path graphics
            if !selection.path.is_empty() {
                let points = selection
                    .path
                    .iter()
                    .map(|p| (p.integers(), layer))
                    .collect::<Vec<((i32, i32), usize)>>();

                if let Err(e) = map.clear_tiles(points) {
                    log::warn!("{:?}", e);
                }
            }
        }
        // if the button is pressed (but not just-pressed) and the selection
        // location is hovering over a new tile, then trigger dragging
        else if inputs.pressed(selection.button)
        {
            if selection.dragging != selection.hovered && state.units.has_selection() {
                if let Err(e) = state.units.update(&mut map,&selection.hovered.into()) {
                    log::warn!("{:?}", e);
                }

                let impedance = state.impedance_map();

                // clear out any old path graphics
                if !selection.path.is_empty() {
                    let points = selection
                        .path
                        .iter()
                        .map(|p| (p.integers(), layer))
                        .collect::<Vec<((i32, i32), usize)>>();

                    if let Err(e) = map.clear_tiles(points) {
                        log::warn!("{:?}", e);
                    }
                }

                for (id,start,end) in state.units.routes().into_iter() {

                    let finder = Pathfinder::new(&impedance, start, end);
                    
                    let mut path = finder
                        .find_weighted()
                        .into_iter()
                        .map(|(p, _)| p)
                        .collect::<Vec<Point>>();

                    // getting last also checks that there is at least one
                    // position in the path
                    if let Some(last) = path.last().cloned() {
                        
                        // get all but the last position
                        path = path.into_iter().filter(|&p| p != last).collect();
                        
                        // build path tiles
                        let tiles = path
                            .iter()
                            .map(|p| Tile {
                                point: p.integers(),
                                sprite_order: layer,
                                sprite_index: blank,
                                tint: Color::rgba(1., 1., 1., 0.25),
                            })
                            .collect::<Vec<Tile<(i32, i32)>>>();

                        // insert partially tranparent tiles for path
                        // into the tile map
                        if let Err(e) = map.insert_tiles(tiles) {
                            log::warn!("{:?}", e);
                        }

                        selection.path = path;
                    }
                }
            }
        }

        selection.dragging = selection.hovered;

        // move cursor to new location
        if let Some(area) = state.areas.get(&selection.selected) {
            state.terrain.selected = area.clone();
            if let Err(e) = state.cursor.moveto(&mut map,selection.selected.into()) {
                log::warn!("{:?}", e);
            }
        }
    }

}

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(selected_position_system.system()); // get world position of pointer
        app.add_system(selected_hovered_system.system()); // convert world position to hovered tile
        app.add_system(selected_highlight_system.system()); // highlight hovered tile on click
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn().insert(Selection::default());
}

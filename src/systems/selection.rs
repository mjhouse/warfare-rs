use bevy::input::mouse::MouseButton;
use bevy::prelude::*;
use bevy_tilemap::Tilemap;

use crate::generation::LayerUse;
use crate::math::MidRound;
use crate::state::{traits::*, Action, State};
use crate::systems::camera::Camera;

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

    /// the position of the active unit
    pub unit: Option<(i32, i32)>,

    pub start: Option<(i32, i32)>,

    pub actions: Option<i32>,

    /// the path from the initial position
    pub path: Vec<Point>,

    /// the button that triggers selection
    pub button: MouseButton,

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
            unit: None,
            start: None,
            actions: None,
            path: vec![],
            button: MouseButton::Left,
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
        selection.hovered =
            Point::from_global(selection.position.x, selection.position.y).integers();
    }
}

fn selected_highlight_system(
    mut state: ResMut<State>,
    windows: Res<Windows>,
    inputs: Res<Input<MouseButton>>,
    mut sel_query: Query<&mut Selection>,
    mut map_query: Query<&mut Tilemap>,
) {
    if !state.loaded {
        return;
    }

    let window = windows.get_primary().unwrap();
    let mut selection = sel_query.single_mut().expect("Need selection");
    let mut map = map_query.single_mut().expect("Need tilemap");

    if selection.interacting {
        return;
    }

    if !selection.hovering {
        return;
    }

    // update the selected tile and move the marker
    if window.cursor_position().is_some() {
        if inputs.pressed(selection.button) && !selection.on_selected() {
            selection.selected = selection.hovered;
            if let Some(area) = state.areas.get(&selection.selected) {
                state.terrain.selected = area.clone();
                if let Err(e) = state.cursor.moveto(&mut map, selection.selected.into()) {
                    log::warn!("{:?}", e);
                }
                state.events.send(Action::SelectionChanged);
            }
        }
    }

    // if player is dragging a selected unit, update it
    let mut clear = true;
    if window.cursor_position().is_some() {
        if inputs.pressed(selection.button) {
            if state.has_unit(&selection.selected) {
                if selection.unit.is_none() {
                    selection.unit = Some(selection.selected);
                    selection.start = Some(selection.selected);
                }
            }
            clear = false;
        }
    }

    if clear {
        if let Some(point) = selection.unit {
            if let Some(_) = selection.start {
                if !selection.path.is_empty() {
                    let layer = state.layers.max(&LayerUse::Selection).unwrap();
                    let path: Vec<((i32, i32), usize)> = selection
                        .path
                        .iter()
                        .map(|p| (p.integers(), layer))
                        .collect();
                    if let Err(e) = map.clear_tiles(path) {
                        log::warn!("{:?}", e);
                    }
                }
                selection.start = None;
            }
            if let Some(unit) = state.find_unit(&point) {
                if let Some(mut actions) = selection.actions {
                    unit.use_actions(actions.max(0).min(255) as u8);
                }
                selection.actions = None;
            }
            selection.unit = None;
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

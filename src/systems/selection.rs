use bevy::input::mouse::MouseButton;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use bevy_tilemap::{Tile, Tilemap};
use std::collections::HashSet;
use crate::generation::{LayerUse, Specialty, Unit};
use crate::math::MidRound;
use crate::state::{traits::*, Action, State};
use crate::systems::camera::Camera;
use crate::behavior::Pathfinder;
use crate::generation::id::Id;
use crate::resources::Label;
use crate::systems::network::NetworkState;
use crate::networking::messages::*;
use crate::state::Flags;
use crate::objects::Point;

pub struct SelectionPlugin;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum SelectionFlag {
    Place,
    Hovering,
}

#[derive(Debug, Clone)]
pub struct PlaceRequest {
    pub name: String,
    pub specialty: Specialty,
}

pub struct Selection {
    pub flags: Flags<SelectionFlag>,

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

    /// the path from the initial position
    pub path: Vec<Point>,

    /// the button that triggers selection
    pub button: MouseButton,

    /// the key that cancels selection
    pub release: KeyCode,

    /// ids of multiple selected units
    pub units: HashSet<Id>,

    /// name to give to the next placed unit
    pub request: Option<PlaceRequest>,
}

impl Selection {
    pub fn clear_path(&mut self, map: &mut Tilemap, layer: usize) {
        // clear out any old path graphics
        if !self.path.is_empty() {
            let points = self
                .path
                .iter()
                .map(|p| (p.integers(), layer))
                .collect::<Vec<((i32, i32), usize)>>();

            if let Err(e) = map.clear_tiles(points) {
                log::warn!("{:?}", e);
            }
        }
    }

    pub fn add(&mut self, unit: &Unit) -> bool {
        self.units.insert(*unit.id())
    }

    pub fn has(&self, unit: &Unit) -> bool {
        self.units.contains(unit.id())
    }

    pub fn remove(&mut self, unit: &Unit) -> bool {
        self.units.remove(unit.id())
    }

    pub fn place_requested(&self) -> bool {
        self.flags.get(SelectionFlag::Place)
    }

    pub fn place_request(&mut self, name: String, specialty: Specialty) -> bool {
        self.request = Some(PlaceRequest { name, specialty });
        self.flags.set(SelectionFlag::Place)
    }

    pub fn clear_flags(&mut self) {
        self.flags.clear();
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self {
            flags: Flags::new(),
            position: Vec2::ZERO,
            hovered: (0, 0),
            hovering: false,
            selected: (0, 0),
            dragging: (0, 0),
            path: vec![],
            button: MouseButton::Left,
            release: KeyCode::Escape,
            units: HashSet::new(),
            request: None,
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
    if !state.is_loaded() {
        return;
    }

    let window = windows.get_primary().unwrap();
    let mut selection = query.single_mut().expect("Need selection");

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
    if !state.is_loaded() {
        return;
    }

    let window = windows.get_primary().unwrap();
    let tilemap = map_query.single_mut().expect("Need tilemap");
    let mut selection = sel_query.single_mut().expect("Need selection");

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

fn selected_highlight_system(
    mut state: ResMut<State>,
    windows: Res<Windows>,
    inputs: Res<Input<MouseButton>>,
    keyboard: Res<Input<KeyCode>>,
    mut network: ResMut<NetworkState>,
    mut sel_query: Query<&mut Selection>,
    mut map_query: Query<&mut Tilemap>,
) {
    if !state.is_loaded() {
        return;
    }

    let window = windows.get_primary().unwrap();
    let mut selection = sel_query.single_mut().expect("Need selection");
    let mut map = map_query.single_mut().expect("Need tilemap");

    let layer = state
        .layers
        .max(&LayerUse::Selection)
        .expect("Need selection layer");

    let blank = state.textures.get(Label::Blank);

    if !selection.hovering {
        return;
    }

    if keyboard.just_pressed(selection.release) && state.units.has_selection() {
        state.units.select_return(&mut map);
        selection.clear_path(&mut map,layer);
    }

    if window.cursor_position().is_some() {
        // if the selection button has just been pressed, then select the
        // top unit at the position
        if inputs.just_pressed(selection.button) {
            selection.selected = selection.hovered;
            if selection.units.is_empty() {
                state.units.select_top(
                    network.id(),
                    &selection.selected.into(),
                );
            }
            else {
                state.units.select_ids(
                    network.id(),
                    &selection.selected.into(),
                    selection.units.drain().collect(),
                );
            }
        }
        // if the selection button has just been released, then deselect
        // whatever units are selected
        else if inputs.just_released(selection.button) {
            let selected = state.units.selected();
            if !selected.is_empty() {
                network.send_move_event(&state.units.selected());
            }
            state.units.select_none();
            selection.clear_path(&mut map,layer);
            selection.units.clear();
        }
        // if the button is pressed (but not just-pressed) and the selection
        // location is hovering over a new tile, then trigger dragging
        else if inputs.pressed(selection.button)
        {
            if selection.dragging != selection.hovered && state.units.has_selection() {
                let impedance = state.impedance_map();
                
                selection.clear_path(&mut map,layer);
                selection.path = state.units.pathto(
                    &mut map,
                    &impedance,
                    &selection.hovered.into(),
                    layer,
                    blank
                );
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

/// React to placement requests and create a new unit.
fn selected_place_system(
    mut state: ResMut<State>,
    mut network: ResMut<NetworkState>,
    inputs: Res<Input<MouseButton>>,
    mut map_query: Query<&mut Tilemap>,
    mut sel_query: Query<&mut Selection>,
) {
    if !state.is_loaded() {
        return;
    }

    let mut tilemap = map_query.single_mut().expect("Need tilemap");
    let mut selection = sel_query.single_mut().expect("Need selection");

    if selection.place_requested() {
        if inputs.just_pressed(selection.button) {
            if let Some(PlaceRequest { name, specialty }) = selection.request.take() {
                if let Some(_) = state.areas.get(&selection.hovered) {
                    let point: Point = selection.hovered.into();
                    
                    if let Some(data) = network.player_data() {
                        // add a unit to the map
                        let unit = Unit::new(network.id())
                            .with_name(name)
                            .with_player(data)
                            .with_specialty(specialty)
                            .with_soldiers(100)
                            .with_position(point.clone())
                            .build(&state);

                        unit.insert(&mut tilemap);
                        state.units.add(point,unit.clone());
                        network.send_create_event(unit);
                    }
                }
            }
            selection.clear_flags();
        }
    }
}

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(selected_position_system.system())
           .add_system(selected_hovered_system.system())
           .add_system(selected_highlight_system.system())
           .add_system(selected_place_system.system());
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn().insert(Selection::default());
}

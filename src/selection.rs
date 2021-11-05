use bevy::input::mouse::{MouseButton};
use bevy_tilemap::{Tilemap,Tile};
use bevy::prelude::*;

use crate::state::State;
use crate::camera::Camera;
use crate::math::MidRound;

pub struct SelectionPlugin;

pub struct Selection {
    /// the position of the pointer
    pub position: Vec2,
    /// the position of the hovered tile
    pub hovered: (i32,i32),
    /// the position of the selected tile
    pub selected: (i32,i32),
    /// the button that triggers selection
	pub button: MouseButton,
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
            hovered: (0,0),
            selected: (0,0),
            button: MouseButton::Left,
		}
	}
}

fn to_tile_coords(w: f32, h: f32, ox: f32, oy: f32) -> (i32,i32) {
    let mut m = 0;
    let mut n = 0;

    let mut x = ox;
    let mut y = oy;

    let k = 0.75 * h;

    x -= w * 0.25;
    y -= h * 0.25 + h * 0.125;

    if y > 0.0 {
        y += k/2.0;
    }
    else if y < 0.0 {
        y -= k/2.0;
    }

    n = (y / k).mid() as i32;
    let odd = n.abs() % 2 == 1;

    if x > 0.0 {
        x += w/2.0;
    }
    else if x < 0.0 {
        x -= w/2.0;
    }

    if odd {
        x -= w/2.0;
    }

    m = (x / w).mid() as i32;

    let c = h*0.25;
    let g = c/(w*0.5);

    let mut ry = oy - (n as f32 * k);
    let mut rx = ox + (w/4.0) - (m as f32 * w);

    if odd {
        rx -= w/2.0;
    }

    let c = h * 0.25;

    let slope = c / (w * 0.5);
    let int1 = k - c;
    let int2 = k + c;

    if ry > (slope * rx) + int1 {
        n += 1;
        if !odd {
            m -= 1;
        }
    }
    else if ry > (-slope * rx) + int2 {
        n += 1;
        if odd {
            m += 1;
        }
    }

    (m,n)
}

fn from_tile_coords(x: i32, y: i32) -> (f32,f32) {
    (0.0,0.0)
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

        selection.position = Vec2::new(x,y);
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
    let mut selection = sel_query.single_mut().expect("Need selection");
    let mut tilemap = map_query.single_mut().expect("Need tilemap");

    if window.cursor_position().is_some() {
        let tile_width = tilemap.tile_width() as i32;
        let tile_height = tilemap.tile_height() as i32;

        // cache the position of the hovered tile
        selection.hovered = to_tile_coords(
            tile_width as f32,
            tile_height as f32,
            selection.position.x,
            selection.position.y
        );
    }
    else {

    }
}

fn selected_highlight_system(
    state: ResMut<State>,
    windows: Res<Windows>,
    inputs: Res<Input<MouseButton>>,
	mut sel_query: Query<&mut Selection>,    
    mut map_query: Query<&mut Tilemap>,
    mut dbg_query: Query<&mut Text, With<crate::DiagText>>,
) {
    if !state.loaded {
        return;
    }

    let window = windows.get_primary().unwrap();
    let mut selection = sel_query.single_mut().expect("Need selection");
    let mut tilemap = map_query.single_mut().expect("Need tilemap");
    let mut display = dbg_query.single_mut().expect("Need diagnostics display");

    // move the cursor shape to the cursor
    if window.cursor_position().is_some() {
        if inputs.pressed(selection.button) && !selection.on_selected() {
            tilemap.clear_tile(selection.selected,1);
            selection.selected = selection.hovered;
            if let Some(area) = state.areas.get(&selection.selected) {
                display.sections[1].value = format!("id={}, pos={:?},\n",area.id(),area.location());
                display.sections[3].value = format!("{},\n",area.texture());
                display.sections[5].value = format!("{},\n",area.biome());
                display.sections[7].value = format!("{},\n",area.soil());
                display.sections[9].value = format!("{}m,\n",area.elevation());
                display.sections[11].value = format!("{}C,\n",area.temperature());
                display.sections[13].value = format!("{}%,\n",area.fertility());
                display.sections[15].value = format!("{}%,\n",area.rocks());
                display.sections[17].value = format!("{}%,\n",area.moisture());
    
                // move marker to new location
                tilemap.insert_tile(Tile {
                    point: selection.selected,
                    sprite_order: 1,
                    sprite_index: state.mark,
                    tint: Color::WHITE,
                });
            }
        }
    }
}

impl Plugin for SelectionPlugin {
	fn build(&self, app: &mut AppBuilder) {
        app.add_system(selected_position_system.system());  // get world position of pointer
        app.add_system(selected_hovered_system.system());   // convert world position to hovered tile
        app.add_system(selected_highlight_system.system()); // highlight hovered tile on click
	}
}

pub fn setup(mut commands: Commands) {
	commands
		.spawn()
		.insert(Selection::default());
}
use bevy::input::mouse::{MouseButton};
use bevy_tilemap::{Tilemap,Tile};
use bevy::prelude::*;
use bevy_tilemap::prelude::*;

use crate::state::GameState;
use crate::camera::Camera;

pub struct SelectionPlugin;

const HEX_ODD_OFFSET_X: f32 = -43.75;
const HEX_ODD_OFFSET_Y: f32 = -50.00;

const HEX_EVEN_OFFSET_X: f32 = 131.25;
const HEX_EVEN_OFFSET_Y: f32 = 050.00;

pub struct Selection {
    pub spawned: bool,
    /// the position of the pointer
    pub position: Vec2,
    /// the position of the highlight
    pub marker: (i32,i32),
    /// the button that triggers selection
	pub button: MouseButton,
}

impl Default for Selection {
	fn default() -> Self {
		Self {
            spawned: true,
            position: Vec2::ZERO,
            marker: (0,0),
            button: MouseButton::Left,
		}
	}
}

fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x2-x1).powf(2.0) + (y2-y1).powf(2.0)).sqrt()
}

fn roundsin(value: f32) -> f32 {
    match value {
        v if v > 0.0 => v.floor(),
        v if v < 0.0 => v.ceil(),
        v => v,
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

    n = roundsin(y / k) as i32;
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

    m = roundsin(x / w) as i32;

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

fn dbg_tile(map: &mut Tilemap, x: i32, y: i32, i: usize) {
    let width = (map.width().unwrap() * map.chunk_width()) as i32;
    let height = (map.height().unwrap() * map.chunk_height()) as i32;
    
    let min_x = -width / 2;
    let max_x = width / 2;

    let min_y = -height / 2;
    let max_y = height / 2;

    if x >= min_x && x <= max_x && y >= min_y && y <= max_y {
        if map.clear_tile((x, y), 0).is_err() {
            return;
        };
        if map.insert_tile(Tile {
            point: (x, y),
            sprite_index: i,
            sprite_order: 0,
            ..Default::default()
        }).is_err() {
            return;
        }
    }
}

fn selected_position_system(
    state: ResMut<GameState>,
    windows: Res<Windows>,
    camera: Query<&Transform, With<Camera>>,
	mut query: Query<&mut Selection>,
    
) {
    if !state.map_loaded {
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
        // dbg!((x,y));
    }
}

fn selected_marker_system(
    state: ResMut<GameState>,
    windows: Res<Windows>,
    camera: Query<&Transform, With<Camera>>,
	mut sel_query: Query<&mut Selection>,
    mut map_query: Query<&mut Tilemap>,
    
) {
    if !state.map_loaded {
        return;
    }

    let window = windows.get_primary().unwrap();
    let mut selection = sel_query.single_mut().expect("Need selection");
    let mut tilemap = map_query.single_mut().expect("Need tilemap");

    if window.cursor_position().is_some() {
        let width = (tilemap.width().unwrap() * tilemap.chunk_width()) as i32;
        let height = (tilemap.height().unwrap() * tilemap.chunk_height()) as i32;

        let tile_width = tilemap.tile_width() as i32;
        let tile_height = tilemap.tile_height() as i32;

        // set position of marker tile
        let coords = to_tile_coords(
            tile_width as f32,
            tile_height as f32,
            selection.position.x,
            selection.position.y
        );

        tilemap.clear_tile(selection.marker,2);
        tilemap.insert_tile(Tile {
            point: coords,
            sprite_order: 2,
            sprite_index: state.indices[0],
            ..Default::default()
        });

        selection.marker = coords;
    }
    else {

    }
}

// fn selected_highlight_system(
//     windows: Res<Windows>,
// 	mut sel_query: Query<&mut Selection>,    
// ) {
//     let window = windows.get_primary().unwrap();
//     let mut selection = sel_query.single_mut().expect("Need selection");

//     // move the cursor shape to the cursor
//     if window.cursor_position().is_some() {
//         let x = 

//         // transform.translation.x = selection.marker.x;
//         // transform.translation.y = selection.marker.y;
//         // dbg!(&transform.translation);
//     }
// }

impl Plugin for SelectionPlugin {
	fn build(&self, app: &mut AppBuilder) {
        app.add_system(selected_position_system.system());  // get world position of pointer
        app.add_system(selected_marker_system.system());    // convert world position to marker
        // app.add_system(selected_highlight_system.system()); // move highlight to to marker position
	}
}

pub fn setup(mut commands: Commands) {
	commands
		.spawn()
		.insert(Selection::default());
}
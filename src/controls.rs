use bevy::input::keyboard::KeyboardInput;
use bevy_tilemap::{Tilemap,Tile};
use bevy::prelude::*;

use crate::state::State;
use crate::area::Attribute;

pub struct ControlsPlugin;

#[derive(Default)]
pub struct Controls {
    pub update: bool,
}

fn overlay_system(
    mut state: ResMut<State>,
    mut keys: EventReader<KeyboardInput>,
	mut ctl_query: Query<&mut Controls>,
    mut map_query: Query<&mut Tilemap>,
) {
    if !state.loaded {
        return;
    }

    let mut controls = ctl_query.single_mut().expect("Need controls");
    let mut tilemap = map_query.single_mut().expect("Need tilemap");

    let mut key_pressed = false;

    for e in keys.iter() {
        use KeyCode::*;
        if let Some(key) = e.key_code {
            match key {
                Key0 | Escape  => {
                    state.terrain.overlay = Attribute::None;
                    key_pressed = true;
                }
                Key1  => {
                    state.terrain.overlay = Attribute::Biome;
                    key_pressed = true;
                }
                Key2  => {
                    state.terrain.overlay = Attribute::Soil;
                    key_pressed = true;
                }
                Key3  => {
                    state.terrain.overlay = Attribute::Elevation;
                    key_pressed = true;
                }
                Key4  => {
                    state.terrain.overlay = Attribute::Temperature;
                    key_pressed = true;
                }
                Key5  => {
                    state.terrain.overlay = Attribute::Fertility;
                    key_pressed = true;
                }
                Key6  => {
                    state.terrain.overlay = Attribute::Rocks;
                    key_pressed = true;
                }
                Key7  => {
                    state.terrain.overlay = Attribute::Moisture;
                    key_pressed = true;
                }
                _ => (),
            };
        }
    }

    if key_pressed || controls.update {
        let width = (tilemap.width().unwrap() * tilemap.chunk_width()) as i32;
        let height = (tilemap.height().unwrap() * tilemap.chunk_height()) as i32;

        let tint = match state.terrain.overlay {
            Attribute::Biome => Color::hex("9a10b5").unwrap(),
            Attribute::Soil => Color::hex("8a6515").unwrap(),
            Attribute::Elevation => Color::YELLOW,
            Attribute::Temperature => Color::RED,
            Attribute::Fertility => Color::GREEN,
            Attribute::Rocks => Color::hex("4d4d4d").unwrap(),
            Attribute::Moisture => Color::BLUE,
            Attribute::None => Color::WHITE,
        };

        let mut tiles = vec![];
        let mut points = vec![];

        for y in 0..height {
            for x in 0..width {
                let y = y - height / 2;
                let x = x - width / 2;

                let point = (x,y);
                let mut color = tint.clone();

                let mut i = state.get_texture_unchecked(&point);

                if state.terrain.overlay != Attribute::None {
                    let s = state.get_attribute(&point,state.terrain.overlay.clone());
                    color.set_a(s);
                    i = state.icons.blank;
                }

                tiles.push(Tile {
                    point: point,
                    sprite_order: 0,
                    sprite_index: i,
                    tint: color,
                });
                points.push((point,0));
            }
        }

        tilemap.clear_tiles(points);
        tilemap.insert_tiles(tiles);
        
        controls.update = false;
    }

}

impl Plugin for ControlsPlugin {
	fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(overlay_system.system());
	}
}

pub fn setup(mut commands: Commands) {
	commands
		.spawn()
		.insert(Controls::default());
}
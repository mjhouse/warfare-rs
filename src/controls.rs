use bevy::input::keyboard::KeyboardInput;
use bevy_tilemap::{Tilemap,Tile};
use bevy::prelude::*;

use crate::state::State;
use crate::area::Attribute;

pub struct ControlsPlugin;

pub struct Controls {
    pub overlay: Attribute,
}

impl Default for Controls {
	fn default() -> Self {
		Self {
            overlay: Attribute::None,
		}
	}
}

fn overlay_system(
    state: ResMut<State>,
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
                    controls.overlay = Attribute::None;
                    key_pressed = true;
                }
                Key1  => {
                    controls.overlay = Attribute::Biome;
                    key_pressed = true;
                }
                Key2  => {
                    controls.overlay = Attribute::Soil;
                    key_pressed = true;
                }
                Key3  => {
                    controls.overlay = Attribute::Elevation;
                    key_pressed = true;
                }
                Key4  => {
                    controls.overlay = Attribute::Temperature;
                    key_pressed = true;
                }
                Key5  => {
                    controls.overlay = Attribute::Fertility;
                    key_pressed = true;
                }
                Key6  => {
                    controls.overlay = Attribute::Rocks;
                    key_pressed = true;
                }
                Key7  => {
                    controls.overlay = Attribute::Moisture;
                    key_pressed = true;
                }
                _ => (),
            };
        }
    }

    if key_pressed {
        let width = (tilemap.width().unwrap() * tilemap.chunk_width()) as i32;
        let height = (tilemap.height().unwrap() * tilemap.chunk_height()) as i32;

        let tint = match controls.overlay {
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

                if controls.overlay != Attribute::None {
                    let s = state.get_attribute(&point,controls.overlay.clone());
                    color.set_a(s);
                    i = state.blank;
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
    }

}

fn overlay_display_system(
    state: ResMut<State>,
	mut sel_query: Query<&mut Controls>,
    mut dbg_query: Query<&mut Text, With<crate::OverText>>,
) {
    if !state.loaded {
        return;
    }

    let mut controls = sel_query.single_mut().expect("Need controls");
    let mut display = dbg_query.single_mut().expect("Need overlay display");

    let label = match controls.overlay {
        Attribute::Biome => "biome".to_string(),
        Attribute::Soil => "soil".to_string(),
        Attribute::Elevation => "elevation".to_string(),
        Attribute::Temperature => "temperature".to_string(),
        Attribute::Fertility => "fertility".to_string(),
        Attribute::Rocks => "rocks".to_string(),
        Attribute::Moisture => "moisture".to_string(),
        Attribute::None => "none".to_string(),
    };

    display.sections[1].value = format!("current: {}",label);
}

impl Plugin for ControlsPlugin {
	fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(overlay_system.system())
            .add_system(overlay_display_system.system());
	}
}

pub fn setup(mut commands: Commands) {
	commands
		.spawn()
		.insert(Controls::default());
}
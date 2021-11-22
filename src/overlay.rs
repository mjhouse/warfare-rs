use bevy::input::keyboard::KeyboardInput;
use bevy_tilemap::{Tilemap,Tile};
use bevy::prelude::*;

use crate::state::{State,LayerUse};
use crate::area::Attribute;
use crate::spectrum::Spectrum;

pub struct OverlayPlugin;

#[derive(Default)]
pub struct Overlay {
    pub update: bool,
}

fn overlay_setup_system(
    mut state: ResMut<State>,
) {
    if state.overlay.is_empty() {
        // https://hslpicker.com/#f600ff

        let biome = Spectrum::default()
            .with_start_color(250.0/360.0,1.0,0.5,1.0)
            .with_end_color(190.0/360.0,1.0,0.5,1.0)
            .finish();

        let soil = Spectrum::default()
            .with_start_color(250.0/360.0,1.0,0.5,1.0)
            .with_end_color(190.0/360.0,1.0,0.5,1.0)
            .finish();

        let elevation = Spectrum::default()
            .with_start_color(250.0/360.0,1.0,0.5,1.0)
            .with_end_color(190.0/360.0,1.0,0.5,1.0)
            .finish();

        let temperature = Spectrum::default()
            .with_start_color(180.0/360.0,0.8,0.5,1.0)
            .with_end_color(360.0/360.0,0.8,0.5,1.0)
            .finish();

        let fertility = Spectrum::default()
            .with_start_color(20.0/360.0,1.0,0.5,1.0)
            .with_end_color(120.0/360.0,1.0,0.5,1.0)
            .finish();

        let rocks = Spectrum::default()
            .with_start_color(190.0/360.0,0.5,0.5,1.0)
            .with_end_color(350.0/360.0,0.5,0.5,1.0)
            .finish();

        let moisture = Spectrum::default()
            .with_start_color(170.0/360.0,1.0,0.5,1.0)
            .with_end_color(240.0/360.0,1.0,0.5,1.0)
            .finish();

        let none = Spectrum::empty();

        state.overlay.insert(Attribute::Biome,biome);
        state.overlay.insert(Attribute::Soil,soil);
        state.overlay.insert(Attribute::Elevation,elevation);
        state.overlay.insert(Attribute::Temperature,temperature);
        state.overlay.insert(Attribute::Fertility,fertility);
        state.overlay.insert(Attribute::Rocks,rocks);
        state.overlay.insert(Attribute::Moisture,moisture);
        state.overlay.insert(Attribute::None,none);
    }
}

fn overlay_update_system(
    mut state: ResMut<State>,
    mut keys: EventReader<KeyboardInput>,
	mut ctl_query: Query<&mut Overlay>,
    mut map_query: Query<&mut Tilemap>,
) {
    if !state.loaded {
        return;
    }

    if state.overlay.is_empty() {
        return;
    }

    let mut overlay = ctl_query.single_mut().expect("Need overlay");
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

    if key_pressed || overlay.update {
        let width = (tilemap.width().unwrap() * tilemap.chunk_width()) as i32;
        let height = (tilemap.height().unwrap() * tilemap.chunk_height()) as i32;

        if let Some(spectrum) = state.overlay.get(&state.terrain.overlay) {
            let mut tiles = vec![];
            let mut points = vec![];

            let i = state.get_layer(LayerUse::Overlay);
    
            for y in 0..height {
                for x in 0..width {
                    let y = y - height / 2;
                    let x = x - width / 2;
    
                    let point = (x,y);

                    // get the attribute value
                    let feature = match state.terrain.overlay {
                        Attribute::None => 0.0,
                        _ => state.get_attribute(&point,&state.terrain.overlay),
                    };


                    // get the color from the overlay spectrum
                    let overlay = match state.terrain.overlay {
                        Attribute::None => Color::WHITE,
                        _ => spectrum.get(feature),
                    };

                    // if the overlay is none, get the real texture,
                    // otherwise get a blank one
                    let texture = match state.terrain.overlay {
                        Attribute::None => state.get_texture(&point),
                        _ => state.icons.blank,
                    };
    
                    tiles.push(Tile {
                        point: point,
                        sprite_order: i,
                        sprite_index: texture,
                        tint: overlay,
                    });

                    points.push((point,i));
                }
            }
    
            if let Err(e) = tilemap.clear_tiles(points) {
                log::warn!("{:?}",e);
            }

            if state.terrain.overlay != Attribute::None {
                if let Err(e) = tilemap.insert_tiles(tiles) {
                    log::warn!("{:?}",e);
                }
            }
            
            overlay.update = false;
        }
    }

}

impl Plugin for OverlayPlugin {
	fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(overlay_setup_system.system())
            .add_system(overlay_update_system.system());
	}
}

pub fn setup(mut commands: Commands) {
	commands
		.spawn()
		.insert(Overlay::default());
}
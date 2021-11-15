use bevy::prelude::*;
use bevy::{
    asset::LoadState,
    sprite::{TextureAtlas, TextureAtlasBuilder},
};


use bevy_tilemap::prelude::*;

use crate::area::{Area};
use crate::overlay::Overlay;
use crate::state::{State,Icons};

pub struct GeneratorPlugin;

#[derive(Default,Debug,Clone)]
pub struct Generator {
    // add map features
}

fn generate(gen: &mut crate::generator::Generator, icons: &Icons, width: i32, height: i32) -> Vec<Area> {
    let mut results = vec![];

    for y in 0..height {
        for x in 0..width {
            let y = y - height / 2;
            let x = x - width / 2;

            let location = (x,y);
            let biome = gen.biome(x,y);
            let soil = gen.soil(x,y);
            let moisture = gen.moisture(x,y);
            let rocks = gen.rockiness(x,y);
            let fertility = gen.fertility(x,y);
            let elevation = gen.elevation(x,y);
            let temperature = gen.temperature(x,y);
            let texture = icons.get(&soil);

            let area = Area::create()
                .with_texture(texture)
                .with_location(location)
                .with_biome(biome)
                .with_soil(soil)
                .with_moisture(moisture)
                .with_rocks(rocks)
                .with_fertility(fertility)
                .with_elevation(elevation)
                .with_temperature(temperature)
                .build();

            results.push(area);
        }
    }

    results
}

fn generator_initialize_system(
    mut commands: Commands,
    mut state: ResMut<State>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    asset_server: Res<AssetServer>,
) {
    if !state.resources.loaded_textures {
        let mut texture_atlas_builder = TextureAtlasBuilder::default();

        if let LoadState::Loaded =
            asset_server.get_group_load_state(state.resources.textures.iter().map(|h| h.id))
        {
            for handle in state.resources.textures.iter() {
                let texture = textures.get(handle).unwrap();
                texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), &texture);
            }
    
            let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
            let atlas_handle = texture_atlases.add(texture_atlas);
    
            let tilemap = Tilemap::builder()
                .topology(GridTopology::HexOddRows)
                .dimensions(1, 1)
                .chunk_dimensions(100, 100, 1)
                .texture_dimensions(175, 200)
                .add_layer(
                    TilemapLayer {
                        kind: LayerKind::Dense,
                        ..Default::default()
                    },
                    0,
                )
                .add_layer(
                    TilemapLayer {
                        kind: LayerKind::Sparse,
                        ..Default::default()
                    },
                    1,
                )
                .texture_atlas(atlas_handle)
                .finish()
                .unwrap();
    
            let tilemap_components = TilemapBundle {
                tilemap,
                visible: Visible {
                    is_visible: true,
                    is_transparent: true,
                },
                transform: Default::default(),
                global_transform: Default::default(),
            };
    
            commands
                .spawn()
                .insert_bundle(tilemap_components)
                .insert(Timer::from_seconds(0.075, true));
    
            state.resources.loaded_textures = true;
        }
    }
}

fn generator_configure_system(
    mut state: ResMut<State>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut map_query: Query<&mut Tilemap>,
    mut ctl_query: Query<&mut Overlay>,
) {
    if state.terrain.update {
        if let Ok(mut map) = map_query.single_mut() {

            // get texture atlas that contains loaded tile textures
            let texture_atlas = texture_atlases.get(map.texture_atlas()).unwrap();

            // insert the chunk if it's not already available
            if !map.contains_chunk((0, 0)) {
                map.insert_chunk((0, 0)).unwrap();
            }

            // calculate width and height of tile map
            let width = (map.width().unwrap() * map.chunk_width()) as i32;
            let height = (map.height().unwrap() * map.chunk_height()) as i32;

            // get icons (tile textures), the user-provided seed for the map,
            // and the user-provided factors for each tile attribute.
            let icons = Icons::from(&asset_server,&texture_atlas).unwrap();
            let seed = state.terrain.seed.parse::<u32>().unwrap_or(0);
            let factors = state.factors.clone();

            // create a map generator
            let mut generator = crate::generator::Generator::new(
                seed,
                width,
                height,
                factors);

            // generate map
            let areas = generate(&mut generator,&icons,width,height);

            // convert areas to bevy_tilemap tiles
            let tiles = areas
                .iter()
                .map(|a| a.tile())
                .collect::<Vec<Tile<_>>>();

            // update state
            state.icons = icons;
            state.add_all(areas);

            // update tilemap
            map.insert_tiles(tiles).unwrap();
            map.spawn_chunk((0, 0)).unwrap();

            // set load flags
            state.terrain.update = false;
            state.loaded = true;

            // update overlay
            if let Ok(mut c) = ctl_query.single_mut() {
                c.update = true;
            }
        }
    }
}

impl Plugin for GeneratorPlugin {
	fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(generator_initialize_system.system())
            .add_system(generator_configure_system.system());
	}
}

pub fn setup(mut commands: Commands) {
	commands
		.spawn()
		.insert(Generator::default());
}
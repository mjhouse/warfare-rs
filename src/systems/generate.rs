use bevy::prelude::*;
use bevy::{
    asset::LoadState,
    sprite::{TextureAtlas, TextureAtlasBuilder},
    render::texture::{
        SamplerDescriptor,
        FilterMode,
    },
};

use bevy_tilemap::prelude::*;

use crate::state::{State,Action,traits::{Textured,Positioned}};
use crate::resources::Textures;
use crate::generation::{Generator,LayerUse,Area};

pub struct GeneratorPlugin;

fn generate(state: &mut State, width: i32, height: i32) -> Vec<Area> {
    let gen = &mut state.generator;
    let tex = &state.textures;

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
            let impedance = gen.impedance(x,y);
            let textures = gen.textures(tex,x,y);

            let area = Area::create()
                .with_textures(textures)
                .with_location(location)
                .with_biome(biome)
                .with_soil(soil)
                .with_moisture(moisture)
                .with_rocks(rocks)
                .with_fertility(fertility)
                .with_elevation(elevation)
                .with_temperature(temperature)
                .with_impedance(impedance)
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
    assets: Res<AssetServer>,
) {
    if !state.textures.loaded {
        let mut texture_atlas_builder = TextureAtlasBuilder::default();
        let texture_ids = state.textures.handles.iter().map(|h| h.id);

        if let LoadState::Loaded = assets.get_group_load_state(texture_ids) {
            for handle in state.textures.handles.iter() {
                let texture = textures.get(handle).unwrap();
                texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), &texture);
            }
    
            let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
            let atlas_handle = texture_atlases.add(texture_atlas);
    
            let mut builder = Tilemap::builder()
                .topology(GridTopology::HexOddRows)
                .dimensions(1, 1)
                .chunk_dimensions(crate::MAP_WIDTH, crate::MAP_HEIGHT, 1)
                .texture_atlas(atlas_handle)
                .texture_dimensions(175, 200);

            for (i,(kind,_)) in state.layers.data().iter().cloned().enumerate() {
                builder = builder.add_layer(
                    TilemapLayer { kind, ..Default::default() }, i );
            }
                
            let tilemap = builder.finish().unwrap();
    
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
    
            state.textures.loaded = true;
        }
    }
}

fn generator_configure_system(
    mut state: ResMut<State>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut map_query: Query<&mut Tilemap>,
) {
    // check if update is requested for terrain generation
    if state.events.receive(Action::UpdateTerrain) {
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

            map.clear_tiles(state.tiles.clone()).unwrap();

            // get icons (tile textures), the user-provided seed for the map,
            // and the user-provided factors for each tile attribute.
            state.textures.load(&asset_server,&texture_atlas);

            let seed = state.terrain.seed.parse::<u32>().unwrap_or(0);

            let factors = state.factors.clone();
            let calendar = state.calendar.clone();

            // create a map generator
            state.generator = Generator::new(
                seed,
                width,
                height,
                factors,
                calendar);

            // generate map
            let areas = generate(&mut state,width,height);
            let max = state.layers
                .max(&LayerUse::Tilemap)
                .expect("Must have a tilemap layer");

            // convert areas to bevy_tilemap tiles
            let tiles = areas
                .iter()
                .map(|a| a.tiles(max))
                .flatten()
                .collect::<Vec<Tile<_>>>();

            state.tiles = tiles
                .iter()
                .map(|t| (
                    t.point.into(),
                    t.sprite_order,
                ))
                .collect();

            // update state
            state.add_all(areas);

            // update tilemap
            map.insert_tiles(tiles).unwrap();
            map.spawn_chunk((0, 0)).unwrap();

            // set loaded flag
            state.loaded = true;

            // update overlay
            state.events.send(Action::UpdateOverlay);
            state.events.clear(Action::UpdateTerrain);

            // update selection marker
            let t = state.textures
                .get("marker");

            let l = state.layers
                .get(&LayerUse::Selection)
                .expect("Must have selection layer");

            state.marker.set_texture(t);
            state.marker.set_layer(l);
            state.marker.place(&mut map);
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
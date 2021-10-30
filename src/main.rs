#![allow(clippy::all)]
use bevy::{
    asset::LoadState,
    sprite::{TextureAtlas, TextureAtlasBuilder},
    window::WindowMode,
};

use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

use bevy::prelude::*;
use bevy_tilemap::prelude::*;

mod math;
mod camera;
mod area;

mod selection;
mod state;

use state::State;
use area::{Area,Biome,Soil};

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 8 })
        .insert_resource(WindowDescriptor {
            title: "Warfare".to_string(),
            width: 1036.,
            height: 1036.,
            vsync: false,
            resizable: true,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .init_resource::<WarefareResources>()
        .init_resource::<State>()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapDefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin)

        // set up camera plugin/system
        .add_plugin(camera::CameraPlugin)
        .add_startup_system(camera::setup.system())

        // set up selection plugin/system
        .add_plugin(selection::SelectionPlugin)
        .add_startup_system(selection::setup.system())
        
        .add_startup_system(setup.system())
        .add_startup_system(diag.system())
        .add_system(load.system())
        .add_system(build.system())
        // .add_system(text_update.system())
        .run()
}

#[derive(Default, Clone)]
struct WarefareResources {
    textures: Vec<HandleUntyped>,
    fonts: Vec<HandleUntyped>,

    loaded_textures: bool,
    loaded_fonts: bool,
}

pub struct DiagText;

fn setup(
    mut _commands: Commands,
    mut resources: ResMut<WarefareResources>, 
    asset_server: Res<AssetServer>,
) {
    resources.textures = asset_server.load_folder("textures").unwrap();
}


fn load(
    mut commands: Commands,
    mut resources: ResMut<WarefareResources>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    asset_server: Res<AssetServer>,
) {
    if !resources.loaded_textures {
        let mut texture_atlas_builder = TextureAtlasBuilder::default();

        if let LoadState::Loaded =
            asset_server.get_group_load_state(resources.textures.iter().map(|h| h.id))
        {
            for handle in resources.textures.iter() {
                let texture = textures.get(handle).unwrap();
                texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), &texture);
            }
    
            let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
            let atlas_handle = texture_atlases.add(texture_atlas);
    
            let tilemap = Tilemap::builder()
                .topology(GridTopology::HexOddRows)
                .dimensions(1, 1)
                .chunk_dimensions(200, 200, 1)
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
    
            resources.loaded_textures = true;
        }
    }
}

fn diag(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    commands.spawn_bundle(UiCameraBundle::default());

    let style = TextStyle {
        font: assets.load("fonts/FiraSans-Regular.ttf"),
        font_size: 30.0,
        color: Color::WHITE,
    };

    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexStart,
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(10.0),
                left: Val::Px(10.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text {
            sections: vec![
                TextSection { value: "area: ".into(), style: style.clone() },
                TextSection { value: "\n".into(), style: style.clone() },

                TextSection { value: "texture: ".into(), style: style.clone() },
                TextSection { value: "\n".into(), style: style.clone() },

                TextSection { value: "biome: ".into(), style: style.clone() },
                TextSection { value: "\n".into(), style: style.clone() },

                TextSection { value: "soil: ".into(), style: style.clone() },
                TextSection { value: "\n".into(), style: style.clone() },

                TextSection { value: "elevation: ".into(), style: style.clone() },
                TextSection { value: "\n".into(), style: style.clone() },

                TextSection { value: "temp: ".into(), style: style.clone() },
                TextSection { value: "\n".into(), style: style.clone() },

                TextSection { value: "fert: ".into(), style: style.clone() },
                TextSection { value: "\n".into(), style: style.clone() },

                TextSection { value: "rocks: ".into(), style: style.clone() },
                TextSection { value: "\n".into(), style: style.clone() },

                TextSection { value: "water: ".into(), style: style.clone() },
                TextSection { value: "\n".into(), style: style.clone() },
            ],
            ..Default::default()
        },
        ..Default::default()
    }).insert(DiagText);
}


fn build(
    mut state: ResMut<State>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut query: Query<&mut Tilemap>,
) {
    if state.loaded {
        return;
    }

    for mut map in query.iter_mut() {
        map.insert_chunk((0, 0)).unwrap();

        let chunk_width = (map.width().unwrap() * map.chunk_width()) as i32;
        let chunk_height = (map.height().unwrap() * map.chunk_height()) as i32;

        // Then we need to find out what the handles were to our textures we are going to use.
        let grass_floor: Handle<Texture> = asset_server.get_handle("textures/grass.png");
        let dirt_floor: Handle<Texture> = asset_server.get_handle("textures/dirt.png");
        let water_floor: Handle<Texture> = asset_server.get_handle("textures/water.png");
        let marker_img: Handle<Texture> = asset_server.get_handle("textures/marker.png");

        let texture_atlas = texture_atlases.get(map.texture_atlas()).unwrap();
        let grass_index = texture_atlas.get_texture_index(&grass_floor).unwrap();
        let dirt_index = texture_atlas.get_texture_index(&dirt_floor).unwrap();
        let _ = texture_atlas.get_texture_index(&water_floor).unwrap();
        let _ = texture_atlas.get_texture_index(&marker_img).unwrap();

        let mut tiles = vec![];

        for y in 0..chunk_height {
            for x in 0..chunk_width {
                let y = y - chunk_height / 2;
                let x = x - chunk_width / 2;

                let area = Area::create()
                    .with_texture(grass_index)
                    .with_location((x,y))
                    .with_biome(Biome::Grassland)
                    .with_soil(Soil::Clay)
                    .with_moisture(25)
                    .with_rocks(25)
                    .with_fertility(25)
                    .with_elevation(1000.0)
                    .with_temperature(22.0)
                    .build();

                tiles.push(area.tile());
                state.add(area);
            }
        }

        // Now we pass all the tiles to our map.
        map.insert_tiles(tiles.clone()).unwrap();
        map.spawn_chunk((0, 0)).unwrap();
        state.loaded = true;
    }
}
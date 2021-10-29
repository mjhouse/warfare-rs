#![allow(clippy::all)]
use bevy::{
    asset::LoadState,
    sprite::{TextureAtlas, TextureAtlasBuilder},
    window::WindowMode,
};

use bevy::prelude::*;
use bevy_tilemap::prelude::*;

mod math;
mod camera;

mod selection;
mod state;

use state::GameState;

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
        .init_resource::<SpriteHandles>()
        .init_resource::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapDefaultPlugins)

        // set up camera plugin/system
        .add_plugin(camera::CameraPlugin)
        .add_startup_system(camera::setup.system())

        // set up selection plugin/system
        .add_plugin(selection::SelectionPlugin)
        .add_startup_system(selection::setup.system())
        
        .add_startup_system(setup.system())
        .add_system(load.system())
        .add_system(build_world.system())
        
        // .add_system(rotate_shape_system.system())
        .run()
}

#[derive(Default, Clone)]
struct SpriteHandles {
    handles: Vec<HandleUntyped>,
    atlas_loaded: bool,
}

#[derive(Default, Copy, Clone, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

// #[derive(Default)]
// struct Render {
//     sprite_index: usize,
//     z_order: usize,
// }

fn setup(
    mut _commands: Commands,
    mut tile_sprite_handles: ResMut<SpriteHandles>, 
    asset_server: Res<AssetServer>,
) {
    tile_sprite_handles.handles = asset_server.load_folder("textures").unwrap();
}


fn load(
    mut commands: Commands,
    mut sprite_handles: ResMut<SpriteHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    asset_server: Res<AssetServer>,
) {
    if sprite_handles.atlas_loaded {
        return;
    }

    // Lets load all our textures from our folder!
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        for handle in sprite_handles.handles.iter() {
            let texture = textures.get(handle).unwrap();
            texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), &texture);
        }

        let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
        let atlas_handle = texture_atlases.add(texture_atlas);

        // These are fairly advanced configurations just to quickly showcase
        // them.
        let tilemap = Tilemap::builder()
            .topology(GridTopology::HexOddRows)
            .dimensions(1, 1)
            .chunk_dimensions(32, 32, 1)
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
            .add_layer(
                TilemapLayer {
                    kind: LayerKind::Sparse,
                    ..Default::default()
                },
                2,
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

        sprite_handles.atlas_loaded = true;
    }
}


fn build_world(
    mut game_state: ResMut<GameState>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut query: Query<&mut Tilemap>,
) {
    if game_state.map_loaded {
        return;
    }

    for mut map in query.iter_mut() {
        // Since we did not `auto_chunk` in the builder, we must manually
        // insert a chunk. This will then communicate with us if we accidentally
        // insert a tile in a chunk we may not want. Also, we only expect to
        // have just 1 chunk.
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
        let water_index = texture_atlas.get_texture_index(&water_floor).unwrap();
        let marker_index = texture_atlas.get_texture_index(&marker_img).unwrap();

        game_state.indices[0] = marker_index;
        game_state.indices[1] = grass_index;
        game_state.indices[2] = water_index;
        game_state.indices[3] = dirt_index;

        game_state.tiles.push(Tile {
            point: (0, 0).into(),
            sprite_order: 2,
            sprite_index: marker_index,
            ..Default::default()
        });

        // Now we fill the entire world with grass.
        for y in 0..chunk_height {
            for x in 0..chunk_width {
                let y = y - chunk_height / 2;
                let x = x - chunk_width / 2;

                let idx = if coinflip::flip() {
                    grass_index
                }
                else {
                    dirt_index
                };


                // By default tile sets the Z order at 0. Lower means that tile
                // will render lower than others. 0 is the absolute bottom
                // level which is perfect for backgrounds.
                let tile = Tile {
                    point: (x, y).into(),
                    sprite_index: idx,
                    ..Default::default()
                };
                game_state.tiles.push(tile);
            }
        }

        // // And lets surround our world with boulders.
        // for x in 0..chunk_width {
        //     let x = x - chunk_width / 2;
        //     let tile_a = (x, -chunk_height / 2);
        //     let tile_b = (x, chunk_height / 2 - 1);
        //     tiles.push(Tile {
        //         point: tile_a,
        //         sprite_index: boulder_index,
        //         sprite_order: 1,
        //         ..Default::default()
        //     });
        //     tiles.push(Tile {
        //         point: tile_b,
        //         sprite_index: boulder_index,
        //         sprite_order: 1,
        //         ..Default::default()
        //     });
        //     game_state.collisions.insert(tile_a);
        //     game_state.collisions.insert(tile_b);
        // }

        // // Then the boulder tiles on the Y axis.
        // for y in 0..chunk_height {
        //     let y = y - chunk_height / 2;
        //     let tile_a = (-chunk_width / 2, y);
        //     let tile_b = (chunk_width / 2 - 1, y);
        //     tiles.push(Tile {
        //         point: tile_a,
        //         sprite_index: boulder_index,
        //         sprite_order: 1,
        //         ..Default::default()
        //     });
        //     tiles.push(Tile {
        //         point: tile_b,
        //         sprite_index: boulder_index,
        //         sprite_order: 1,
        //         ..Default::default()
        //     });
        //     game_state.collisions.insert(tile_a);
        //     game_state.collisions.insert(tile_b);
        // }

        // // Lets just generate some random walls to sparsely place around the
        // // world!
        // let range = (chunk_width * chunk_height) as usize / 5;
        // let mut rng = rand::thread_rng();
        // for _ in 0..range {
        //     let x = rng.gen_range((-chunk_width / 2)..(chunk_width / 2));
        //     let y = rng.gen_range((-chunk_height / 2)..(chunk_height / 2));
        //     let coord = (x, y, 0i32);
        //     if coord != (0, 0, 0) {
        //         if rng.gen_bool(0.5) {
        //             tiles.push(Tile {
        //                 point: (x, y),
        //                 sprite_index: boulder_index,
        //                 sprite_order: 1,
        //                 ..Default::default()
        //             });
        //         } else {
        //             tiles.push(Tile {
        //                 point: (x, y),
        //                 sprite_index: trees_index,
        //                 sprite_order: 1,
        //                 ..Default::default()
        //             });
        //         }
        //         game_state.collisions.insert((x, y));
        //     }
        // }

        // // Lets finally vary it up and add some dirt patches.
        // for _ in 0..range {
        //     let x = rng.gen_range((-chunk_width / 2)..(chunk_width / 2));
        //     let y = rng.gen_range((-chunk_height / 2)..(chunk_height / 2));
        //     tiles.push(Tile {
        //         point: (x, y),
        //         sprite_index: dirt_index,
        //         ..Default::default()
        //     });
        // }

        // // Now lets add in a dwarf friend!
        // let dwarf_sprite: Handle<Texture> = asset_server.get_handle("textures/hex-dwarf.png");
        // let dwarf_sprite_index = texture_atlas.get_texture_index(&dwarf_sprite).unwrap();
        // // We add in a Z order of 2 to place the tile above the background on Z
        // // order 0.
        // let dwarf_tile = Tile {
        //     point: (0, 0),
        //     sprite_index: dwarf_sprite_index,
        //     sprite_order: 2,
        //     ..Default::default()
        // };
        // tiles.push(dwarf_tile);

        // commands.spawn().insert_bundle(PlayerBundle {
        //     player: Player {},
        //     position: Position { x: 0, y: 0 },
        //     render: Render {
        //         sprite_index: dwarf_sprite_index,
        //         z_order: 2,
        //     },
        // });

        // Now we pass all the tiles to our map.
        map.insert_tiles(game_state.tiles.clone()).unwrap();

        // Finally we spawn the chunk! In actual use this should be done in a
        // spawn system.
        map.spawn_chunk((0, 0)).unwrap();

        game_state.map_loaded = true;
    }
}
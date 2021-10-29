// fn transform_tile_system(
//     mut game_state: ResMut<GameState>,
//     time: Res<Time>,
//     keyboard_input: Res<Input<KeyCode>>,
//     mut sel_query: Query<&mut Selection>,
//     mut map_query: Query<(&mut Tilemap, &mut Timer)>,
// ) {
//     if !game_state.map_loaded {
//         return;
//     }

//     let mut selection = sel_query.single_mut().expect("Need selection");

//     if let Some((x,y)) = selection.tile {
//         for (mut map, mut timer) in map_query.iter_mut() {
//             timer.tick(time.delta());
//             if !timer.finished() {
//                 continue;
//             }
    
//             for key in keyboard_input.get_pressed() {
//                 use KeyCode::*;
    
//                 let index: i32 = match key {
//                     Key0 | Numpad0 => 0,
//                     Key1 | Numpad1 => 1,
//                     Key2 | Numpad2 => 2,
//                     _ => -1,
//                 };
    
                
//                 if index >= 0 {
//                     let idx = index as usize;
    
    
//                     map.clear_tile((x, y), 0).unwrap();
        
//                     let tile = Tile {
//                         point: (x, y),
//                         sprite_index: idx,
//                         sprite_order: 0,
//                         ..Default::default()
//                     };
        
//                     map.insert_tile(tile).unwrap();
//                 }
//             }
//         }
//     }
// }


// fn selection_system(
//     wnds: Res<Windows>,
//     camera: Query<&Transform, With<Camera>>,
//     mut state: ResMut<GameState>,
//     inputs: Res<Input<MouseButton>>,
//     mut cursor: EventReader<CursorMoved>,
// 	mut sel_query: Query<&mut Selection>,
//     mut map_query: Query<(&mut Tilemap,&Transform)>,
    
// ) {
//     // get the primary window
//     let wnd = wnds.get_primary().unwrap();

//     if !state.map_loaded {
//         return;
//     }

//     let position = cursor.iter().last().map(|v| v.position);
//     let mut selection = sel_query.single_mut().expect("Need selection");
//     let (mut map, tilemap_transform) = map_query.single_mut().expect("Need tilemap");

//     if let Some(p) = position {
//         selection.position = p;
//     }

//     let chunk_width = (map.width().unwrap() * map.chunk_width()) as i32;
//     let chunk_height = (map.height().unwrap() * map.chunk_height()) as i32;

//     // check if the cursor is in the primary window
//     if let Some(pos) = wnd.cursor_position() {
//         // get the size of the window
//         let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

//         // the default orthographic projection is in pixels from the center;
//         // just undo the translation
//         let p = pos - size / 2.0;

//         // assuming there is exactly one main camera entity, so this is OK
//         let camera_transform = camera.single().unwrap();

//         // apply the camera transform
//         let mut pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
//         let mut pos_lcl = tilemap_transform.compute_matrix() * pos_wld;


//         // even rows
//         // let x = (pos_lcl.x - 131.25).round();
//         // let y = (pos_lcl.y - 050.00).round();

//         // odd rows
//         let x = (pos_lcl.x - 43.75).round();
//         let y = (pos_lcl.y - 50.00).round();

//         eprintln!("({},{})", x, y);
//     }



//     if inputs.pressed(selection.button) {
//         // select a hex square
//         let x = selection.position.x.round() as i32 - chunk_width / 2;
//         let y = selection.position.y.round() as i32 - chunk_height / 2;

//         dbg!((x,y));

//         // map.clear_tile((0, 0), 0).unwrap();

//         // map.insert_tile(Tile {
//         //     point: (0, 0),
//         //     sprite_index: 2,
//         //     sprite_order: 1,
//         //     ..Default::default()
//         // }).unwrap();

//         // map.despawn_chunk((0,0)).expect("Could not despawn chunk");
//         // map.remove_chunk((0,0)).expect("Could not remove chunk");

//         // if map.contains_chunk((0,0)) {
//         //     state.tiles[0].sprite_index = 2;

//         //     map.despawn_chunk((0,0)).expect("Could not despawn chunk");
//         //     map.remove_chunk((0,0)).expect("Could not remove chunk");
//         // }
//         // else {
//         //     map.insert_chunk((0, 0)).unwrap();
//         //     map.insert_tiles(state.tiles.clone()).unwrap();
//         //     map.spawn_chunk((0,0)).expect("Could not spawn chunk");
//         // }

//         // map.clear_tile((0, 0), 0);
//     }
// }
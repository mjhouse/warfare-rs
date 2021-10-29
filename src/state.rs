use bevy::utils::HashSet;
use bevy_tilemap::Tile;
use bevy_tilemap::point::Point3;

#[derive(Default, Clone)]
pub struct GameState {
    pub indices: [usize;4],
    pub tiles: Vec<Tile<Point3>>,
    pub map_loaded: bool,
    pub spawned: bool,
    pub collisions: HashSet<(i32, i32)>,
}

impl GameState {
    // fn try_move_player(&mut self, position: &mut Position, delta_xy: (i32, i32)) {
    //     let new_pos = (position.x + delta_xy.0, position.y + delta_xy.1);
    //     if !self.collisions.contains(&new_pos) {
    //         position.x = new_pos.0;
    //         position.y = new_pos.1;
    //     }
    // }
}
use bevy_tilemap::{Tilemap};
use bevy::prelude::Color;

use crate::generation::id;
use crate::state::traits::{Textured,Positioned,Point,Tiled};

#[derive(Clone)]
pub struct Marker {
    /// globally unique id
    id: usize,

    /// layer for this unit
    layer: usize,

    /// texture for this unit
    texture: usize,

    /// position of the unit
    position: (i32,i32),
}

impl Marker {

    pub fn new(layer: usize, texture: usize, position: (i32,i32)) -> Self {
        Self {
            id: id::get(),
            layer: layer,
            texture: texture,
            position: position,
        }
    }

    pub fn place(&self, map: &mut Tilemap) {
        let mut tile = self.as_tile();

        // clear the current tile if it exists
        map.clear_tile(
            tile.point,
            tile.sprite_order);

        // if tile could not be inserted log err
        if let Err(e) = map.insert_tile(tile) {
            log::warn!("{:?}",e);
        }
    }

}

impl Default for Marker {
    fn default() -> Self {
        Self::new(0,0,(0,0))
    }
}

crate::impl_positioned!(Marker);
crate::impl_textured!(Marker);
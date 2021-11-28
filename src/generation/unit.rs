use bevy_tilemap::{Tilemap,Tile};
use bevy::prelude::Color;

use crate::generation::id;
use crate::state::traits::{Textured,Positioned,Point};

#[derive(Clone)]
pub struct Unit {
    /// globally unique id
    id: usize,

    /// layer for this unit
    layer: usize,

    /// texture for this unit
    texture: usize,

    /// position of the unit
    position: (i32,i32),
}

impl Unit {

    pub fn new(layer: usize, texture: usize, position: (i32,i32)) -> Self {
        Self {
            id: id::get(),
            layer: layer,
            texture: texture,
            position: position,
        }
    }

}

impl Default for Unit {
    fn default() -> Self {
        Self::new(0,0,(0,0))
    }
}

crate::impl_positioned!(Unit);
crate::impl_textured!(Unit);
use bevy_tilemap::{Tilemap,Tile};
use bevy::prelude::Color;

use crate::generation::id;

type Point = (i32,i32);

pub trait Positioned {
    fn get_position(&self) -> Point;

    fn set_position(&mut self, point: Point);

    fn get_layer(&self) -> usize;

    fn set_layer(&mut self, layer: usize);
}

pub trait Textured {
    fn get_texture(&self) -> usize;

    fn set_texture(&mut self, texture: usize);
}

pub trait Moveable {
    fn moved(&mut self, map: &mut Tilemap, point: Point);
}

impl<T> Moveable for T 
where 
    T: Positioned + Textured
{
    fn moved(&mut self, map: &mut Tilemap, point: Point) {
        let p = self.get_position();
        let t = self.get_texture();
        let z = self.get_layer();

        let tile = Tile {
            point: point,
            sprite_order: z,
            sprite_index: t,
            tint: Color::WHITE,
        };

        // if tile could not be inserted, exit
        // without changing anything else
        if let Err(e) = map.insert_tile(tile) {
            log::warn!("{:?}",e);
            return;
        }

        // clear the previous tile
        map.clear_tile(p,z);
        self.set_position(point);
    }
}

#[derive(Clone)]
pub struct Unit {
    /// globally unique id
    id: usize,

    /// texture for this unit
    texture: usize,

    /// location of the unit
    location: (i32,i32),
}

impl Default for Unit {
    fn default() -> Self {
        Self::new(0,(0,0))
    }
}

impl Unit {

    pub fn new(texture: usize, location: (i32,i32)) -> Self {
        Self {
            id: id::get(),
            texture: texture,
            location: location,
        }
    }

    // pub fn move(&mut self, state: &mut State, point: (i32,i32)) -> (i32,i32) {
        

    //     state.units.retain(|&p| p.location != point);
    //     state.units.push(selection.selected.clone());
    //     selection.unit = Some(selection.selected);
    // }

}
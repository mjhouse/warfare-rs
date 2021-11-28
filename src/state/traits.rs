use bevy_tilemap::{Tilemap,Tile};
use bevy::prelude::Color;

pub type Point = (i32,i32);

#[macro_export]
macro_rules! impl_positioned {
    ( $name:ident ) => {
        impl Positioned for $name {
            fn get_position(&self) -> Point { 
                self.position.clone()
            }
        
            fn set_position(&mut self, point: Point) {
                self.position = point;
            }
        
            fn get_layer(&self) -> usize {
                self.layer.clone()
            }
        
            fn set_layer(&mut self, layer: usize) {
                self.layer = layer;
            }
        }
    }
}

#[macro_export]
macro_rules! impl_textured {
    ( $name:ident ) => {
        impl Textured for $name {
            fn get_texture(&self) -> usize {
                self.texture.clone()
            }
        
            fn set_texture(&mut self, texture: usize){
                self.texture = texture;
            }
        }
    }
}

pub trait Positioned {
    fn get_position(&self) -> Point;

    fn set_position(&mut self, point: Point);

    fn get_layer(&self) -> usize;

    fn set_layer(&mut self, layer: usize);
}

pub trait Textured {
    fn get_texture(&self) -> usize;

    fn set_texture(&mut self, texture: usize);

    // fn hide(&self, map: &mut Tilemap);

    // fn show(&self, map: &mut Tilemap);
}

pub trait Tiled {
    fn as_tile(&self) -> Tile<Point>;
}

pub trait Moveable {
    fn moved(&mut self, map: &mut Tilemap, point: Point);
}

impl<T> Tiled for T 
where 
    T: Positioned + Textured
{
    fn as_tile(&self) -> Tile<Point> {
        Tile {
            point: self.get_position(),
            sprite_order: self.get_layer(),
            sprite_index: self.get_texture(),
            tint: Color::WHITE,
        }
    }
}

impl<T> Moveable for T 
where 
    T: Positioned + Tiled
{
    fn moved(&mut self, map: &mut Tilemap, point: Point) {
        let mut tile = self.as_tile();

        let p = tile.point;
        let z = tile.sprite_order;

        // update tile position in map
        tile.point = point;

        // if tile could not be inserted, exit
        // without changing anything
        if let Err(e) = map.insert_tile(tile) {
            log::warn!("{:?}",e);
            return;
        }

        // on success, remove the old texture from 
        // the map and update the position.
        map.clear_tile(p,z);
        self.set_position(point);
    }
}
use bevy_tilemap::{Tilemap,Tile};
use bevy::prelude::Color;
use crate::objects::Point;

pub trait HasPosition {
    fn position(&self) -> &Point;

    fn position_mut(&mut self) -> &mut Point;
}

pub trait HasLayer {
    fn layer(&self) -> &usize;

    fn layer_mut(&mut self) -> &mut usize;
}

pub trait HasTexture {
    fn texture(&self) -> &usize;

    fn texture_mut(&mut self) -> &mut usize;
}

pub trait AsTile {
    fn as_tile(&self) -> Tile<(i32,i32)>;

    fn to_tile(self) -> Tile<(i32,i32)>;
}

pub trait CanMove {
    fn moveto(&mut self, map: &mut Tilemap, point: Point); // TODO: return error type here

    fn remove(&self, map: &mut Tilemap); // TODO: return error type here

    fn insert(&self, map: &mut Tilemap); // TODO: return error type here
}

impl<T> AsTile for T 
where 
    T: HasPosition + HasLayer + HasTexture
{
    fn as_tile(&self) -> Tile<(i32,i32)> {
        Tile {
            point: self.position().integers(),
            sprite_order: self.layer().clone(),
            sprite_index: self.texture().clone(),
            tint: Color::WHITE,
        }
    }

    fn to_tile(self) -> Tile<(i32,i32)> {
        (&self).as_tile()
    }
}

impl<T> CanMove for T 
where 
    T: HasPosition + HasLayer + AsTile
{
    fn moveto(&mut self, map: &mut Tilemap, point: Point) {
        self.remove(map);
        *self.position_mut() = point;
        self.insert(map);
    }

    fn remove(&self, map: &mut Tilemap) {
        let p = self.position().integers();
        let z = self.layer().clone();
        if let Err(e) = map.clear_tile(p,z) {
            log::warn!("{:?}",e);
        }
    }

    fn insert(&self, map: &mut Tilemap) {
        let tile = self.as_tile();
        if let Err(e) = map.insert_tile(tile) {
            log::warn!("{:?}",e);
        }
    }
}
use crate::generation::id;
use crate::state::traits::*;
use crate::objects::Point;

#[derive(Clone)]
pub struct Unit {
    /// globally unique id
    id: usize,

    /// layer for this unit
    layer: usize,

    /// texture for this unit
    texture: usize,

    /// position of the unit
    position: Point,

    /// how much unit can do
    pub actions: u32,
}

impl Unit {

    pub fn new(layer: usize, texture: usize, position: Point) -> Self {
        Self {
            id: id::get(),
            layer: layer,
            texture: texture,
            position: position,
            actions: 100,
        }
    }

}

impl Default for Unit {
    fn default() -> Self {
        Self::new(0,0,(0,0).into())
    }
}

impl HasPosition for Unit {
    fn position(&self) -> &Point { &self.position }

    fn position_mut(&mut self) -> &mut Point { &mut self.position }
}

impl HasLayer for Unit {
    fn layer(&self) -> &usize { &self.layer }

    fn layer_mut(&mut self) -> &mut usize { &mut self.layer }
}

impl HasTexture for Unit {
    fn texture(&self) -> &usize { &self.texture }

    fn texture_mut(&mut self) -> &mut usize { &mut self.texture }
}
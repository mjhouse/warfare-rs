use crate::generation::id;
use crate::state::traits::{HasPosition,HasLayer,HasTexture};
use crate::objects::Point;

#[derive(Clone)]
pub struct Marker {
    /// globally unique id
    id: usize,

    /// layer for this marker
    layer: usize,

    /// texture for this marker
    texture: usize,

    /// position of the marker
    position: Point,
}

impl Marker {
    pub fn new(layer: usize, texture: usize, position: Point) -> Self {
        Self {
            id: id::get(),
            layer: layer,
            texture: texture,
            position: position,
        }
    }
}

impl Default for Marker {
    fn default() -> Self {
        Self::new(0,0,(0,0).into())
    }
}

impl HasPosition for Marker {
    fn position(&self) -> &Point { &self.position }

    fn position_mut(&mut self) -> &mut Point { &mut self.position }
}

impl HasLayer for Marker {
    fn layer(&self) -> &usize { &self.layer }

    fn layer_mut(&mut self) -> &mut usize { &mut self.layer }
}

impl HasTexture for Marker {
    fn texture(&self) -> &usize { &self.texture }

    fn texture_mut(&mut self) -> &mut usize { &mut self.texture }
}
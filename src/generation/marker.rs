use crate::generation::id;
use crate::objects::Point;
use crate::state::traits::*;

#[derive(Debug, Clone)]
pub struct Marker {
    /// layer for this unit
    pub layer: usize,

    /// texture for this unit
    pub texture: usize,

    /// position of the unit
    pub position: Point,
}

/// thin marker for selection highlight
#[derive(Debug, Clone)]
pub struct Cursor {
    /// globally unique id
    pub id: usize,

    /// display information
    pub marker: Marker,
}

impl Cursor {
    pub fn new(layer: usize, texture: usize, position: Point) -> Self {
        Self {
            id: id::get(),
            marker: Marker {
                layer,
                texture,
                position,
            },
        }
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self::new(0, 0, (0, 0).into())
    }
}

impl HasMarker for Cursor {
    fn marker(&self) -> &Marker {
        &self.marker
    }

    fn marker_mut(&mut self) -> &mut Marker {
        &mut self.marker
    }
}

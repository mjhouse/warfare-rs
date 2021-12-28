// TODO: remove location
mod location;
mod map;
mod name;
mod points;

pub use location::Location;
pub use map::{Map,Selection};
pub use name::{Name, NameGenerator};
pub use points::{Axial, Cubic, Offset, Point};

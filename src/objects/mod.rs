
// TODO: remove location
mod location;
mod points;
mod name;

pub use name::{Name,NameGenerator};
pub use location::Location;
pub use points::{Point,Offset,Axial,Cubic};
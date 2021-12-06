use crate::objects::Location;

pub struct Movement {
    start: Location,
    end:   Location,
    tiles: Vec<Location>
}
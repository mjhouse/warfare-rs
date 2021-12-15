pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
#[allow(dead_code)]
pub enum Error {
    #[error("An unknown error occurred")]
    Unknown,

    #[error("Could not load texture")]
    TextureNotFound,

    #[error("No units selected")]
    NoSelection,

    #[error("The target position was not found")]
    TargetNotFound,

    #[error("The target position is too small to hold the selection")]
    TargetTooSmall,

    #[error("Tilemap operation failed")]
    TilemapError(#[from] bevy_tilemap::tilemap::TilemapError),
}

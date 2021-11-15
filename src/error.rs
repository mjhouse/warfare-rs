pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]

#[allow(dead_code)]
pub enum Error {

    #[error("An unknown error occurred")]
    Unknown,

    #[error("Could not load texture")]
    TextureNotFound,

}
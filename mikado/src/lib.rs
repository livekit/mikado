pub mod backends;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, MikadoError>;

#[derive(Error, Debug)]
pub enum MikadoError {
    #[error("Error fetching devices: {0}")]
    GeneralError(String),
}

#[derive(Debug, Clone, Copy)]
pub struct ScreenDisplay {
    pub origin: (f32, f32),
    pub size: (f32, f32),
}

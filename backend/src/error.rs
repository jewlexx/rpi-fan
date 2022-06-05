use std::{io::Error as IOError, num::ParseIntError};

use rocket::{http::Status, response::status::Custom};
use sys::gpio::Error as GPIOError;
use thiserror::Error as IsError;

use crate::state::ConfigError;

#[derive(Debug, IsError)]
pub enum FanError {
    #[error("Failed to interact with system IO")]
    IOError(#[from] IOError),
    #[error("Failed to parse integer from string")]
    ParseIntError(#[from] ParseIntError),
    #[error("Failed to open or interact with GPIO pin")]
    GPIOError(#[from] GPIOError),
    #[error("Failed to interact with config")]
    ConfigError(#[from] ConfigError),
    #[error("The given state is invalid")]
    InvalidState(String),
    #[error("Failed to lock the required mutex")]
    LockError,
}

pub type ResponseError = Custom<FanError>;

impl From<FanError> for Custom<FanError> {
    fn from(error: FanError) -> Self {
        Custom(Status::InternalServerError, error)
    }
}

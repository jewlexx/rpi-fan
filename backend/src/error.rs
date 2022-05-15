use std::{io::Error as IOError, num::ParseIntError};

use rocket::{http::Status, response::status::Custom};
use rppal::gpio::Error as GPIOError;
use thiserror::Error as IsError;

#[derive(Debug, IsError)]
pub enum FanError {
    #[error("Failed to interact with system IO")]
    IOError(IOError),
    #[error("Failed to parse integer from string")]
    ParseIntError(ParseIntError),
    #[error("Failed to open or interact with GPIO pin")]
    GPIOError(GPIOError),
    #[error("The given state is invalid")]
    InvalidState(String),
}

pub type ResponseError = Custom<FanError>;

impl From<FanError> for Custom<FanError> {
    fn from(error: FanError) -> Self {
        Custom(Status::InternalServerError, error)
    }
}

use thiserror::Error as AsError;

#[derive(Debug, Clone, Copy, AsError)]
pub enum Error {}

pub struct Gpio;

impl Gpio {
    pub fn new() -> Result<Gpio, Error> {
        Ok(Gpio)
    }
}

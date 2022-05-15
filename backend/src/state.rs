use std::{
    fs::File,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};
use strum::Display;
use thiserror::Error as IsError;

use crate::{
    consts::{CONFIG_DIR, FAN_STATE},
    error::FanError,
    get_pin,
};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    pub fan_pin: u8,
    pub fan_state: FanState,
    pub auto: bool,
}

#[derive(Debug, IsError)]
pub enum ConfigError {
    #[error("Could not read config file")]
    Read(#[from] std::io::Error),
    #[error("Could not write config file")]
    Write(std::io::Error),
    #[error("Could not parse config file")]
    Parse(#[from] serde_json::Error),
}

impl Default for Config {
    fn default() -> Self {
        Config {
            fan_pin: 14,
            fan_state: FanState::Off,
            auto: true,
        }
    }
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let cfg_path = CONFIG_DIR.join("config.json");
        let mut file = File::create(cfg_path).map_err(ConfigError::Write)?;

        let mut file_contents = String::new();

        file.read_to_string(&mut file_contents)?;

        let is_empty = file_contents.is_empty();

        if is_empty {
            let cfg = Config::default();
            let serialized = serde_json::to_string(&cfg)?;
            file.write_all(serialized.as_bytes())
                .map_err(ConfigError::Write)?;

            Ok(cfg)
        } else {
            let cfg = serde_json::from_str(&file_contents)?;

            Ok(cfg)
        }
    }

    fn write_config(cfg: &Config) -> Result<(), ConfigError> {
        let cfg_path = CONFIG_DIR.join("config.json");
        let mut file = File::create(cfg_path).map_err(ConfigError::Write)?;

        let serialized = serde_json::to_string(cfg)?;
        file.write_all(serialized.as_bytes())
            .map_err(ConfigError::Write)?;

        Ok(())
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        Config::write_config(self)
    }

    pub fn reset() -> Result<Self, ConfigError> {
        let cfg = Config::default();

        Config::write_config(&cfg)?;

        Ok(cfg)
    }
}

pub fn set_fan_state(state_opt: Option<FanState>) -> Result<FanState, FanError> {
    let mut fan_state = FAN_STATE.lock();
    let mut pin = get_pin()?;

    let updated_state: FanState = if let Some(state) = state_opt {
        if *fan_state == state {
            return Ok(*fan_state);
        }

        if bool::from(state) {
            pin.set_high();
        } else {
            pin.set_low();
        }

        state
    } else {
        pin.toggle();

        pin.is_set_high().into()
    };

    *fan_state = updated_state;

    Ok(updated_state)
}

#[derive(Debug, Clone, Copy, Display, PartialEq, Eq, Serialize, Deserialize)]
#[strum(serialize_all = "lowercase")]
pub enum FanState {
    On,
    Off,
}

impl From<bool> for FanState {
    fn from(state: bool) -> Self {
        if state {
            FanState::On
        } else {
            FanState::Off
        }
    }
}

impl From<FanState> for bool {
    fn from(state: FanState) -> Self {
        match state {
            FanState::On => true,
            FanState::Off => false,
        }
    }
}

impl From<String> for FanState {
    fn from(state: String) -> Self {
        match state.as_str() {
            "on" => FanState::On,
            "off" => FanState::Off,
            _ => FanState::Off,
        }
    }
}

impl<'a> From<&'a str> for FanState {
    fn from(state: &'a str) -> Self {
        match state {
            "on" => FanState::On,
            "off" => FanState::Off,
            _ => FanState::Off,
        }
    }
}

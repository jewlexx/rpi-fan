use strum::Display;

use crate::{consts::FAN_STATE, error::FanError, get_pin};

pub fn set_fan(state_opt: Option<FanState>) -> Result<(), FanError> {
    let mut pin = get_pin()?;

    let updated_state: FanState = if let Some(state) = state_opt {
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

    *FAN_STATE.lock() = updated_state;

    Ok(())
}

#[derive(Debug, Clone, Copy, Display)]
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

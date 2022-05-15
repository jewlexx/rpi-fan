use strum::Display;

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

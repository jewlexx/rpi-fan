use std::path::PathBuf;

use directories::ProjectDirs;
use parking_lot::{const_mutex, Mutex};

use crate::state::FanState;

lazy_static! {
    pub static ref DIRS: ProjectDirs = {
        ProjectDirs::from("", "jewelexx", "fan-control")
            .expect("failed to find project directories")
    };
    pub static ref CONFIG_DIR: PathBuf = DIRS.config_dir().into();
}

// This is the path to the file that holds temp information
pub const TEMPERATURE_PATH: &str = "/sys/class/thermal/thermal_zone0/temp";

pub const FAN_PIN: u8 = 14;

pub const MAX_TEMP: i128 = 80000;

pub static FAN_STATE: Mutex<FanState> = const_mutex(FanState::Off);

pub static TEMPERATURE: Mutex<i128> = const_mutex(0);

pub static AUTO_TEMP: Mutex<bool> = const_mutex(true);

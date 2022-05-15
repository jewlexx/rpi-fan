use std::path::PathBuf;

use directories::ProjectDirs;
use parking_lot::{const_mutex, Mutex};

use crate::state::Config;

lazy_static! {
    pub static ref DIRS: ProjectDirs = ProjectDirs::from("", "jewelexx", "fan-control")
        .expect("failed to find project directories");
    pub static ref CONFIG_DIR: PathBuf =
        std::env::current_dir().expect("failed to get current directory");
    pub static ref CONFIG: Mutex<Config> = const_mutex(Config::new().unwrap());
}

// This is the path to the file that holds temp information
pub const TEMPERATURE_PATH: &str = "/sys/class/thermal/thermal_zone0/temp";

pub static TEMPERATURE: Mutex<i128> = const_mutex(0);

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

// This is the constant that holds the current temperature with some magic known as const mutex
pub static TEMPERATURE: Mutex<i128> = const_mutex(0);

pub static CAN_ENTER: Mutex<bool> = const_mutex(true);

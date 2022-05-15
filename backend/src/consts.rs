use parking_lot::const_mutex;

use parking_lot::Mutex;

// This is the path to the file that holds temp information
pub const TEMPERATURE_PATH: &str = "/sys/class/thermal/thermal_zone0/temp";

pub const FAN_PIN: u8 = 14;

pub const MAX_TEMP: u32 = 80000;

pub static FAN_STATE: Mutex<bool> = const_mutex(false);

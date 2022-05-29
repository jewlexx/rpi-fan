use std::fs::{File, Read};

pub const TEMPERATURE_PATH: &str = "/sys/class/thermal/thermal_zone0/temp";

pub fn get_temp() -> i128 {
    let mut file = File::open(TEMPERATURE_PATH).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents.trim().parse::<i128>().unwrap()
}

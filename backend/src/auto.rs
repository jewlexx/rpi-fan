use std::{
    io::Read,
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::{
    consts::{CONFIG, TEMPERATURE, TEMPERATURE_PATH},
    state::{set_fan_state, FanState},
};

pub fn begin_monitoring() -> JoinHandle<()> {
    thread::spawn(|| loop {
        let mut file = std::fs::File::open(TEMPERATURE_PATH).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let temp = contents.trim().parse::<i128>().unwrap();
        *TEMPERATURE.lock() = temp;

        let res = if temp > (*CONFIG.lock()).max_temp {
            set_fan_state(Some(FanState::On))
        } else {
            set_fan_state(Some(FanState::Off))
        };

        if let Err(e) = res {
            println!("{}", e);
        }

        thread::sleep(Duration::from_secs(1));
    })
}

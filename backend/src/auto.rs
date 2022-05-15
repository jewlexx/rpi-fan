use std::{
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::consts::{FAN_STATE, MAX_TEMP, TEMPERATURE, TEMPERATURE_PATH};

pub fn begin_monitoring() -> JoinHandle<()> {
    thread::spawn(|| loop {
        let mut file = std::fs::File::open(TEMPERATURE_PATH).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let temp = contents.trim().parse::<i128>().unwrap();
        TEMPERATURE
            .lock()
            .store(temp, std::sync::atomic::Ordering::SeqCst);

        if temp > MAX_TEMP {
            FAN_STATE.lock().set_state(crate::state::FanState::On);
        } else {
            FAN_STATE.lock().set_state(crate::state::FanState::Off);
        }

        thread::sleep(Duration::from_secs(1));
    })
}

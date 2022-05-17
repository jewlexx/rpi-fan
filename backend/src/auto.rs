use std::{
    thread::{self, JoinHandle},
    time::Duration,
};

use sys::temp::get_temp;

use crate::{
    consts::{CONFIG, TEMPERATURE},
    state::{set_fan_state, FanState},
};

pub fn begin_monitoring() -> JoinHandle<()> {
    thread::spawn(|| loop {
        let temp = get_temp();
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

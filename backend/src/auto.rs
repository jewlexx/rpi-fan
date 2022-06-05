use std::{
    thread::{self, JoinHandle},
    time::Duration,
};

use reqwest::blocking::get;

use sys::temp::get_temp;

use crate::{
    consts::{CONFIG, TEMPERATURE},
    lock_inner_blocking,
    state::{set_fan_state, FanState},
};

const WEBHOOK_URL: &str =
    "https://maker.ifttt.com/trigger/temperature/with/key/bVBRTPY7TqwKOPzIFwf_M0";

pub fn begin_monitoring() -> JoinHandle<()> {
    thread::spawn(|| loop {
        let temp = get_temp();
        let config = *lock_inner_blocking!(CONFIG);
        *lock_inner_blocking!(TEMPERATURE) = temp;

        let res = if temp > config.max_temp {
            if let Err(e) = get(WEBHOOK_URL) {
                eprintln!("{}", e);
            };

            set_fan_state(Some(FanState::On))
        } else {
            set_fan_state(Some(FanState::Off))
        };

        if let Err(e) = res {
            eprintln!("{}", e);
        }

        thread::sleep(Duration::from_secs(1));
    })
}

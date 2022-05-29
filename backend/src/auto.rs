use std::{thread, time::Duration};

use reqwest::get;
use tokio::task::JoinHandle;

use sys::temp::get_temp;

use crate::{
    consts::{CONFIG, TEMPERATURE},
    state::{set_fan_state, FanState},
};

const WEBHOOK_URL: &str =
    "https://maker.ifttt.com/trigger/temperature/with/key/bVBRTPY7TqwKOPzIFwf_M0";

pub fn begin_monitoring() -> JoinHandle<()> {
    tokio::spawn(async {
        loop {
            let temp = get_temp();
            *TEMPERATURE.lock() = temp;

            let res = if temp > (*CONFIG.lock()).max_temp {
                if let Err(e) = get(WEBHOOK_URL).await {
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
        }
    })
}

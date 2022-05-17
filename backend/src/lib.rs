#![feature(decl_macro)]

use std::thread::{self, JoinHandle};

use rocket::{error::LaunchError, http::Status, response::status::Custom};
use rocket_contrib::serve::StaticFiles;
use sys::gpio::{Gpio, OutputPin};

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

mod auto;
mod consts;
mod error;
mod state;

use consts::*;
use error::*;
use state::{set_fan_state, FanState};

pub use auto::begin_monitoring;
pub use state::Config;

fn get_tmp_inner() -> i128 {
    *TEMPERATURE.lock()
}

fn get_pin() -> Result<OutputPin, FanError> {
    let cfg = Config::get();
    let gpio = Gpio::new().map_err(FanError::GPIOError)?;

    let pin = gpio
        .get(cfg.fan_pin)
        .map_err(FanError::GPIOError)?
        .into_output();

    Ok(pin)
}

#[get("/temp")]
fn get_tmp() -> String {
    get_tmp_inner().to_string()
}

#[get("/fan/<state>")]
fn set_fan(state: String) -> Result<String, ResponseError> {
    let mut cfg = Config::get();
    // These will actually mutate the fan's state
    let new_state: Option<FanState> = match state.as_str() {
        "on" | "off" => Some(state.into()),
        "toggle" => None,
        _ => return Err(Custom(Status::BadRequest, FanError::InvalidState(state))),
    };

    cfg.auto = false;
    cfg.save().map_err(FanError::from)?;

    Ok(format!("Fan set to {}", set_fan_state(new_state)?))
}

#[get("/auto/<state>")]
fn set_auto(state: String) -> Result<String, ResponseError> {
    let mut cfg = Config::get();

    let new_state: bool = match state.as_str() {
        "on" | "off" => state == "on",
        "toggle" => !cfg.auto,
        _ => return Err(Custom(Status::BadRequest, FanError::InvalidState(state))),
    };

    cfg.auto = new_state;
    cfg.save().map_err(FanError::from)?;

    Ok(format!("Auto set to {}", new_state))
}

pub fn run_server() -> JoinHandle<LaunchError> {
    thread::spawn(|| {
        let routes = routes![get_tmp, set_fan, set_auto];

        let ign = rocket::ignite()
            .mount("/api", routes)
            .mount("/", StaticFiles::from("./www"));

        ign.launch()
    })
}

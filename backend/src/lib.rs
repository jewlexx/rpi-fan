#![feature(decl_macro)]

use std::{
    io::Read,
    thread::{self, JoinHandle},
};

use rocket::{http::Status, response::status::Custom};
use rocket_contrib::serve::StaticFiles;
use rppal::gpio::{Gpio, OutputPin};

pub use rocket::error::{LaunchError, LaunchErrorKind};

#[macro_use]
extern crate rocket;

pub mod auto;
mod consts;
mod error;
mod state;

use consts::*;
use error::*;
use state::{set_fan_state, FanState};

fn get_tmp_inner() -> Result<i128, FanError> {
    let mut file = std::fs::File::open(TEMPERATURE_PATH).map_err(FanError::IOError)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(FanError::IOError)?;

    contents
        .trim()
        .parse::<i128>()
        .map_err(FanError::ParseIntError)
}

pub fn get_pin() -> Result<OutputPin, FanError> {
    let gpio = Gpio::new().map_err(FanError::GPIOError)?;

    let pin = gpio
        .get(FAN_PIN)
        .map_err(FanError::GPIOError)?
        .into_output();

    Ok(pin)
}

#[get("/temp")]
fn get_tmp() -> Result<String, ResponseError> {
    let temp = get_tmp_inner();

    match temp {
        Ok(t) => Ok(t.to_string()),
        Err(e) => Err(Custom(Status::InternalServerError, e)),
    }
}

#[get("/fan/<state>")]
fn set_fan(state: String) -> Result<String, ResponseError> {
    // These will actually mutate the fan's state
    let new_state: Option<FanState> = match state.as_str() {
        "on" | "off" => Some(state.into()),
        "toggle" => None,
        _ => return Err(Custom(Status::BadRequest, FanError::InvalidState(state))),
    };

    Ok(format!("Fan set to {}", set_fan_state(new_state)?))
}

pub fn run_server() -> JoinHandle<LaunchError> {
    thread::spawn(|| {
        let routes = routes![get_tmp, set_fan];

        let ign = rocket::ignite()
            .mount("/api", routes)
            .mount("/", StaticFiles::from("/www"));

        ign.launch()
    })
}

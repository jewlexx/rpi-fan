#![feature(decl_macro)]

use std::{
    io::{Error as IOError, Read},
    num::ParseIntError,
    thread::{self, JoinHandle},
};

use rocket::{http::Status, response::status::Custom};
use rocket_contrib::serve::StaticFiles;
use rppal::gpio::{Error as GPIOError, Gpio};
use thiserror::Error as IsError;

pub use rocket::error::{LaunchError, LaunchErrorKind};

#[macro_use]
extern crate rocket;

mod consts;

use consts::*;

#[derive(Debug, IsError)]
enum FanError {
    #[error("Failed to interact with system IO")]
    IOError(IOError),
    #[error("Failed to parse integer from string")]
    ParseIntError(ParseIntError),
    #[error("Failed to open or interact with GPIO pin")]
    GPIOError(GPIOError),
    #[error("The given state is invalid")]
    InvalidState(String),
}

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

impl From<FanError> for Custom<FanError> {
    fn from(error: FanError) -> Self {
        Custom(Status::InternalServerError, error)
    }
}

type ResponseError = Custom<FanError>;

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
    let mut fan_state = FAN_STATE.lock();

    if state == "state" {
        return Ok(if *fan_state { "on" } else { "off" }.into());
    }

    let gpio = Gpio::new().map_err(FanError::GPIOError)?;
    let mut pin = gpio
        .get(FAN_PIN)
        .map_err(FanError::GPIOError)?
        .into_output();

    match state.as_str() {
        "on" => {
            if !*fan_state {
                *fan_state = true;
                pin.set_high()
            }
        }
        "off" => {
            if *fan_state {
                *fan_state = false;
                pin.set_low()
            }
        }
        "toggle" => pin.toggle(),
        _ => return Err(Custom(Status::BadRequest, FanError::InvalidState(state))),
    }

    let new_state = if pin.is_set_high() { "on" } else { "off" };

    Ok(format!("Fan set to {}", new_state))
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

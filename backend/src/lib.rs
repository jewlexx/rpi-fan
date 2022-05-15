#![feature(decl_macro)]

use std::{
    io::Read,
    thread::{self, JoinHandle},
};

use rocket::{error::LaunchError, http::Status, response::status::Custom};
use strum::Display as SDisplay;
use thiserror::Error as IsError;

#[macro_use]
extern crate rocket;

// This is the path to the file that holds temp information
const TEMPERATURE_PATH: &str = "/sys/class/thermal/thermal_zone0/temp";

#[derive(Debug, IsError, SDisplay)]
enum TempError {
    IOError(std::io::Error),
    ParseError(std::num::ParseIntError),
}

fn get_tmp_inner() -> Result<i128, TempError> {
    let mut file = std::fs::File::open(TEMPERATURE_PATH).map_err(TempError::IOError)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(TempError::IOError)?;

    contents
        .trim()
        .parse::<i128>()
        .map_err(TempError::ParseError)
}

type ResponseError = Custom<TempError>;

#[get("/temp")]
fn get_tmp() -> Result<String, ResponseError> {
    let temp = get_tmp_inner();

    match temp {
        Ok(t) => Ok(t.to_string()),
        Err(e) => Err(Custom(Status::InternalServerError, e)),
    }
}

pub async fn run_server() -> JoinHandle<LaunchError> {
    thread::spawn(|| {
        let routes = routes![get_tmp];

        rocket::ignite().mount("/", routes).launch()
    })
}

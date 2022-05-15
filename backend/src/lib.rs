#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/tmp")]
fn get_tmp() -> &'static str {
    "Hello, world!"
}

pub async fn run_server() {
    let routes = routes![get_tmp];

    rocket::ignite().mount("/", routes).launch();
}

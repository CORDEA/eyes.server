#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::State;
use rocket_contrib::json::Json;
use serde::Deserialize;

use client::Client;
use crate::client::new_client;

mod client;

#[derive(Deserialize)]
pub struct LatLng {
    latitude: String,
    longitude: String,
}

#[post("/latlng", data = "<latlng>")]
fn latlng(client: State<Client>, latlng: Json<LatLng>) -> &'static str {
    ""
}

fn main() {
    rocket::ignite()
        .mount("/", routes![latlng])
        .manage(new_client(""))
        .launch();
}

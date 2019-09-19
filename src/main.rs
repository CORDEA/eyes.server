#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::env;

use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use client::Client;
use dotenv::dotenv;

use crate::client::new_client;

mod client;

#[derive(Deserialize)]
pub struct LatLng {
    latitude: String,
    longitude: String,
}

#[derive(Serialize)]
pub struct LatLngResponse {
    name: String,
}

#[post("/latlng", data = "<latlng>")]
fn latlng(client: State<Client>, latlng: Json<LatLng>) -> Json<LatLngResponse> {
    Json(LatLngResponse {
        name: "".to_string()
    })
}

fn main() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("Failed to get API key");
    rocket::ignite()
        .mount("/", routes![latlng])
        .manage(new_client(api_key))
        .launch();
}

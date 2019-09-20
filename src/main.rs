#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::env;

use log::error;
use rocket::http::Status;
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
fn latlng(client: State<Client>, latlng: Json<LatLng>) -> Result<Json<LatLngResponse>, Status> {
    let response = client.request(&latlng.latitude, &latlng.longitude);
    match response {
        Ok(response) => {
            let first = response.results.first();
            match first {
                None => Err(Status::InternalServerError),
                Some(result) => {
                    Ok(Json(LatLngResponse {
                        name: result.formatted_address.to_owned()
                    }))
                }
            }
        }
        Err(err) => {
            error!("{}", err);
            Err(Status::InternalServerError)
        }
    }
}

fn main() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("Failed to get API key");
    rocket::ignite()
        .mount("/", routes![latlng])
        .manage(new_client(api_key))
        .launch();
}

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::env;

use log::error;
use rocket::http::Status;
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use dotenv::dotenv;

mod google;
mod slack;
mod formatter;

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
fn latlng(google_client: State<google::Client>, slack_client: State<slack::Client>,
          latlng: Json<LatLng>) -> Result<Json<LatLngResponse>, Status> {
    let response = match google_client.request(&latlng.latitude, &latlng.longitude) {
        Ok(response) => response,
        Err(err) => {
            error!("{}", err);
            return Err(Status::InternalServerError);
        }
    };

    let result = match response.results.first() {
        Some(result) => result,
        None => return Err(Status::InternalServerError)
    };

    let format = formatter::format(&result.address_components);
    if let Err(err) = slack_client.request(&format) {
        error!("{}", err);
        return Err(Status::InternalServerError);
    }

    Ok(Json(LatLngResponse {
        name: format
    }))
}

fn main() {
    dotenv().ok();
    let google_api_key = env::var("GOOGLE_API_KEY").expect("Failed to get API key");
    let slack_api_key = env::var("SLACK_API_KEY").expect("Failed to get API key");
    rocket::ignite()
        .mount("/", routes![latlng])
        .manage(google::new_client(google_api_key))
        .manage(slack::new_client(slack_api_key))
        .launch();
}

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::json::Json;
use serde::Deserialize;

#[derive(Deserialize)]
struct LatLng {
    latitude: String,
    longitude: String,
}

#[post("/latlng", data = "<latlng>")]
fn latlng(latlng: Json<LatLng>) -> &'static str {
    ""
}

fn main() {
    rocket::ignite()
        .mount("/", routes![latlng])
        .launch();
}

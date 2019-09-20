use serde::Deserialize;

#[derive(Deserialize)]
pub struct Response {
    pub results: Vec<Result>,
}

#[derive(Deserialize)]
pub struct Result {
    pub address_components: Vec<AddressComponent>,
    formatted_address: String,
    geometry: Geometry,
}

#[derive(Deserialize)]
pub struct AddressComponent {
    pub long_name: String,
    short_name: String,
    pub types: Vec<String>,
}

#[derive(Deserialize)]
pub struct Geometry {
    location: Location,
    location_type: String,
}

#[derive(Deserialize)]
pub struct Location {
    lat: f32,
    lng: f32,
}
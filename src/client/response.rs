use serde::Deserialize;

#[derive(Deserialize)]
pub struct Response {
    pub results: Vec<Result>,
}

#[derive(Deserialize)]
pub struct Result {
    address_components: Vec<AddressComponent>,
    pub formatted_address: String,
    geometry: Geometry,
}

#[derive(Deserialize)]
pub struct AddressComponent {
    long_name: String,
    short_name: String,
    types: Vec<String>,
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
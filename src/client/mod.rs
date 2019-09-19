mod response;

use url::{Url, ParseError};
use response::Response;

const URL: &str = "https://maps.googleapis.com/maps/api/geocode/json";

struct Client {
    key: &'static str,
}

impl Client {
    fn request(&self, latitude: String, longitude: String) -> Result<Response, reqwest::Error> {
        let url = &self.build_url(latitude, longitude).expect("Failed to parse url");
        reqwest::get(url.as_str())?.json()
    }

    fn build_url(&self, latitude: String, longitude: String) -> Result<Url, ParseError> {
        let mut base = Url::parse(URL)?;
        base.set_query(Some(&format!("latlng={},{}", latitude, longitude)));
        base.set_query(Some(&format!("key={}", &self.key)));
        Ok(base)
    }
}
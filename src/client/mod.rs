use url::{ParseError, Url};

use response::Response;

mod response;

const URL: &str = "https://maps.googleapis.com/maps/api/geocode/json";

pub struct Client {
    key: String,
}

impl Client {
    pub fn request(&self, latitude: &str, longitude: &str) -> Result<Response, reqwest::Error> {
        let url = self.build_url(latitude, longitude).expect("Failed to parse url");
        reqwest::get(url.as_str())?.json()
    }

    fn build_url(&self, latitude: &str, longitude: &str) -> Result<Url, ParseError> {
        let mut base = Url::parse(URL)?;
        {
            let mut queries = base.query_pairs_mut();
            queries.append_pair("latlng", &format!("{},{}", latitude, longitude));
            queries.append_pair("key", &self.key);
        }
        Ok(base)
    }
}

pub fn new_client(key: String) -> Client {
    Client { key }
}
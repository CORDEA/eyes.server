use url::{Url, ParseError};

const URL: &str = "https://maps.googleapis.com/maps/api/geocode/json";

struct Client {
    key: &'static str,
}

impl Client {
    fn request(&self, latitude: String, longitude: String) {

    }

    fn build_url(&self, latitude: String, longitude: String) -> Result<Url, ParseError> {
        let mut base = Url::parse(URL)?;
        base.set_query(Some(&format!("latlng={},{}", latitude, longitude)));
        base.set_query(Some(&format!("key={}", &self.key)));
        Ok(base)
    }
}
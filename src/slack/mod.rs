use serde::Serialize;

const URL: &str = "https://slack.com/api/users.profile.set";

#[derive(Serialize)]
pub struct Request {
    profile: Profile
}

#[derive(Serialize)]
struct Profile {
    status_text: String,
    status_emoji: String,
    status_expiration: i32,
}

pub struct Client {
    key: String,
}

impl Client {
    pub fn request(&self, text: &str) -> Result<reqwest::Response, reqwest::Error> {
        let client = reqwest::Client::new();
        let profile = Request {
            profile: Profile {
                status_text: text.to_owned(),
                status_emoji: String::new(),
                status_expiration: 0,
            }
        };
        client.post(URL)
            .bearer_auth(&self.key)
            .json(&profile)
            .send()
    }
}

pub fn new_client(key: String) -> Client {
    Client { key }
}
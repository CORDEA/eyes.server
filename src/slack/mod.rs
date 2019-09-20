use serde::Deserialize;

const URL: &str = "https://slack.com/api/users.profile.set";

#[derive(Deserialize)]
pub struct Request {
    profile: Profile
}

#[derive(Deserialize)]
struct Profile {
    status_text: String,
    status_emoji: String,
    status_expiration: i32,
}

pub struct Client {
    key: String,
}

impl Client {

}

pub fn new_client(key: String) -> Client {
    Client { key }
}
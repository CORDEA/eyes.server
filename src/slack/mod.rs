
const URL: &str = "https://slack.com/api/users.profile.set";

pub struct Client {
    key: String,
}

impl Client {

}

pub fn new_client(key: String) -> Client {
    Client { key }
}
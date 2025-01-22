use reqwest::Client;


pub fn unauth_client() -> Client {
    Client::new()
}
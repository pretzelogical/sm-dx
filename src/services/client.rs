use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client,
};

pub fn unauth_client() -> Client {
    Client::new()
}

pub fn client(token: String) -> Client {
    Client::builder()
        .default_headers({
            let mut header_map = HeaderMap::new();
            header_map.append(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {:?}", token)).unwrap(),
            );
            header_map
        })
        .build()
        .unwrap()
}


use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, ORIGIN},
    Client,
};

pub fn create_client() -> Client {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(ORIGIN, HeaderValue::from_static("http://localhost:1420"));

    Client::builder()
        .default_headers(headers)
        .cookie_store(true)
        .build()
        .expect("Failed to build client")
}

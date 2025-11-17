// main.rs
use std::error::Error;
use std::result;

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};
use reqwest::StatusCode;

type Result<T> = result::Result<T, Box<dyn Error>>;

fn create_client() -> Result<Client> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("my-http-client"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = Client::builder()
        .default_headers(headers)
        .gzip(true)
        .build()?;
    
    Ok(client)
}

fn get_url(client: &Client, url: &str) -> Result<String> {
    let response = client.get(url).send()?;
    match response.status() {
        StatusCode::OK => {
            let body = response.text()?;
            Ok(body)
        },
        _ => Err(format!("Failed to get {}: {}", url, response.status()).into()),
    }
}

fn main() -> Result<()> {
    let client = create_client()?;
    let url = "https://example.com";
    let body = get_url(&client, url)?;
    println!("Body: {}", body);
    Ok(())
}
```

```rust
// Cargo.toml
[package]
name = "http_client"
version = "0.1.0"
edition = "2021"

[dependencies]
reqwest = "0.11.10"
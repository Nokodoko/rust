use reqwest::Error;
use serde_json;
use std::{collections::HashMap, ops::Bound};
use tokio;

async fn role_name<'a>(headers: HashMap<&'a str, &'a str>) -> Result<(), Error> {
    Ok(())
}

#[tokio::main]
async fn main() {
    let api_key: &'static str = match std::env::var("DD_API_KEY") {
        Ok(val) => Box::leak(val.into_boxed_str()),
        Err(e) => panic!("Error: {}", e),
    };

    let app_key: &'static str = match std::env::var("DD_APP_KEY") {
        Ok(val) => Box::leak(val.into_boxed_str()),
        Err(e) => panic!("Error: {}", e),
    };

    let dd_headers: HashMap<&str, &str> = vec![
        ("Accept", "application/json"),
        ("DD-API-KEY", api_key),
        ("DD-APP-KEY", app_key),
    ]
    .into_iter()
    .collect();
}

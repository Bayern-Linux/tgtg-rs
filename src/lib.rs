mod error;

mod items;

use crate::error::TgtgError;
use crate::items::Items;
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::StatusCode;
use serde_json::{json, Value};
use std::str::FromStr;
use std::time::Duration;
use tokio::time::Instant;
use tracing::{debug, info, trace};

const MAX_POLLING_TRIES: u8 = 24;

struct Client {
    base_url: String,
    email: String,
    access_token: Option<String>,
    refresh_token: Option<String>,
    user_id: Option<String>,
    datadome_cookie: Option<String>,
    language: String,
    proxies: Option<Vec<String>>,
    reqwest_client: reqwest::Client,
    access_token_lifetime: u64,
    last_time_token_refreshed: Instant,
    device_type: String,
}

impl Default for Client {
    fn default() -> Self {
        let reqwest_client = reqwest::Client::builder()
            // TODO: Make configurable
            .user_agent(
                "TGTG/22.11.11 Dalvik/2.1.0 (Linux; U; Android 14; Pixel 7 Build/UPB3.230519.008)",
            )
            .timeout(Duration::from_secs(2))
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::from_iter([
                    (
                        HeaderName::from_str("accept-language").unwrap(),
                        HeaderValue::from_str("en-GB").unwrap(),
                    ),
                    (
                        HeaderName::from_str("accept").unwrap(),
                        HeaderValue::from_str("application/json").unwrap(),
                    ),
                    (
                        HeaderName::from_str("content-type").unwrap(),
                        HeaderValue::from_str("application/json; charset=utf-8").unwrap(),
                    ),
                    (
                        HeaderName::from_str("Accept-Encoding").unwrap(),
                        HeaderValue::from_str("gzip").unwrap(),
                    ),
                ]);
                headers
            })
            .build()
            .unwrap();
        Self {
            base_url: "https://apptoogoodtogo.com/api/".to_string(),
            email: "test@email.com".to_string(),
            refresh_token: None,
            user_id: None,
            datadome_cookie: None,
            reqwest_client,
            access_token: None,
            language: "en-GB".to_string(),
            proxies: None,
            access_token_lifetime: 86400,
            last_time_token_refreshed: Instant::now(),
            device_type: "ANDROID".to_string(),
        }
    }
}

impl Client {
    async fn login(&mut self) {
        let response = self
            .reqwest_client
            .post(&format!("{}auth/v3/authByEmail", self.base_url))
            .json(&json!({
            "device_type": self.device_type,
            "email": self.email}))
            .send()
            .await
            .unwrap();
        if response.status() == 200 {
            let response_text = response.text().await.unwrap();
            debug!("authbyEmail Response: {}", response_text);
            let first_response: Value = serde_json::from_str(&response_text).unwrap();
            match first_response["state"].as_str().unwrap() {
                "TERMS" => panic!("Error: Account not linked to tgtg"),
                "WAIT" => self
                    .poll(first_response["polling_id"].to_string())
                    .await
                    .unwrap(),
                _ => {}
            }
        } else {
            panic!("Error: {}", response.status())
        }
    }
    async fn poll(&mut self, polling_id: String) -> Result<(), TgtgError> {
        for _ in 0..MAX_POLLING_TRIES {
            let response = self
                .reqwest_client
                .post(&format!("{}auth/v3/authByRequestPollingId", self.base_url))
                .json(&json!({
                "device_type": self.device_type,
                "email": self.email,
                "request_polling_id": polling_id}))
                .send()
                .await
                .unwrap();
            match response.status() {
                StatusCode::ACCEPTED => {
                    info!("Check your mailbox");
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue;
                }
                StatusCode::OK => {
                    info!("Token received");
                    let json: Value = response.json().await.unwrap();
                    self.access_token = Some(json["access_token"].to_string());
                    self.refresh_token = Some(json["refresh_token"].to_string());
                    self.last_time_token_refreshed = Instant::now();
                    self.user_id = Some(json["startup_data"]["user"]["user_id"].to_string());
                    return Ok(());
                }
                i => {
                    return Err(TgtgError::PollingError(format!("Status: {}\n Body: {}", i, response.text().await.unwrap())));
                }
            }
        }
        Err(TgtgError::PollingError(
            "Max polling tries exceeded".to_string(),
        ))
    }
    async fn get_items(&self, items: Items) {
        let response = self
            .reqwest_client
            .post(&format!("{}item/v8/", self.base_url))
            .json(&json!(items))
            .send()
            .await
            .unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::items::Position;
    use std::fs::read_to_string;
    use tracing_subscriber::EnvFilter;

    fn get_client() -> Client {
        Client {
            email: read_to_string(".email").unwrap(),
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_request_token() {
        // Start with tracing level set to trace
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::new("trace"))
            .init();
        let mut client = get_client();
        client.login().await;
    }

    async fn test_get_items() {
        let client = get_client();
        let test_items = Items {
            origin: Position {
                latitude: 49.476411,
                longitude: 8.481981,
            },
            radius: 10,
            page_size: 10,
            page_number: 1,
            discover: true,
            favorites_only: false,
            item_categories: vec!["sushi".to_string()],
            diet_categories: vec!["vegan".to_string()],
            pickup_earliest: vec!["2024-05-28T00:00:00Z".to_string()],
            pickup_latest: vec!["2024-05-28T23:59:59Z".to_string()],
            search_phrase: "vegan".to_string(),
            with_stock_only: true,
            hidden_only: false,
            we_care_only: false,
        };
        client.get_items(test_items).await;
    }
}

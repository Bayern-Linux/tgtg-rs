use std::str::FromStr;
use std::time::Duration;
use reqwest::header::{HeaderName, HeaderValue};
use serde_json::json;

const MAX_POLLING_TRIES: u8 = 24;

struct Client {
    base_url: String,
    email: String,
    refresh_token: Option<String>,
    user_id: Option<String>,
    datadome_cookie: Option<String>,
    language: String,
    proxies: Option<Vec<String>>,
    reqwest_client: reqwest::Client,
    access_token_lifetime: u64,
    device_type: String,
}
impl Default for Client {
    fn default() -> Self {
        let reqwest_client = reqwest::Client::builder()
            // TODO: Make configurable
            .user_agent("TGTG/22.11.11 Dalvik/2.1.0 (Linux; U; Android 14; Pixel 7 Build/UPB3.230519.008)")
            .timeout(Duration::from_secs(2))
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::from_iter([
                    (HeaderName::from_str("accept-language").unwrap(), HeaderValue::from_str("en-GB").unwrap()),
                    (HeaderName::from_str("accept").unwrap(), HeaderValue::from_str("application/json").unwrap()),
                    (HeaderName::from_str("content-type").unwrap(), HeaderValue::from_str("application/json; charset=utf-8").unwrap()),
                    (HeaderName::from_str("Accept-Encoding").unwrap(), HeaderValue::from_str("gzip").unwrap())]);
                headers
            })
            .build().unwrap();
        Self {
            base_url: "https://apptoogoodtogo.com/api/".to_string(),
            email: "test@email.com".to_string(),
            refresh_token: None,
            user_id: None,
            datadome_cookie: None,
            reqwest_client,
            language: "en-GB".to_string(),
            proxies: None,
            access_token_lifetime: 86400,
            device_type: "ANDROID".to_string(),
        }
    }
}

impl Client {
    async fn request_token(&self) {

        let response = self.reqwest_client.post(&format!("{}auth/v3/authByEmail", self.base_url))
            .json(&json!({
            "device_type": self.device_type,
            "email": self.email}))
            .send()
            .await.unwrap();
        // TODO: Extract polling ID
        self.poll(0).await;
        assert_eq!(response.status(), 200);
    }
    async fn poll(&self, polling_id: usize) {
        for _ in 0..MAX_POLLING_TRIES{
            let response = self.reqwest_client.post(&format!("{}auth/v3/authByRequestPollingId", self.base_url))
                .json(&json!({
                "device_type": self.device_type,
                "email": self.email}))
                .send()
                .await.unwrap();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_request_token() {
        let client = Client::default();
        client.request_token().await;
    }
}
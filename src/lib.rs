use std::str::FromStr;
use std::time::Duration;
use reqwest::header::{HeaderName, HeaderValue};
use serde_json::json;

struct Client {
    base_url: String,
    email: String,
    refresh_token: Option<String>,
    user_id: Option<String>,
    datadome_cookie: Option<String>,
    user_agent: String,
    language: String,
    proxies: Option<Vec<String>>,
    timeout: u64,
    access_token_lifetime: u64,
    device_type: String,
}
impl Default for Client {
    fn default() -> Self {
        Self {
            base_url: "https://apptoogoodtogo.com/api/".to_string(),
            email: "test@email.com".to_string(),
            refresh_token: None,
            user_id: None,
            datadome_cookie: None,
            user_agent: "TGTG/22.11.11 Dalvik/2.1.0 (Linux; U; Android 14; Pixel 7 Build/UPB3.230519.008)".to_string(),
            language: "en-GB".to_string(),
            proxies: None,
            timeout: 2,
            access_token_lifetime: 86400,
            device_type: "ANDROID".to_string(),
        }
    }

}
#[tokio::test]
async fn testilol() {
    let client = Client::default();
    let request_client = reqwest::Client::builder()
        .user_agent(client.user_agent)
        .timeout(Duration::from_secs(client.timeout))
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::from_iter([
                (HeaderName::from_str("accept-language").unwrap(), HeaderValue::from_str("en-GB").unwrap()),
                (HeaderName::from_str("accept").unwrap(), HeaderValue::from_str("application/json").unwrap()),
                (HeaderName::from_str("content-type").unwrap(), HeaderValue::from_str("application/json; charset=utf-8").unwrap()),
               (HeaderName::from_str("Accept-Encoding").unwrap(), HeaderValue::from_str("gzip").unwrap())]);
            headers
        })
        .build().unwrap();
    let response = request_client.post(&format!("{}auth/v3/authByEmail", client.base_url))
        .json(&json!({
            "device_type": client.device_type,
            "email": client.email}))
        .send()
        .await.unwrap();
    assert_eq!(response.status(), 200);
}
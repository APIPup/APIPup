use std::collections::HashMap;
use std::time::Instant;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct HttpResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub elapsed_ms: u64,
}

#[tauri::command]
pub async fn send_request(request: HttpRequest) -> Result<HttpResponse, String> {
    let client = reqwest::Client::new();

    let method: reqwest::Method = request
        .method
        .parse()
        .map_err(|e| format!("Invalid method: {e}"))?;

    let mut headers = HeaderMap::new();
    for (key, value) in &request.headers {
        let name = HeaderName::from_bytes(key.as_bytes())
            .map_err(|e| format!("Invalid header name '{key}': {e}"))?;
        let val = HeaderValue::from_str(value)
            .map_err(|e| format!("Invalid header value for '{key}': {e}"))?;
        headers.insert(name, val);
    }

    let mut req_builder = client.request(method, &request.url).headers(headers);

    if let Some(body) = &request.body {
        req_builder = req_builder.body(body.clone());
    }

    let start = Instant::now();
    let response = req_builder.send().await.map_err(|e| format!("Request failed: {e}"))?;
    let elapsed_ms = start.elapsed().as_millis() as u64;

    let status = response.status().as_u16();
    let status_text = response
        .status()
        .canonical_reason()
        .unwrap_or("Unknown")
        .to_string();

    let resp_headers: HashMap<String, String> = response
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("<binary>").to_string()))
        .collect();

    let body = response.text().await.map_err(|e| format!("Failed to read body: {e}"))?;

    Ok(HttpResponse {
        status,
        status_text,
        headers: resp_headers,
        body,
        elapsed_ms,
    })
}

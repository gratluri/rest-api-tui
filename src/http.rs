// HTTP client layer for executing API requests

use crate::models::{ApiEndpoint, AuthConfig, ApiKeyLocation, HttpMethod};
use crate::template;
use reqwest::{Client, StatusCode};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
    
    #[error("Template error: {0}")]
    Template(#[from] template::TemplateError),
    
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    
    #[error("Invalid header: {0}")]
    InvalidHeader(String),
}

pub type Result<T> = std::result::Result<T, HttpError>;

/// Input values for executing a request
#[derive(Debug, Clone, Default)]
pub struct RequestInputs {
    pub headers: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub body: Option<String>,
    pub variables: HashMap<String, String>,
}

/// Detailed timing breakdown for network traffic analysis
#[derive(Debug, Clone)]
pub struct NetworkTiming {
    pub dns_lookup: Option<Duration>,
    pub tcp_connect: Option<Duration>,
    pub tls_handshake: Option<Duration>,
    pub request_sent: Duration,
    pub waiting: Duration,
    pub content_download: Duration,
    pub total: Duration,
}

/// Captured request details for traffic analysis
#[derive(Debug, Clone)]
pub struct RequestDetails {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
    pub body_size: usize,
}

/// Network traffic details (Wireshark-style)
#[derive(Debug, Clone)]
pub struct NetworkTraffic {
    pub timing: NetworkTiming,
    pub request: RequestDetails,
    pub response_headers_size: usize,
    pub response_body_size: usize,
}

/// HTTP response with metadata
#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub duration: Duration,
    pub traffic: Option<NetworkTraffic>,
}

/// HTTP client for executing API requests
pub struct HttpClient {
    client: Client,
    default_timeout: Duration,
}

impl HttpClient {
    /// Create a new HTTP client with default timeout of 30 seconds
    pub fn new() -> Result<Self> {
        Self::with_timeout(Duration::from_secs(30))
    }
    
    /// Create a new HTTP client with custom timeout
    pub fn with_timeout(timeout: Duration) -> Result<Self> {
        let client = Client::builder()
            .timeout(timeout)
            .pool_max_idle_per_host(10)
            .build()
            .map_err(HttpError::Request)?;
        
        Ok(Self {
            client,
            default_timeout: timeout,
        })
    }
    
    /// Get the default timeout
    pub fn timeout(&self) -> Duration {
        self.default_timeout
    }
    
    /// Build a URL with query parameters
    fn build_url(base_url: &str, query_params: &HashMap<String, String>) -> Result<String> {
        if query_params.is_empty() {
            return Ok(base_url.to_string());
        }
        
        let mut url = base_url.to_string();
        let separator = if url.contains('?') { '&' } else { '?' };
        
        let query_string: Vec<String> = query_params
            .iter()
            .map(|(k, v)| {
                format!(
                    "{}={}",
                    urlencoding::encode(k),
                    urlencoding::encode(v)
                )
            })
            .collect();
        
        url.push(separator);
        url.push_str(&query_string.join("&"));
        
        Ok(url)
    }
    
    /// Validate and normalize header name
    fn validate_header_name(name: &str) -> Result<String> {
        // HTTP header names should be ASCII and not contain certain characters
        if name.is_empty() {
            return Err(HttpError::InvalidHeader("Header name cannot be empty".to_string()));
        }
        
        if !name.chars().all(|c| c.is_ascii() && !c.is_control()) {
            return Err(HttpError::InvalidHeader(
                format!("Invalid header name: {}", name)
            ));
        }
        
        Ok(name.to_string())
    }
    
    /// Apply authentication configuration to headers or query params
    fn apply_auth(
        auth: &AuthConfig,
        headers: &mut HashMap<String, String>,
        query_params: &mut HashMap<String, String>,
        variables: &HashMap<String, String>,
    ) -> Result<()> {
        match auth {
            AuthConfig::Bearer { token } => {
                let token_value = template::substitute(token, variables)?;
                headers.insert("Authorization".to_string(), format!("Bearer {}", token_value));
            }
            AuthConfig::Basic { username, password } => {
                let username_value = template::substitute(username, variables)?;
                let password_value = template::substitute(password, variables)?;
                let credentials = format!("{}:{}", username_value, password_value);
                let encoded = base64::Engine::encode(
                    &base64::engine::general_purpose::STANDARD,
                    credentials.as_bytes()
                );
                headers.insert("Authorization".to_string(), format!("Basic {}", encoded));
            }
            AuthConfig::ApiKey { name, value, location } => {
                let key_value = template::substitute(value, variables)?;
                match location {
                    ApiKeyLocation::Header => {
                        let header_name = template::substitute(name, variables)?;
                        headers.insert(header_name, key_value);
                    }
                    ApiKeyLocation::QueryParam => {
                        let param_name = template::substitute(name, variables)?;
                        query_params.insert(param_name, key_value);
                    }
                }
            }
        }
        Ok(())
    }
    
    /// Execute an HTTP request
    pub async fn execute(
        &self,
        endpoint: &ApiEndpoint,
        inputs: &RequestInputs,
    ) -> Result<HttpResponse> {
        let start = Instant::now();
        
        // Substitute variables in URL
        let url = template::substitute(&endpoint.url, &inputs.variables)?;
        
        // Build query parameters (merge endpoint defaults with inputs)
        let mut query_params = inputs.query_params.clone();
        
        // Apply authentication (may add to headers or query params)
        let mut headers = inputs.headers.clone();
        if let Some(auth) = &endpoint.auth {
            Self::apply_auth(auth, &mut headers, &mut query_params, &inputs.variables)?;
        }
        
        // Merge endpoint headers with input headers (inputs override)
        for (key, value) in &endpoint.headers {
            if !headers.contains_key(key) {
                let substituted = template::substitute(value, &inputs.variables)?;
                headers.insert(key.clone(), substituted);
            }
        }
        
        // Build final URL with query parameters
        let final_url = Self::build_url(&url, &query_params)?;
        
        // Capture request details for traffic analysis
        let request_body = if let Some(body) = &inputs.body {
            Some(template::substitute(body, &inputs.variables)?)
        } else if let Some(body_template) = &endpoint.body_template {
            Some(template::substitute(body_template, &inputs.variables)?)
        } else {
            None
        };
        
        let request_body_bytes = request_body.as_ref().map(|b| b.as_bytes().to_vec());
        let request_body_size = request_body_bytes.as_ref().map(|b| b.len()).unwrap_or(0);
        
        let request_details = RequestDetails {
            method: format!("{:?}", endpoint.method),
            url: final_url.clone(),
            headers: headers.clone(),
            body: request_body_bytes.clone(),
            body_size: request_body_size,
        };
        
        // Build request
        let mut request = match endpoint.method {
            HttpMethod::GET => self.client.get(&final_url),
            HttpMethod::POST => self.client.post(&final_url),
            HttpMethod::PUT => self.client.put(&final_url),
            HttpMethod::PATCH => self.client.patch(&final_url),
            HttpMethod::DELETE => self.client.delete(&final_url),
            HttpMethod::HEAD => self.client.head(&final_url),
            HttpMethod::OPTIONS => self.client.request(reqwest::Method::OPTIONS, &final_url),
        };
        
        // Add headers
        for (key, value) in headers {
            Self::validate_header_name(&key)?;
            request = request.header(key, value);
        }
        
        // Add body if present
        if let Some(body_content) = request_body {
            request = request.body(body_content);
        }
        
        // Mark request send start
        let request_send_start = Instant::now();
        
        // Execute request
        let response = request.send().await?;
        
        // Mark waiting time (time to first byte)
        let waiting_end = Instant::now();
        let waiting_duration = waiting_end.duration_since(request_send_start);
        
        // Extract response data
        let status = response.status();
        let response_headers: HashMap<String, String> = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();
        
        // Calculate response headers size (approximate)
        let response_headers_size: usize = response_headers
            .iter()
            .map(|(k, v)| k.len() + v.len() + 4) // +4 for ": " and "\r\n"
            .sum();
        
        // Download response body
        let download_start = Instant::now();
        let body = response.bytes().await?.to_vec();
        let download_duration = download_start.elapsed();
        
        let total_duration = start.elapsed();
        
        // Build network traffic details
        // Note: We can't easily get DNS/TCP/TLS timing from reqwest without custom connectors
        // So we'll estimate based on what we have
        let request_sent_duration = Duration::from_millis(1); // Approximate
        
        let traffic = NetworkTraffic {
            timing: NetworkTiming {
                dns_lookup: None, // Would need custom DNS resolver
                tcp_connect: None, // Would need custom connector
                tls_handshake: None, // Would need custom TLS connector
                request_sent: request_sent_duration,
                waiting: waiting_duration,
                content_download: download_duration,
                total: total_duration,
            },
            request: request_details,
            response_headers_size,
            response_body_size: body.len(),
        };
        
        Ok(HttpResponse {
            status,
            headers: response_headers,
            body,
            duration: total_duration,
            traffic: Some(traffic),
        })
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default HTTP client")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_client_creation() {
        let client = HttpClient::new().unwrap();
        assert_eq!(client.timeout(), Duration::from_secs(30));
    }

    #[test]
    fn test_http_client_custom_timeout() {
        let timeout = Duration::from_secs(60);
        let client = HttpClient::with_timeout(timeout).unwrap();
        assert_eq!(client.timeout(), timeout);
    }

    #[test]
    fn test_request_inputs_default() {
        let inputs = RequestInputs::default();
        assert_eq!(inputs.headers.len(), 0);
        assert_eq!(inputs.query_params.len(), 0);
        assert!(inputs.body.is_none());
        assert_eq!(inputs.variables.len(), 0);
    }
    
    #[test]
    fn test_build_url_no_params() {
        let url = HttpClient::build_url("https://api.example.com/users", &HashMap::new()).unwrap();
        assert_eq!(url, "https://api.example.com/users");
    }
    
    #[test]
    fn test_build_url_with_params() {
        let mut params = HashMap::new();
        params.insert("page".to_string(), "1".to_string());
        params.insert("limit".to_string(), "20".to_string());
        
        let url = HttpClient::build_url("https://api.example.com/users", &params).unwrap();
        assert!(url.contains("page=1"));
        assert!(url.contains("limit=20"));
        assert!(url.contains("?"));
    }
    
    #[test]
    fn test_build_url_with_special_chars() {
        let mut params = HashMap::new();
        params.insert("q".to_string(), "hello world".to_string());
        params.insert("filter".to_string(), "name=John&age>25".to_string());
        
        let url = HttpClient::build_url("https://api.example.com/search", &params).unwrap();
        assert!(url.contains("hello%20world"));
        assert!(url.contains("%3D")); // encoded =
        assert!(url.contains("%26")); // encoded &
    }
    
    #[test]
    fn test_validate_header_name_valid() {
        let result = HttpClient::validate_header_name("Content-Type");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Content-Type");
    }
    
    #[test]
    fn test_validate_header_name_empty() {
        let result = HttpClient::validate_header_name("");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_apply_auth_bearer() {
        let auth = AuthConfig::Bearer {
            token: "my-token-{{env}}".to_string(),
        };
        let mut headers = HashMap::new();
        let mut query_params = HashMap::new();
        let mut variables = HashMap::new();
        variables.insert("env".to_string(), "prod".to_string());
        
        HttpClient::apply_auth(&auth, &mut headers, &mut query_params, &variables).unwrap();
        
        assert_eq!(headers.get("Authorization"), Some(&"Bearer my-token-prod".to_string()));
    }
    
    #[test]
    fn test_apply_auth_basic() {
        let auth = AuthConfig::Basic {
            username: "user".to_string(),
            password: "pass".to_string(),
        };
        let mut headers = HashMap::new();
        let mut query_params = HashMap::new();
        let variables = HashMap::new();
        
        HttpClient::apply_auth(&auth, &mut headers, &mut query_params, &variables).unwrap();
        
        let auth_header = headers.get("Authorization").unwrap();
        assert!(auth_header.starts_with("Basic "));
        
        // Decode and verify
        let encoded = auth_header.strip_prefix("Basic ").unwrap();
        let decoded = base64::Engine::decode(
            &base64::engine::general_purpose::STANDARD,
            encoded
        ).unwrap();
        assert_eq!(String::from_utf8(decoded).unwrap(), "user:pass");
    }
    
    #[test]
    fn test_apply_auth_api_key_header() {
        let auth = AuthConfig::ApiKey {
            name: "X-API-Key".to_string(),
            value: "secret-{{key}}".to_string(),
            location: ApiKeyLocation::Header,
        };
        let mut headers = HashMap::new();
        let mut query_params = HashMap::new();
        let mut variables = HashMap::new();
        variables.insert("key".to_string(), "123".to_string());
        
        HttpClient::apply_auth(&auth, &mut headers, &mut query_params, &variables).unwrap();
        
        assert_eq!(headers.get("X-API-Key"), Some(&"secret-123".to_string()));
        assert_eq!(query_params.len(), 0);
    }
    
    #[test]
    fn test_apply_auth_api_key_query() {
        let auth = AuthConfig::ApiKey {
            name: "api_key".to_string(),
            value: "secret-{{key}}".to_string(),
            location: ApiKeyLocation::QueryParam,
        };
        let mut headers = HashMap::new();
        let mut query_params = HashMap::new();
        let mut variables = HashMap::new();
        variables.insert("key".to_string(), "456".to_string());
        
        HttpClient::apply_auth(&auth, &mut headers, &mut query_params, &variables).unwrap();
        
        assert_eq!(query_params.get("api_key"), Some(&"secret-456".to_string()));
        assert_eq!(headers.len(), 0);
    }
}


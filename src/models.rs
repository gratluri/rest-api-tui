use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// HTTP methods supported by the application
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
}

/// Location where API key should be placed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ApiKeyLocation {
    Header,
    QueryParam,
}

/// Authentication configuration for API endpoints
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum AuthConfig {
    ApiKey {
        name: String,
        value: String,
        location: ApiKeyLocation,
    },
    Bearer {
        token: String,
    },
    Basic {
        username: String,
        password: String,
    },
}

/// A single API endpoint definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApiEndpoint {
    pub id: Uuid,
    pub name: String,
    pub method: HttpMethod,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body_template: Option<String>,
    pub auth: Option<AuthConfig>,
    pub description: Option<String>,
    pub load_test_config: Option<LoadTestConfigData>,
}

/// Load test configuration data (serializable)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoadTestConfigData {
    pub concurrency: usize,
    pub duration_secs: u64,
    pub ramp_up_secs: Option<u64>,
    pub rate_limit: Option<usize>,
}

/// A collection of related API endpoints
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApiCollection {
    pub id: Uuid,
    pub name: String,
    pub endpoints: Vec<ApiEndpoint>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ApiCollection {
    /// Create a new empty collection
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            endpoints: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Add an endpoint to the collection
    pub fn add_endpoint(&mut self, endpoint: ApiEndpoint) {
        self.endpoints.push(endpoint);
        self.updated_at = Utc::now();
    }

    /// Remove an endpoint by ID
    pub fn remove_endpoint(&mut self, endpoint_id: &Uuid) -> bool {
        let initial_len = self.endpoints.len();
        self.endpoints.retain(|e| &e.id != endpoint_id);
        let removed = self.endpoints.len() < initial_len;
        if removed {
            self.updated_at = Utc::now();
        }
        removed
    }
}

impl ApiEndpoint {
    /// Create a new API endpoint
    pub fn new(name: String, method: HttpMethod, url: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            method,
            url,
            headers: HashMap::new(),
            body_template: None,
            auth: None,
            description: None,
            load_test_config: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collection_creation() {
        let collection = ApiCollection::new("Test Collection".to_string());
        assert_eq!(collection.name, "Test Collection");
        assert_eq!(collection.endpoints.len(), 0);
    }

    #[test]
    fn test_add_endpoint() {
        let mut collection = ApiCollection::new("Test".to_string());
        let endpoint = ApiEndpoint::new(
            "Get Users".to_string(),
            HttpMethod::GET,
            "https://api.example.com/users".to_string(),
        );
        collection.add_endpoint(endpoint);
        assert_eq!(collection.endpoints.len(), 1);
    }

    #[test]
    fn test_remove_endpoint() {
        let mut collection = ApiCollection::new("Test".to_string());
        let endpoint = ApiEndpoint::new(
            "Get Users".to_string(),
            HttpMethod::GET,
            "https://api.example.com/users".to_string(),
        );
        let endpoint_id = endpoint.id;
        collection.add_endpoint(endpoint);
        
        let removed = collection.remove_endpoint(&endpoint_id);
        assert!(removed);
        assert_eq!(collection.endpoints.len(), 0);
    }

    #[test]
    fn test_serialization() {
        let collection = ApiCollection::new("Test".to_string());
        let json = serde_json::to_string(&collection).unwrap();
        let deserialized: ApiCollection = serde_json::from_str(&json).unwrap();
        assert_eq!(collection.id, deserialized.id);
        assert_eq!(collection.name, deserialized.name);
    }
}

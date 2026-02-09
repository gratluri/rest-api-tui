// Variable management and storage

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VariableError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Variable not found: {0}")]
    NotFound(String),
    
    #[error("Invalid variable name: {0}")]
    InvalidName(String),
}

pub type Result<T> = std::result::Result<T, VariableError>;

/// A set of variables (key-value pairs)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VariableSet {
    pub name: String,
    pub variables: HashMap<String, String>,
    pub description: Option<String>,
}

impl VariableSet {
    pub fn new(name: String) -> Self {
        Self {
            name,
            variables: HashMap::new(),
            description: None,
        }
    }
    
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
    
    pub fn set(&mut self, key: String, value: String) {
        self.variables.insert(key, value);
    }
    
    pub fn get(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }
    
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.variables.remove(key)
    }
    
    pub fn keys(&self) -> Vec<String> {
        self.variables.keys().cloned().collect()
    }
    
    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }
    
    pub fn len(&self) -> usize {
        self.variables.len()
    }
}

/// Variable manager for storing and loading variables
pub struct VariableManager {
    storage_path: PathBuf,
    current_set: VariableSet,
}

impl VariableManager {
    /// Create a new variable manager with default storage path
    pub fn new() -> Result<Self> {
        let storage_path = Self::default_storage_path()?;
        Self::with_path(storage_path)
    }
    
    /// Create a new variable manager with custom storage path
    pub fn with_path(storage_path: PathBuf) -> Result<Self> {
        // Ensure parent directory exists
        if let Some(parent) = storage_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Load existing variables or create new set
        let current_set = if storage_path.exists() {
            Self::load_from_file(&storage_path)?
        } else {
            VariableSet::new("default".to_string())
        };
        
        Ok(Self {
            storage_path,
            current_set,
        })
    }
    
    /// Get the default storage path for variables
    fn default_storage_path() -> Result<PathBuf> {
        let home = dirs::home_dir()
            .ok_or_else(|| VariableError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Home directory not found"
            )))?;
        
        Ok(home.join(".rest-api-tui").join("variables.json"))
    }
    
    /// Load variables from file
    fn load_from_file(path: &PathBuf) -> Result<VariableSet> {
        let content = fs::read_to_string(path)?;
        let variable_set = serde_json::from_str(&content)?;
        Ok(variable_set)
    }
    
    /// Save variables to file
    pub fn save(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.current_set)?;
        fs::write(&self.storage_path, json)?;
        Ok(())
    }
    
    /// Get a reference to the current variable set
    pub fn current_set(&self) -> &VariableSet {
        &self.current_set
    }
    
    /// Get a mutable reference to the current variable set
    pub fn current_set_mut(&mut self) -> &mut VariableSet {
        &mut self.current_set
    }
    
    /// Set a variable
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.current_set.set(key, value);
        self.save()?;
        Ok(())
    }
    
    /// Get a variable value
    pub fn get(&self, key: &str) -> Option<&String> {
        self.current_set.get(key)
    }
    
    /// Remove a variable
    pub fn remove(&mut self, key: &str) -> Result<Option<String>> {
        let result = self.current_set.remove(key);
        self.save()?;
        Ok(result)
    }
    
    /// Get all variables as a HashMap
    pub fn get_all(&self) -> &HashMap<String, String> {
        &self.current_set.variables
    }
    
    /// Get all variable keys
    pub fn keys(&self) -> Vec<String> {
        self.current_set.keys()
    }
    
    /// Check if variables are empty
    pub fn is_empty(&self) -> bool {
        self.current_set.is_empty()
    }
    
    /// Get the number of variables
    pub fn len(&self) -> usize {
        self.current_set.len()
    }
    
    /// Clear all variables
    pub fn clear(&mut self) -> Result<()> {
        self.current_set.variables.clear();
        self.save()?;
        Ok(())
    }
}

impl Default for VariableManager {
    fn default() -> Self {
        Self::new().expect("Failed to create default VariableManager")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_variable_set_new() {
        let set = VariableSet::new("test".to_string());
        assert_eq!(set.name, "test");
        assert!(set.variables.is_empty());
        assert!(set.description.is_none());
    }

    #[test]
    fn test_variable_set_operations() {
        let mut set = VariableSet::new("test".to_string());
        
        set.set("key1".to_string(), "value1".to_string());
        assert_eq!(set.get("key1"), Some(&"value1".to_string()));
        assert_eq!(set.len(), 1);
        
        set.set("key2".to_string(), "value2".to_string());
        assert_eq!(set.len(), 2);
        
        let removed = set.remove("key1");
        assert_eq!(removed, Some("value1".to_string()));
        assert_eq!(set.len(), 1);
        
        assert!(set.get("key1").is_none());
        assert_eq!(set.get("key2"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_variable_manager_with_temp_path() {
        let temp_dir = TempDir::new().unwrap();
        let storage_path = temp_dir.path().join("variables.json");
        
        let mut manager = VariableManager::with_path(storage_path.clone()).unwrap();
        
        manager.set("test_key".to_string(), "test_value".to_string()).unwrap();
        assert_eq!(manager.get("test_key"), Some(&"test_value".to_string()));
        
        // Reload from file
        let manager2 = VariableManager::with_path(storage_path).unwrap();
        assert_eq!(manager2.get("test_key"), Some(&"test_value".to_string()));
    }

    #[test]
    fn test_variable_manager_operations() {
        let temp_dir = TempDir::new().unwrap();
        let storage_path = temp_dir.path().join("variables.json");
        let mut manager = VariableManager::with_path(storage_path).unwrap();
        
        assert!(manager.is_empty());
        
        manager.set("key1".to_string(), "value1".to_string()).unwrap();
        manager.set("key2".to_string(), "value2".to_string()).unwrap();
        
        assert_eq!(manager.len(), 2);
        assert!(!manager.is_empty());
        
        let keys = manager.keys();
        assert!(keys.contains(&"key1".to_string()));
        assert!(keys.contains(&"key2".to_string()));
        
        manager.remove("key1").unwrap();
        assert_eq!(manager.len(), 1);
        
        manager.clear().unwrap();
        assert!(manager.is_empty());
    }
}

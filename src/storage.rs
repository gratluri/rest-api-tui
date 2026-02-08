// Storage layer for persisting collections and load test results

use crate::models::ApiCollection;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Collection not found: {0}")]
    NotFound(Uuid),
}

pub type Result<T> = std::result::Result<T, StorageError>;

/// Manages persistent storage of collections and load test results
pub struct StorageManager {
    collections_dir: PathBuf,
    #[allow(dead_code)]
    results_dir: PathBuf,
}

impl StorageManager {
    /// Create a new StorageManager with specified directories
    pub fn new(collections_dir: PathBuf, results_dir: PathBuf) -> Result<Self> {
        // Create directories if they don't exist
        fs::create_dir_all(&collections_dir)?;
        fs::create_dir_all(&results_dir)?;
        
        Ok(Self {
            collections_dir,
            results_dir,
        })
    }
    
    /// Create a StorageManager with default directories in user's home
    pub fn with_defaults() -> Result<Self> {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let base_dir = home.join(".rest-api-tui");
        
        Self::new(
            base_dir.join("collections"),
            base_dir.join("results"),
        )
    }
    
    /// Get the path to a collection file
    fn collection_path(&self, id: &Uuid) -> PathBuf {
        self.collections_dir.join(format!("{}.json", id))
    }
    
    /// Load all collections from the collections directory
    pub fn load_collections(&self) -> Result<Vec<ApiCollection>> {
        let mut collections = Vec::new();
        
        // Read all files in the collections directory
        let entries = match fs::read_dir(&self.collections_dir) {
            Ok(entries) => entries,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                // Directory doesn't exist yet, return empty vec
                return Ok(collections);
            }
            Err(e) => return Err(e.into()),
        };
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            // Only process .json files
            if path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }
            
            // Try to load and parse the collection
            match self.load_collection_from_path(&path) {
                Ok(collection) => collections.push(collection),
                Err(e) => {
                    // Log error and skip corrupted files
                    eprintln!("Warning: Failed to load collection from {:?}: {}", path, e);
                    continue;
                }
            }
        }
        
        Ok(collections)
    }
    
    /// Load a single collection from a file path
    fn load_collection_from_path(&self, path: &Path) -> Result<ApiCollection> {
        let contents = fs::read_to_string(path)?;
        let collection: ApiCollection = serde_json::from_str(&contents)?;
        Ok(collection)
    }
    
    /// Save a collection to disk using atomic writes
    pub fn save_collection(&self, collection: &ApiCollection) -> Result<()> {
        let path = self.collection_path(&collection.id);
        
        // Serialize to JSON with pretty printing
        let json = serde_json::to_string_pretty(collection)?;
        
        // Atomic write: write to temp file, then rename
        let temp_path = path.with_extension("json.tmp");
        fs::write(&temp_path, json)?;
        fs::rename(&temp_path, &path)?;
        
        Ok(())
    }
    
    /// Delete a collection by ID
    pub fn delete_collection(&self, id: &Uuid) -> Result<()> {
        let path = self.collection_path(id);
        
        if !path.exists() {
            return Err(StorageError::NotFound(*id));
        }
        
        fs::remove_file(path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_storage_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let collections_dir = temp_dir.path().join("collections");
        let results_dir = temp_dir.path().join("results");
        
        let storage = StorageManager::new(collections_dir.clone(), results_dir.clone()).unwrap();
        
        // Verify directories were created
        assert!(collections_dir.exists());
        assert!(results_dir.exists());
    }
    
    #[test]
    fn test_collection_path() {
        let temp_dir = TempDir::new().unwrap();
        let storage = StorageManager::new(
            temp_dir.path().join("collections"),
            temp_dir.path().join("results"),
        ).unwrap();
        
        let id = Uuid::new_v4();
        let path = storage.collection_path(&id);
        
        assert!(path.to_string_lossy().contains(&id.to_string()));
        assert!(path.to_string_lossy().ends_with(".json"));
    }
    
    #[test]
    fn test_load_collections_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let storage = StorageManager::new(
            temp_dir.path().join("collections"),
            temp_dir.path().join("results"),
        ).unwrap();
        
        let collections = storage.load_collections().unwrap();
        assert_eq!(collections.len(), 0);
    }
    
    #[test]
    fn test_load_collections_with_corrupted_file() {
        let temp_dir = TempDir::new().unwrap();
        let collections_dir = temp_dir.path().join("collections");
        let storage = StorageManager::new(
            collections_dir.clone(),
            temp_dir.path().join("results"),
        ).unwrap();
        
        // Create a corrupted JSON file
        let corrupted_path = collections_dir.join("corrupted.json");
        fs::write(&corrupted_path, "{ invalid json }").unwrap();
        
        // Should skip corrupted file and return empty vec
        let collections = storage.load_collections().unwrap();
        assert_eq!(collections.len(), 0);
    }
    
    #[test]
    fn test_save_and_load_collection() {
        let temp_dir = TempDir::new().unwrap();
        let storage = StorageManager::new(
            temp_dir.path().join("collections"),
            temp_dir.path().join("results"),
        ).unwrap();
        
        // Create and save a collection
        let collection = ApiCollection::new("Test Collection".to_string());
        storage.save_collection(&collection).unwrap();
        
        // Load it back
        let collections = storage.load_collections().unwrap();
        assert_eq!(collections.len(), 1);
        assert_eq!(collections[0].id, collection.id);
        assert_eq!(collections[0].name, "Test Collection");
    }
    
    #[test]
    fn test_atomic_write() {
        let temp_dir = TempDir::new().unwrap();
        let storage = StorageManager::new(
            temp_dir.path().join("collections"),
            temp_dir.path().join("results"),
        ).unwrap();
        
        let collection = ApiCollection::new("Test".to_string());
        storage.save_collection(&collection).unwrap();
        
        // Verify the temp file was cleaned up
        let temp_path = storage.collection_path(&collection.id).with_extension("json.tmp");
        assert!(!temp_path.exists());
        
        // Verify the actual file exists and is valid JSON
        let path = storage.collection_path(&collection.id);
        assert!(path.exists());
        let contents = fs::read_to_string(&path).unwrap();
        let _: ApiCollection = serde_json::from_str(&contents).unwrap();
    }
    
    #[test]
    fn test_delete_collection() {
        let temp_dir = TempDir::new().unwrap();
        let storage = StorageManager::new(
            temp_dir.path().join("collections"),
            temp_dir.path().join("results"),
        ).unwrap();
        
        // Create and save a collection
        let collection = ApiCollection::new("Test".to_string());
        storage.save_collection(&collection).unwrap();
        
        // Verify it exists
        assert!(storage.collection_path(&collection.id).exists());
        
        // Delete it
        storage.delete_collection(&collection.id).unwrap();
        
        // Verify it's gone
        assert!(!storage.collection_path(&collection.id).exists());
    }
    
    #[test]
    fn test_delete_nonexistent_collection() {
        let temp_dir = TempDir::new().unwrap();
        let storage = StorageManager::new(
            temp_dir.path().join("collections"),
            temp_dir.path().join("results"),
        ).unwrap();
        
        let id = Uuid::new_v4();
        let result = storage.delete_collection(&id);
        
        assert!(result.is_err());
        match result {
            Err(StorageError::NotFound(found_id)) => assert_eq!(found_id, id),
            _ => panic!("Expected NotFound error"),
        }
    }
}

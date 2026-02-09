// Complete TUI application

use crate::models::{ApiCollection, ApiEndpoint, HttpMethod};
use crate::storage::StorageManager;
use crate::http::{HttpClient, RequestInputs, HttpResponse};
use crate::formatter;
use crate::load_test::{LoadTestEngine, LoadTestConfig, LoadTestMetrics};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    CollectionList,
    CollectionEdit(Option<usize>), // None for new, Some(idx) for edit
    EndpointList(usize), // collection index
    EndpointEdit(usize, Option<usize>), // collection index, None for new, Some(idx) for edit
    EndpointDetail(usize, usize), // collection index, endpoint index
    ResponseView(usize, usize), // collection index, endpoint index
    LoadTestConfig(usize, usize), // collection index, endpoint index
    LoadTestRunning(usize, usize), // collection index, endpoint index
    ConfirmDelete(DeleteTarget), // confirmation dialog
    Help,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeleteTarget {
    Collection(usize),
    Endpoint(usize, usize), // collection index, endpoint index
}

#[derive(Debug, Clone)]
pub struct CollectionForm {
    pub name: String,
    pub editing_index: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct EndpointForm {
    pub name: String,
    pub method: HttpMethod,
    pub url: String,
    pub description: String,
    pub headers: HashMap<String, String>,
    pub body_template: String,
    pub timeout_secs: String, // Timeout in seconds (empty = use default)
    pub collection_index: usize,
    pub editing_index: Option<usize>,
    pub current_field: usize, // 0=name, 1=method, 2=url, 3=description, 4=headers, 5=body, 6=timeout
    pub header_edit_mode: bool, // true when editing headers
    pub header_key: String, // current header key being edited
    pub header_value: String, // current header value being edited
    pub header_edit_field: usize, // 0=key, 1=value
}

#[derive(Debug, Clone)]
pub struct LoadTestConfigForm {
    pub concurrency: String,
    pub duration: String,
    pub ramp_up: String,
    pub current_field: usize, // 0=concurrency, 1=duration, 2=ramp_up
    pub collection_index: usize,
    pub endpoint_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PanelFocus {
    Collections,
    Endpoints,
}

pub struct AppState {
    pub collections: Vec<ApiCollection>,
    pub current_screen: Screen,
    pub previous_screen: Option<Screen>,
    pub selected_collection_index: usize,
    pub selected_endpoint_index: usize,
    pub selected_index: usize, // For backward compatibility
    pub panel_focus: PanelFocus,
    pub show_network_traffic: bool, // Toggle for network traffic display
    pub show_response_headers: bool, // Toggle for response headers display
    pub response_scroll_offset: usize, // Vertical scroll offset for response panel
    pub storage: StorageManager,
    pub http_client: HttpClient,
    pub last_response: Option<HttpResponse>,
    pub last_response_formatted: Option<String>,
    pub load_test_engine: Option<LoadTestEngine>,
    pub load_test_config: LoadTestConfig,
    pub error_message: Option<String>,
    pub status_message: Option<String>,
    pub collection_form: Option<CollectionForm>,
    pub endpoint_form: Option<EndpointForm>,
    pub load_test_config_form: Option<LoadTestConfigForm>,
}

impl AppState {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let storage = StorageManager::with_defaults()?;
        let collections = storage.load_collections()?;
        let http_client = HttpClient::new()?;
        
        Ok(Self {
            collections,
            current_screen: Screen::CollectionList,
            previous_screen: None,
            selected_collection_index: 0,
            selected_endpoint_index: 0,
            selected_index: 0,
            panel_focus: PanelFocus::Collections,
            show_network_traffic: false, // Disabled by default
            show_response_headers: false, // Disabled by default
            response_scroll_offset: 0,
            storage,
            http_client,
            last_response: None,
            last_response_formatted: None,
            load_test_engine: None,
            load_test_config: LoadTestConfig::new(10, Duration::from_secs(30)),
            error_message: None,
            status_message: None,
            collection_form: None,
            endpoint_form: None,
            load_test_config_form: None,
        })
    }
    
    pub fn toggle_network_traffic(&mut self) {
        self.show_network_traffic = !self.show_network_traffic;
    }
    
    pub fn toggle_response_headers(&mut self) {
        self.show_response_headers = !self.show_response_headers;
    }
    
    pub fn scroll_response_up(&mut self, lines: usize) {
        self.response_scroll_offset = self.response_scroll_offset.saturating_sub(lines);
    }
    
    pub fn scroll_response_down(&mut self, lines: usize) {
        self.response_scroll_offset = self.response_scroll_offset.saturating_add(lines);
    }
    
    pub fn reset_response_scroll(&mut self) {
        self.response_scroll_offset = 0;
    }
    
    pub fn scroll_response_to_end(&mut self) {
        // Set to a very large number, will be clamped in draw function
        self.response_scroll_offset = usize::MAX;
    }
    
    pub fn navigate_up(&mut self) {
        match self.panel_focus {
            PanelFocus::Collections => {
                if self.selected_collection_index > 0 {
                    self.selected_collection_index -= 1;
                    self.selected_endpoint_index = 0; // Reset endpoint selection
                }
            }
            PanelFocus::Endpoints => {
                if self.selected_endpoint_index > 0 {
                    self.selected_endpoint_index -= 1;
                }
            }
        }
        // Backward compatibility
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }
    
    pub fn navigate_down(&mut self, max: usize) {
        match self.panel_focus {
            PanelFocus::Collections => {
                if self.selected_collection_index < max.saturating_sub(1) {
                    self.selected_collection_index += 1;
                    self.selected_endpoint_index = 0; // Reset endpoint selection
                }
            }
            PanelFocus::Endpoints => {
                if self.selected_endpoint_index < max.saturating_sub(1) {
                    self.selected_endpoint_index += 1;
                }
            }
        }
        // Backward compatibility
        if self.selected_index < max.saturating_sub(1) {
            self.selected_index += 1;
        }
    }
    
    pub fn toggle_panel_focus(&mut self) {
        self.panel_focus = match self.panel_focus {
            PanelFocus::Collections => PanelFocus::Endpoints,
            PanelFocus::Endpoints => PanelFocus::Collections,
        };
    }
    
    pub fn navigate_back(&mut self) {
        let new_screen = match &self.current_screen {
            Screen::CollectionEdit(_) => {
                self.collection_form = None;
                Screen::CollectionList
            }
            Screen::EndpointList(_) => Screen::CollectionList,
            Screen::EndpointEdit(coll_idx, _) => {
                self.endpoint_form = None;
                Screen::EndpointList(*coll_idx)
            }
            Screen::EndpointDetail(coll_idx, _) => Screen::EndpointList(*coll_idx),
            Screen::ResponseView(coll_idx, _) => Screen::EndpointList(*coll_idx),
            Screen::LoadTestConfig(_, _) => {
                self.load_test_config_form = None;
                Screen::CollectionList
            }
            Screen::LoadTestRunning(coll_idx, _) => Screen::EndpointList(*coll_idx),
            Screen::ConfirmDelete(_) => {
                // Go back to previous screen
                self.previous_screen.clone().unwrap_or(Screen::CollectionList)
            }
            Screen::Help => Screen::CollectionList,
            _ => Screen::CollectionList,
        };
        self.current_screen = new_screen;
        self.selected_index = 0;
    }
    
    pub fn select(&mut self) {
        match &self.current_screen {
            Screen::CollectionList => {
                // In new layout, selecting a collection shows its endpoints
                if self.panel_focus == PanelFocus::Collections {
                    // Switch focus to endpoints panel
                    self.panel_focus = PanelFocus::Endpoints;
                    self.selected_endpoint_index = 0;
                } else if self.panel_focus == PanelFocus::Endpoints {
                    // Select the endpoint to view details
                    if let Some(collection) = self.collections.get(self.selected_collection_index) {
                        if self.selected_endpoint_index < collection.endpoints.len() {
                            self.current_screen = Screen::EndpointDetail(
                                self.selected_collection_index,
                                self.selected_endpoint_index
                            );
                        }
                    }
                }
            }
            Screen::EndpointList(coll_idx) => {
                if let Some(collection) = self.collections.get(*coll_idx) {
                    if self.selected_index < collection.endpoints.len() {
                        self.current_screen = Screen::EndpointDetail(*coll_idx, self.selected_index);
                    }
                }
            }
            _ => {}
        }
    }
    
    pub async fn execute_request(&mut self, coll_idx: usize, ep_idx: usize) {
        if let Some(collection) = self.collections.get(coll_idx) {
            if let Some(endpoint) = collection.endpoints.get(ep_idx) {
                self.status_message = Some("Executing request...".to_string());
                
                let inputs = RequestInputs::default();
                match self.http_client.execute(endpoint, &inputs).await {
                    Ok(response) => {
                        // Format response
                        let formatted = formatter::format_auto(&response.body)
                            .unwrap_or_else(|_| String::from_utf8_lossy(&response.body).to_string());
                        
                        self.last_response = Some(response);
                        self.last_response_formatted = Some(formatted);
                        self.response_scroll_offset = 0; // Reset scroll on new response
                        // Stay on the same screen in new layout
                        self.status_message = Some("Request completed successfully".to_string());
                        self.error_message = None;
                    }
                    Err(e) => {
                        self.error_message = Some(format!("Request failed: {}", e));
                        self.status_message = None;
                    }
                }
            }
        }
    }
    
    pub fn start_load_test(&mut self, coll_idx: usize, ep_idx: usize) {
        // Show configuration form first
        if let Some(collection) = self.collections.get(coll_idx) {
            if let Some(endpoint) = collection.endpoints.get(ep_idx) {
                // Load existing config or use defaults
                let (concurrency, duration, ramp_up) = if let Some(config) = &endpoint.load_test_config {
                    (
                        config.concurrency.to_string(),
                        config.duration_secs.to_string(),
                        config.ramp_up_secs.map(|s| s.to_string()).unwrap_or_default(),
                    )
                } else {
                    ("10".to_string(), "30".to_string(), String::new())
                };
                
                self.load_test_config_form = Some(LoadTestConfigForm {
                    concurrency,
                    duration,
                    ramp_up,
                    current_field: 0,
                    collection_index: coll_idx,
                    endpoint_index: ep_idx,
                });
                
                self.current_screen = Screen::LoadTestConfig(coll_idx, ep_idx);
            }
        }
    }
    
    pub fn execute_load_test(&mut self) {
        if let Some(form) = &self.load_test_config_form {
            let coll_idx = form.collection_index;
            let ep_idx = form.endpoint_index;
            
            // Parse configuration
            let concurrency = form.concurrency.parse::<usize>().unwrap_or(10);
            let duration_secs = form.duration.parse::<u64>().unwrap_or(30);
            let ramp_up_secs = if form.ramp_up.is_empty() {
                None
            } else {
                form.ramp_up.parse::<u64>().ok()
            };
            
            // Create config
            let mut config = LoadTestConfig::new(concurrency, Duration::from_secs(duration_secs));
            if let Some(ramp_up) = ramp_up_secs {
                config = config.with_ramp_up(Duration::from_secs(ramp_up));
            }
            
            // Validate
            if let Err(e) = config.validate() {
                self.error_message = Some(e);
                return;
            }
            
            // Save config to endpoint
            if let Some(collection) = self.collections.get_mut(coll_idx) {
                if let Some(endpoint) = collection.endpoints.get_mut(ep_idx) {
                    endpoint.load_test_config = Some(crate::models::LoadTestConfigData {
                        concurrency,
                        duration_secs,
                        ramp_up_secs,
                        rate_limit: None,
                    });
                    let _ = self.storage.save_collection(collection);
                }
            }
            
            // Clear form
            self.load_test_config_form = None;
            
            // Start the actual load test
            self.execute_load_test_with_config(coll_idx, ep_idx, config);
        }
    }
    
    fn execute_load_test_with_config(&mut self, coll_idx: usize, ep_idx: usize, config: LoadTestConfig) {
        if let Some(collection) = self.collections.get(coll_idx) {
            if let Some(endpoint) = collection.endpoints.get(ep_idx) {
                let endpoint = endpoint.clone();
                let http_client = self.http_client.clone();
                
                match LoadTestEngine::new(config.clone()) {
                    Ok(engine) => {
                        let collector = engine.collector();
                        let is_running = Arc::new(Mutex::new(true));
                        let is_running_clone = is_running.clone();
                        
                        // Set engine state
                        engine.set_start_time(std::time::Instant::now());
                        engine.set_running(true);
                        
                        // Store engine before spawning thread
                        self.load_test_engine = Some(engine);
                        self.current_screen = Screen::LoadTestRunning(coll_idx, ep_idx);
                        self.status_message = Some("Load test started...".to_string());
                        self.error_message = None;
                        
                        // Spawn background thread for load test execution
                        std::thread::spawn(move || {
                            let runtime = tokio::runtime::Runtime::new().unwrap();
                            runtime.block_on(async {
                                let start = std::time::Instant::now();
                                let mut handles = vec![];
                                
                                // Spawn concurrent tasks based on ramp-up
                                for worker_id in 0..config.concurrency {
                                    let endpoint = endpoint.clone();
                                    let http_client = http_client.clone();
                                    let collector = collector.clone();
                                    let is_running = is_running_clone.clone();
                                    let duration = config.duration;
                                    let ramp_up = config.ramp_up;
                                    
                                    let handle = tokio::spawn(async move {
                                        // Calculate delay for this worker based on ramp-up
                                        if let Some(ramp_up_duration) = ramp_up {
                                            let worker_delay = ramp_up_duration.as_secs_f64() 
                                                * (worker_id as f64 / config.concurrency as f64);
                                            tokio::time::sleep(tokio::time::Duration::from_secs_f64(worker_delay)).await;
                                        }
                                        
                                        while start.elapsed() < duration && *is_running.lock().unwrap() {
                                            let req_start = std::time::Instant::now();
                                            let inputs = RequestInputs::default();
                                            
                                            match http_client.execute(&endpoint, &inputs).await {
                                                Ok(response) => {
                                                    collector.record_success(response.duration);
                                                }
                                                Err(e) => {
                                                    collector.record_failure(
                                                        e.to_string(),
                                                        req_start.elapsed()
                                                    );
                                                }
                                            }
                                            
                                            // Small delay to prevent overwhelming the server
                                            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                                        }
                                    });
                                    
                                    handles.push(handle);
                                }
                                
                                // Periodically update RPS
                                let collector_for_rps = collector.clone();
                                let is_running_for_rps = is_running_clone.clone();
                                tokio::spawn(async move {
                                    while *is_running_for_rps.lock().unwrap() {
                                        collector_for_rps.update_rps(Duration::from_secs(1));
                                        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                                    }
                                });
                                
                                // Periodically collect time-series data (every 5 seconds)
                                let collector_for_timeseries = collector.clone();
                                let is_running_for_timeseries = is_running_clone.clone();
                                tokio::spawn(async move {
                                    while *is_running_for_timeseries.lock().unwrap() {
                                        collector_for_timeseries.add_time_series_point(start);
                                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                                    }
                                });
                                
                                // Wait for all tasks to complete
                                for handle in handles {
                                    let _ = handle.await;
                                }
                                
                                // Mark as stopped
                                *is_running_clone.lock().unwrap() = false;
                            });
                        });
                    }
                    Err(e) => {
                        self.error_message = Some(format!("Failed to start load test: {}", e));
                    }
                }
            }
        }
    }
    
    pub fn stop_load_test(&mut self) {
        if let Some(engine) = &self.load_test_engine {
            engine.stop();
            self.status_message = Some("Load test stopped".to_string());
        }
    }
    
    pub fn get_load_test_metrics(&self) -> Option<LoadTestMetrics> {
        self.load_test_engine.as_ref().map(|e| e.metrics())
    }
    
    // Collection Management
    
    pub fn start_new_collection(&mut self) {
        self.collection_form = Some(CollectionForm {
            name: String::new(),
            editing_index: None,
        });
        self.current_screen = Screen::CollectionEdit(None);
    }
    
    pub fn start_edit_collection(&mut self, index: usize) {
        if let Some(collection) = self.collections.get(index) {
            self.collection_form = Some(CollectionForm {
                name: collection.name.clone(),
                editing_index: Some(index),
            });
            self.current_screen = Screen::CollectionEdit(Some(index));
        }
    }
    
    pub fn save_collection(&mut self) {
        if let Some(form) = &self.collection_form {
            if form.name.trim().is_empty() {
                self.error_message = Some("Collection name cannot be empty".to_string());
                return;
            }
            
            match form.editing_index {
                Some(index) => {
                    // Edit existing collection
                    if let Some(collection) = self.collections.get_mut(index) {
                        collection.name = form.name.clone();
                        match self.storage.save_collection(collection) {
                            Ok(_) => {
                                self.status_message = Some("Collection updated successfully".to_string());
                                self.error_message = None;
                                self.current_screen = Screen::CollectionList;
                                self.collection_form = None;
                            }
                            Err(e) => {
                                self.error_message = Some(format!("Failed to save collection: {}", e));
                            }
                        }
                    }
                }
                None => {
                    // Create new collection
                    let collection = ApiCollection::new(form.name.clone());
                    match self.storage.save_collection(&collection) {
                        Ok(_) => {
                            self.collections.push(collection);
                            self.status_message = Some("Collection created successfully".to_string());
                            self.error_message = None;
                            self.current_screen = Screen::CollectionList;
                            self.collection_form = None;
                        }
                        Err(e) => {
                            self.error_message = Some(format!("Failed to create collection: {}", e));
                        }
                    }
                }
            }
        }
    }
    
    pub fn confirm_delete_collection(&mut self, index: usize) {
        self.previous_screen = Some(self.current_screen.clone());
        self.current_screen = Screen::ConfirmDelete(DeleteTarget::Collection(index));
    }
    
    pub fn delete_collection(&mut self, index: usize) {
        if let Some(collection) = self.collections.get(index) {
            match self.storage.delete_collection(&collection.id) {
                Ok(_) => {
                    self.collections.remove(index);
                    self.status_message = Some("Collection deleted successfully".to_string());
                    self.error_message = None;
                    if self.selected_index >= self.collections.len() && self.selected_index > 0 {
                        self.selected_index -= 1;
                    }
                    self.current_screen = Screen::CollectionList;
                }
                Err(e) => {
                    self.error_message = Some(format!("Failed to delete collection: {}", e));
                    self.navigate_back();
                }
            }
        }
    }
    
    // Endpoint Management
    
    pub fn start_new_endpoint(&mut self, collection_index: usize) {
        self.endpoint_form = Some(EndpointForm {
            name: String::new(),
            method: HttpMethod::GET,
            url: String::new(),
            description: String::new(),
            headers: HashMap::new(),
            body_template: String::new(),
            timeout_secs: String::new(), // Empty = use default
            collection_index,
            editing_index: None,
            current_field: 0,
            header_edit_mode: false,
            header_key: String::new(),
            header_value: String::new(),
            header_edit_field: 0,
        });
        self.current_screen = Screen::EndpointEdit(collection_index, None);
    }
    
    pub fn start_edit_endpoint(&mut self, collection_index: usize, endpoint_index: usize) {
        if let Some(collection) = self.collections.get(collection_index) {
            if let Some(endpoint) = collection.endpoints.get(endpoint_index) {
                self.endpoint_form = Some(EndpointForm {
                    name: endpoint.name.clone(),
                    method: endpoint.method.clone(),
                    url: endpoint.url.clone(),
                    description: endpoint.description.clone().unwrap_or_default(),
                    headers: endpoint.headers.clone(),
                    body_template: endpoint.body_template.clone().unwrap_or_default(),
                    timeout_secs: endpoint.timeout_secs.map(|t| t.to_string()).unwrap_or_default(),
                    collection_index,
                    editing_index: Some(endpoint_index),
                    current_field: 0,
                    header_edit_mode: false,
                    header_key: String::new(),
                    header_value: String::new(),
                    header_edit_field: 0,
                });
                self.current_screen = Screen::EndpointEdit(collection_index, Some(endpoint_index));
            }
        }
    }
    
    pub fn save_endpoint(&mut self) {
        if let Some(form) = &self.endpoint_form {
            if form.name.trim().is_empty() {
                self.error_message = Some("Endpoint name cannot be empty".to_string());
                return;
            }
            
            if form.url.trim().is_empty() {
                self.error_message = Some("Endpoint URL cannot be empty".to_string());
                return;
            }
            
            if let Some(collection) = self.collections.get_mut(form.collection_index) {
                // Parse timeout from form
                let timeout_secs = if form.timeout_secs.trim().is_empty() {
                    None
                } else {
                    form.timeout_secs.trim().parse::<u64>().ok()
                };
                
                let endpoint = ApiEndpoint {
                    id: if let Some(idx) = form.editing_index {
                        collection.endpoints.get(idx).map(|e| e.id).unwrap_or_else(|| uuid::Uuid::new_v4())
                    } else {
                        uuid::Uuid::new_v4()
                    },
                    name: form.name.clone(),
                    method: form.method.clone(),
                    url: form.url.clone(),
                    description: if form.description.is_empty() { None } else { Some(form.description.clone()) },
                    headers: form.headers.clone(),
                    body_template: if form.body_template.is_empty() { None } else { Some(form.body_template.clone()) },
                    auth: None,
                    load_test_config: if let Some(idx) = form.editing_index {
                        collection.endpoints.get(idx).and_then(|e| e.load_test_config.clone())
                    } else {
                        None
                    },
                    timeout_secs,
                };
                
                match form.editing_index {
                    Some(index) => {
                        // Edit existing endpoint
                        if let Some(ep) = collection.endpoints.get_mut(index) {
                            *ep = endpoint;
                        }
                    }
                    None => {
                        // Add new endpoint
                        collection.add_endpoint(endpoint);
                    }
                }
                
                match self.storage.save_collection(collection) {
                    Ok(_) => {
                        self.status_message = Some("Endpoint saved successfully".to_string());
                        self.error_message = None;
                        self.current_screen = Screen::EndpointList(form.collection_index);
                        self.endpoint_form = None;
                    }
                    Err(e) => {
                        self.error_message = Some(format!("Failed to save endpoint: {}", e));
                    }
                }
            }
        }
    }
    
    pub fn confirm_delete_endpoint(&mut self, collection_index: usize, endpoint_index: usize) {
        self.previous_screen = Some(self.current_screen.clone());
        self.current_screen = Screen::ConfirmDelete(DeleteTarget::Endpoint(collection_index, endpoint_index));
    }
    
    pub fn delete_endpoint(&mut self, collection_index: usize, endpoint_index: usize) {
        if let Some(collection) = self.collections.get_mut(collection_index) {
            if let Some(endpoint) = collection.endpoints.get(endpoint_index) {
                let endpoint_id = endpoint.id;
                collection.remove_endpoint(&endpoint_id);
                
                match self.storage.save_collection(collection) {
                    Ok(_) => {
                        self.status_message = Some("Endpoint deleted successfully".to_string());
                        self.error_message = None;
                        if self.selected_index >= collection.endpoints.len() && self.selected_index > 0 {
                            self.selected_index -= 1;
                        }
                        self.current_screen = Screen::EndpointList(collection_index);
                    }
                    Err(e) => {
                        self.error_message = Some(format!("Failed to delete endpoint: {}", e));
                        self.navigate_back();
                    }
                }
            }
        }
    }
    
    pub fn cycle_http_method(&mut self) {
        if let Some(form) = &mut self.endpoint_form {
            form.method = match form.method {
                HttpMethod::GET => HttpMethod::POST,
                HttpMethod::POST => HttpMethod::PUT,
                HttpMethod::PUT => HttpMethod::PATCH,
                HttpMethod::PATCH => HttpMethod::DELETE,
                HttpMethod::DELETE => HttpMethod::HEAD,
                HttpMethod::HEAD => HttpMethod::OPTIONS,
                HttpMethod::OPTIONS => HttpMethod::GET,
            };
        }
    }
    
    pub fn get_delete_confirmation_message(&self) -> Option<String> {
        match &self.current_screen {
            Screen::ConfirmDelete(target) => {
                match target {
                    DeleteTarget::Collection(idx) => {
                        self.collections.get(*idx).map(|c| {
                            format!("Delete collection '{}'?\n\nThis will permanently delete the collection and all {} endpoint(s).", 
                                c.name, c.endpoints.len())
                        })
                    }
                    DeleteTarget::Endpoint(coll_idx, ep_idx) => {
                        self.collections.get(*coll_idx).and_then(|c| {
                            c.endpoints.get(*ep_idx).map(|e| {
                                format!("Delete endpoint '{}'?\n\n{:?} {}", 
                                    e.name, e.method, e.url)
                            })
                        })
                    }
                }
            }
            _ => None
        }
    }
    
    pub fn confirm_delete_action(&mut self) {
        if let Screen::ConfirmDelete(target) = &self.current_screen.clone() {
            match target {
                DeleteTarget::Collection(idx) => {
                    self.delete_collection(*idx);
                }
                DeleteTarget::Endpoint(coll_idx, ep_idx) => {
                    self.delete_endpoint(*coll_idx, *ep_idx);
                }
            }
        }
    }
    
    // Header Management
    
    pub fn toggle_header_edit_mode(&mut self) {
        if let Some(form) = &mut self.endpoint_form {
            if form.current_field == 4 { // Headers field
                form.header_edit_mode = !form.header_edit_mode;
                if form.header_edit_mode {
                    // Entering header edit mode
                    form.header_key = String::new();
                    form.header_value = String::new();
                    form.header_edit_field = 0;
                }
            }
        }
    }
    
    pub fn add_header(&mut self) {
        if let Some(form) = &mut self.endpoint_form {
            if !form.header_key.trim().is_empty() {
                form.headers.insert(form.header_key.clone(), form.header_value.clone());
                form.header_key = String::new();
                form.header_value = String::new();
                form.header_edit_field = 0;
                self.status_message = Some("Header added".to_string());
            }
        }
    }
    
    pub fn remove_header(&mut self, key: &str) {
        if let Some(form) = &mut self.endpoint_form {
            form.headers.remove(key);
            self.status_message = Some(format!("Header '{}' removed", key));
        }
    }
    
    pub fn cycle_header_field(&mut self) {
        if let Some(form) = &mut self.endpoint_form {
            if form.header_edit_mode {
                form.header_edit_field = (form.header_edit_field + 1) % 2;
            }
        }
    }
}

# Load Test Bug Report

## Bug Description

When pressing 'l' key to start a load test, nothing happens. The progress bar and statistics remain unchanged, indicating no action is being taken.

**Note**: The 'e' key DOES work for executing single requests.

## Root Cause Analysis

### The 'l' Key Handler Exists

**File**: `src/tui/ui.rs` (lines 444-453)

```rust
'l' => {
    // Start load test if endpoint is selected
    if let Some(collection) = app.collections.get(app.selected_collection_index) {
        if app.selected_endpoint_index < collection.endpoints.len() {
            app.start_load_test(
                app.selected_collection_index,
                app.selected_endpoint_index
            );
        }
    }
}
```

✅ Keyboard handler exists and is called
✅ Calls `app.start_load_test()`
❌ But `start_load_test()` doesn't actually execute requests

### The Problem: start_load_test() Does Nothing

**File**: `src/tui_app.rs` (lines 264-275)

```rust
pub fn start_load_test(&mut self, coll_idx: usize, ep_idx: usize) {
    match LoadTestEngine::new(self.load_test_config.clone()) {
        Ok(engine) => {
            self.load_test_engine = Some(engine);
            self.current_screen = Screen::LoadTestRunning(coll_idx, ep_idx);
            self.status_message = Some("Load test started".to_string());
            self.error_message = None;
        }
        Err(e) => {
            self.error_message = Some(format!("Failed to start load test: {}", e));
        }
    }
}
```

This method:
1. Creates a `LoadTestEngine` (which is just a data structure)
2. Changes the screen to `LoadTestRunning`
3. Sets a status message

But it **never actually starts executing HTTP requests**!

### LoadTestEngine Has No Execution Logic

**File**: `src/load_test.rs`

The `LoadTestEngine` struct has:
- ✅ Metrics collection (`MetricsCollector`)
- ✅ Configuration (`LoadTestConfig`)
- ✅ Running flag (`is_running`)
- ❌ **NO method to actually execute requests**
- ❌ **NO background task spawning**
- ❌ **NO HTTP client integration**

The engine is created but never "started". It's like creating a car but never turning on the engine.

## Comparison: Why 'e' Works But 'l' Doesn't

### 'e' Key (Execute Single Request) - WORKS ✅

**File**: `src/tui/ui.rs` (lines 395-401)

```rust
'e' => {
    // Edit or Execute based on context
    if matches!(app.current_screen, Screen::EndpointDetail(_, _)) {
        // Execute request
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(app.execute_request(
            app.selected_collection_index,
            app.selected_endpoint_index
        ));
    } else {
        // ... edit logic ...
    }
}
```

This works because:
1. Creates a tokio runtime
2. Calls `block_on()` to execute the async `execute_request()` method
3. Actually makes HTTP request via `HttpClient`
4. Updates `last_response` with results

### 'l' Key (Load Test) - DOESN'T WORK ❌

```rust
'l' => {
    app.start_load_test(coll_idx, ep_idx);  // Just creates engine, doesn't execute
}
```

This doesn't work because:
1. Only creates a `LoadTestEngine` object
2. Changes screen to show "running"
3. Never actually executes any HTTP requests
4. Metrics stay at zero

## Impact

- ❌ Load testing feature is completely non-functional
- ❌ Users see "Load test started" message but nothing happens
- ❌ Progress bar shows 0/0 requests
- ❌ Statistics remain unchanged
- ✅ Single request execution ('e' key) works fine

## Required Fix

### Solution: Implement Actual Load Test Execution

We need to add a method that actually executes requests in a loop, similar to how 'e' works but with concurrency.

**Option 1: Blocking Approach (Simple)**

Add to `src/tui_app.rs`:

```rust
pub fn start_load_test(&mut self, coll_idx: usize, ep_idx: usize) {
    if let Some(collection) = self.collections.get(coll_idx) {
        if let Some(endpoint) = collection.endpoints.get(ep_idx) {
            let endpoint = endpoint.clone();
            let config = self.load_test_config.clone();
            let http_client = self.http_client.clone();
            
            // Create engine
            match LoadTestEngine::new(config.clone()) {
                Ok(mut engine) => {
                    self.load_test_engine = Some(engine.clone());
                    self.current_screen = Screen::LoadTestRunning(coll_idx, ep_idx);
                    
                    // Actually execute requests
                    let runtime = tokio::runtime::Runtime::new().unwrap();
                    runtime.block_on(async {
                        let start = std::time::Instant::now();
                        let mut handles = vec![];
                        
                        // Spawn concurrent tasks
                        for _ in 0..config.concurrency {
                            let endpoint = endpoint.clone();
                            let http_client = http_client.clone();
                            let collector = engine.collector.clone();
                            let duration = config.duration;
                            
                            let handle = tokio::spawn(async move {
                                while start.elapsed() < duration {
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
                                }
                            });
                            
                            handles.push(handle);
                        }
                        
                        // Wait for all tasks
                        for handle in handles {
                            let _ = handle.await;
                        }
                    });
                    
                    self.status_message = Some("Load test completed".to_string());
                }
                Err(e) => {
                    self.error_message = Some(format!("Failed to start load test: {}", e));
                }
            }
        }
    }
}
```

**Pros**:
- Simple to implement
- Uses same pattern as 'e' key
- Will actually execute requests

**Cons**:
- UI freezes during load test (can't cancel, can't see progress)
- Not ideal UX but functional

**Option 2: Background Task (Better)**

Spawn a background thread that updates metrics while UI remains responsive:

```rust
pub fn start_load_test(&mut self, coll_idx: usize, ep_idx: usize) {
    if let Some(collection) = self.collections.get(coll_idx) {
        if let Some(endpoint) = collection.endpoints.get(ep_idx) {
            let endpoint = endpoint.clone();
            let config = self.load_test_config.clone();
            let http_client = self.http_client.clone();
            
            match LoadTestEngine::new(config.clone()) {
                Ok(engine) => {
                    let collector = engine.collector.clone();
                    let is_running = engine.is_running.clone();
                    
                    // Set running flag
                    *is_running.lock().unwrap() = true;
                    
                    // Spawn background thread
                    std::thread::spawn(move || {
                        let runtime = tokio::runtime::Runtime::new().unwrap();
                        runtime.block_on(async {
                            let start = std::time::Instant::now();
                            let mut handles = vec![];
                            
                            for _ in 0..config.concurrency {
                                let endpoint = endpoint.clone();
                                let http_client = http_client.clone();
                                let collector = collector.clone();
                                let is_running = is_running.clone();
                                let duration = config.duration;
                                
                                let handle = tokio::spawn(async move {
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
                                    }
                                });
                                
                                handles.push(handle);
                            }
                            
                            for handle in handles {
                                let _ = handle.await;
                            }
                            
                            // Mark as stopped
                            *is_running.lock().unwrap() = false;
                        });
                    });
                    
                    self.load_test_engine = Some(engine);
                    self.current_screen = Screen::LoadTestRunning(coll_idx, ep_idx);
                    self.status_message = Some("Load test started".to_string());
                }
                Err(e) => {
                    self.error_message = Some(format!("Failed to start load test: {}", e));
                }
            }
        }
    }
}
```

**Pros**:
- UI remains responsive
- Can see real-time progress
- Can cancel with Esc key
- Better UX

**Cons**:
- More complex
- Need to handle thread safety

## Additional Issues

### 1. HttpClient Not Cloneable

`HttpClient` needs to implement `Clone` for concurrent execution:

```rust
// In src/http.rs
#[derive(Clone)]
pub struct HttpClient {
    client: Client,  // reqwest::Client is already Clone
    default_timeout: Duration,
}
```

### 2. LoadTestEngine Needs Clone

```rust
// In src/load_test.rs
#[derive(Clone)]
pub struct LoadTestEngine {
    collector: MetricsCollector,  // Already Clone
    config: LoadTestConfig,  // Already Clone
    start_time: Option<Instant>,  // Not Clone - need Arc<Mutex<>>
    is_running: Arc<Mutex<bool>>,  // Already Clone-friendly
}
```

### 3. No UI for Load Test Progress

The `Screen::LoadTestRunning` screen needs to be implemented to show:
- Current metrics (total requests, success/failure)
- Current RPS
- Elapsed time
- Progress bar
- "Press Esc to stop" message

## Configuration Issue

Load test configuration is hardcoded:

```rust
load_test_config: LoadTestConfig::new(10, Duration::from_secs(30)),
```

**Concurrency**: 10 requests
**Duration**: 30 seconds

Users have no way to change these values without editing code.

**Needed**: Configuration UI screen before starting load test.

## Recommended Solution

### Phase 1: Quick Fix (4-6 hours)

1. Make `HttpClient` cloneable
2. Implement Option 1 (blocking approach) in `start_load_test()`
3. Add basic progress display in `LoadTestRunning` screen
4. Test with real endpoint

**Result**: Load test will work but UI will freeze during execution.

### Phase 2: Proper Fix (8-12 hours)

1. Implement Option 2 (background thread approach)
2. Add real-time metrics display
3. Add configuration UI screen
4. Add ability to cancel mid-test
5. Show final results after completion

**Result**: Professional, responsive load testing feature.

## Testing Plan

### Manual Test

1. Create collection
2. Add endpoint: `https://jsonplaceholder.typicode.com/users`
3. Select endpoint
4. Press 'l'
5. Should see:
   - Screen changes to "Load Test Running"
   - Requests being executed
   - Metrics updating (total, success, RPS)
   - After 30 seconds, test completes
6. Press Esc to return

### Expected Behavior

- Total requests: ~300 (10 concurrent * 30 seconds * ~1 req/sec each)
- Success rate: 100%
- RPS: ~10
- Latencies: 100-500ms (depending on network)

## Priority

**HIGH** - Core feature that is advertised but doesn't work.

## Estimated Effort

- **Phase 1 (Quick Fix)**: 4-6 hours
  - Make HttpClient cloneable: 30 min
  - Implement blocking execution: 2-3 hours
  - Add basic UI: 1-2 hours
  - Test and debug: 1 hour

- **Phase 2 (Proper Fix)**: 8-12 hours
  - Background thread implementation: 3-4 hours
  - Real-time UI updates: 2-3 hours
  - Configuration screen: 2-3 hours
  - Testing: 1-2 hours

## Summary

The load test feature is **partially implemented** but missing the critical execution logic. The keyboard handler exists, the data structures exist, but the actual HTTP request execution loop is missing. This is like having a car with an engine, steering wheel, and pedals, but no connection between the pedals and the engine.

**Fix**: Add the execution logic to `start_load_test()` method, similar to how `execute_request()` works but with concurrency and looping.


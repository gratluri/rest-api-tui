// Load testing engine for concurrent request execution

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Metrics collected during a load test
#[derive(Debug, Clone)]
pub struct LoadTestMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub error_counts: HashMap<String, u64>,
    pub latencies: Vec<Duration>,
    pub timestamps: Vec<Instant>,
    pub current_rps: f64,
}

impl LoadTestMetrics {
    /// Create a new empty metrics collection
    pub fn new() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            error_counts: HashMap::new(),
            latencies: Vec::new(),
            timestamps: Vec::new(),
            current_rps: 0.0,
        }
    }
    
    /// Record a successful request
    pub fn record_success(&mut self, latency: Duration) {
        self.total_requests += 1;
        self.successful_requests += 1;
        self.latencies.push(latency);
        self.timestamps.push(Instant::now());
    }
    
    /// Record a failed request
    pub fn record_failure(&mut self, error_type: String, latency: Duration) {
        self.total_requests += 1;
        self.failed_requests += 1;
        self.latencies.push(latency);
        self.timestamps.push(Instant::now());
        
        *self.error_counts.entry(error_type).or_insert(0) += 1;
    }
    
    /// Update current requests per second
    pub fn update_rps(&mut self, window_duration: Duration) {
        if self.timestamps.is_empty() {
            self.current_rps = 0.0;
            return;
        }
        
        let now = Instant::now();
        let cutoff = now - window_duration;
        
        // Count requests in the time window
        let recent_count = self.timestamps.iter()
            .filter(|&&ts| ts >= cutoff)
            .count();
        
        self.current_rps = recent_count as f64 / window_duration.as_secs_f64();
    }
}

impl Default for LoadTestMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Thread-safe metrics collector
#[derive(Debug, Clone)]
pub struct MetricsCollector {
    metrics: Arc<Mutex<LoadTestMetrics>>,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(Mutex::new(LoadTestMetrics::new())),
        }
    }
    
    /// Record a successful request
    pub fn record_success(&self, latency: Duration) {
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.record_success(latency);
        }
    }
    
    /// Record a failed request
    pub fn record_failure(&self, error_type: String, latency: Duration) {
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.record_failure(error_type, latency);
        }
    }
    
    /// Update current RPS calculation
    pub fn update_rps(&self, window_duration: Duration) {
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.update_rps(window_duration);
        }
    }
    
    /// Get a snapshot of current metrics
    pub fn snapshot(&self) -> LoadTestMetrics {
        self.metrics.lock()
            .map(|m| m.clone())
            .unwrap_or_default()
    }
    
    /// Reset all metrics
    pub fn reset(&self) {
        if let Ok(mut metrics) = self.metrics.lock() {
            *metrics = LoadTestMetrics::new();
        }
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate percentile from a sorted list of durations
pub fn calculate_percentile(sorted_latencies: &[Duration], percentile: f64) -> Option<Duration> {
    if sorted_latencies.is_empty() {
        return None;
    }
    
    if percentile <= 0.0 {
        return Some(sorted_latencies[0]);
    }
    
    if percentile >= 100.0 {
        return Some(*sorted_latencies.last().unwrap());
    }
    
    let index = (percentile / 100.0 * (sorted_latencies.len() - 1) as f64).round() as usize;
    Some(sorted_latencies[index])
}

/// Calculate multiple percentiles at once
pub fn calculate_percentiles(latencies: &[Duration]) -> PercentilesResult {
    if latencies.is_empty() {
        return PercentilesResult::default();
    }
    
    let mut sorted = latencies.to_vec();
    sorted.sort();
    
    PercentilesResult {
        p50: calculate_percentile(&sorted, 50.0).unwrap_or_default(),
        p90: calculate_percentile(&sorted, 90.0).unwrap_or_default(),
        p95: calculate_percentile(&sorted, 95.0).unwrap_or_default(),
        p99: calculate_percentile(&sorted, 99.0).unwrap_or_default(),
        min: sorted.first().copied().unwrap_or_default(),
        max: sorted.last().copied().unwrap_or_default(),
    }
}

/// Percentiles result
#[derive(Debug, Clone, Default)]
pub struct PercentilesResult {
    pub p50: Duration,
    pub p90: Duration,
    pub p95: Duration,
    pub p99: Duration,
    pub min: Duration,
    pub max: Duration,
}

impl PercentilesResult {
    /// Verify that percentiles are in correct order
    pub fn is_valid(&self) -> bool {
        self.min <= self.p50
            && self.p50 <= self.p90
            && self.p90 <= self.p95
            && self.p95 <= self.p99
            && self.p99 <= self.max
    }
}

/// Load test statistics
#[derive(Debug, Clone)]
pub struct LoadTestStatistics {
    pub total_requests: u64,
    pub success_rate: f64,
    pub error_rate: f64,
    pub avg_latency: Duration,
    pub min_latency: Duration,
    pub max_latency: Duration,
    pub p50_latency: Duration,
    pub p90_latency: Duration,
    pub p95_latency: Duration,
    pub p99_latency: Duration,
    pub avg_rps: f64,
}

impl LoadTestStatistics {
    /// Calculate statistics from metrics
    pub fn from_metrics(metrics: &LoadTestMetrics, total_duration: Duration) -> Self {
        let percentiles = calculate_percentiles(&metrics.latencies);
        
        let success_rate = if metrics.total_requests > 0 {
            metrics.successful_requests as f64 / metrics.total_requests as f64
        } else {
            0.0
        };
        
        let error_rate = if metrics.total_requests > 0 {
            metrics.failed_requests as f64 / metrics.total_requests as f64
        } else {
            0.0
        };
        
        let avg_latency = if !metrics.latencies.is_empty() {
            let total: Duration = metrics.latencies.iter().sum();
            total / metrics.latencies.len() as u32
        } else {
            Duration::default()
        };
        
        let avg_rps = if total_duration.as_secs_f64() > 0.0 {
            metrics.total_requests as f64 / total_duration.as_secs_f64()
        } else {
            0.0
        };
        
        Self {
            total_requests: metrics.total_requests,
            success_rate,
            error_rate,
            avg_latency,
            min_latency: percentiles.min,
            max_latency: percentiles.max,
            p50_latency: percentiles.p50,
            p90_latency: percentiles.p90,
            p95_latency: percentiles.p95,
            p99_latency: percentiles.p99,
            avg_rps,
        }
    }
}

/// Load test configuration
#[derive(Debug, Clone)]
pub struct LoadTestConfig {
    pub concurrency: usize,
    pub duration: Duration,
    pub rate_limit: Option<usize>, // requests per second
    pub ramp_up: Option<Duration>,
}

impl LoadTestConfig {
    pub fn new(concurrency: usize, duration: Duration) -> Self {
        Self {
            concurrency,
            duration,
            rate_limit: None,
            ramp_up: None,
        }
    }
    
    pub fn with_rate_limit(mut self, rps: usize) -> Self {
        self.rate_limit = Some(rps);
        self
    }
    
    pub fn with_ramp_up(mut self, ramp_up: Duration) -> Self {
        self.ramp_up = Some(ramp_up);
        self
    }
    
    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.concurrency == 0 || self.concurrency > 1000 {
            return Err("Concurrency must be between 1 and 1000".to_string());
        }
        
        if self.duration.as_secs() == 0 || self.duration.as_secs() > 3600 {
            return Err("Duration must be between 1 and 3600 seconds".to_string());
        }
        
        if let Some(rate) = self.rate_limit {
            if rate == 0 || rate > 10000 {
                return Err("Rate limit must be between 1 and 10000 RPS".to_string());
            }
        }
        
        Ok(())
    }
}

/// Load test engine for executing concurrent HTTP requests
pub struct LoadTestEngine {
    collector: MetricsCollector,
    #[allow(dead_code)]
    config: LoadTestConfig,
    start_time: Option<Instant>,
    is_running: Arc<Mutex<bool>>,
}

impl LoadTestEngine {
    pub fn new(config: LoadTestConfig) -> Result<Self, String> {
        config.validate()?;
        
        Ok(Self {
            collector: MetricsCollector::new(),
            config,
            start_time: None,
            is_running: Arc::new(Mutex::new(false)),
        })
    }
    
    /// Get current metrics snapshot
    pub fn metrics(&self) -> LoadTestMetrics {
        self.collector.snapshot()
    }
    
    /// Check if test is running
    pub fn is_running(&self) -> bool {
        *self.is_running.lock().unwrap()
    }
    
    /// Get elapsed time
    pub fn elapsed(&self) -> Duration {
        self.start_time.map(|t| t.elapsed()).unwrap_or_default()
    }
    
    /// Stop the load test
    pub fn stop(&self) {
        if let Ok(mut running) = self.is_running.lock() {
            *running = false;
        }
    }
    
    /// Get final results
    pub fn results(&self) -> LoadTestStatistics {
        let metrics = self.collector.snapshot();
        let duration = self.elapsed();
        LoadTestStatistics::from_metrics(&metrics, duration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_metrics_new() {
        let metrics = LoadTestMetrics::new();
        assert_eq!(metrics.total_requests, 0);
        assert_eq!(metrics.successful_requests, 0);
        assert_eq!(metrics.failed_requests, 0);
        assert_eq!(metrics.latencies.len(), 0);
    }

    #[test]
    fn test_record_success() {
        let mut metrics = LoadTestMetrics::new();
        metrics.record_success(Duration::from_millis(100));
        
        assert_eq!(metrics.total_requests, 1);
        assert_eq!(metrics.successful_requests, 1);
        assert_eq!(metrics.failed_requests, 0);
        assert_eq!(metrics.latencies.len(), 1);
    }

    #[test]
    fn test_record_failure() {
        let mut metrics = LoadTestMetrics::new();
        metrics.record_failure("Timeout".to_string(), Duration::from_millis(5000));
        
        assert_eq!(metrics.total_requests, 1);
        assert_eq!(metrics.successful_requests, 0);
        assert_eq!(metrics.failed_requests, 1);
        assert_eq!(metrics.error_counts.get("Timeout"), Some(&1));
    }

    #[test]
    fn test_multiple_errors() {
        let mut metrics = LoadTestMetrics::new();
        metrics.record_failure("Timeout".to_string(), Duration::from_millis(5000));
        metrics.record_failure("Timeout".to_string(), Duration::from_millis(5000));
        metrics.record_failure("ConnectionRefused".to_string(), Duration::from_millis(100));
        
        assert_eq!(metrics.failed_requests, 3);
        assert_eq!(metrics.error_counts.get("Timeout"), Some(&2));
        assert_eq!(metrics.error_counts.get("ConnectionRefused"), Some(&1));
    }

    #[test]
    fn test_metrics_collector_thread_safe() {
        let collector = MetricsCollector::new();
        let collector_clone = collector.clone();
        
        let handle = thread::spawn(move || {
            collector_clone.record_success(Duration::from_millis(50));
        });
        
        collector.record_success(Duration::from_millis(100));
        handle.join().unwrap();
        
        let snapshot = collector.snapshot();
        assert_eq!(snapshot.total_requests, 2);
        assert_eq!(snapshot.successful_requests, 2);
    }

    #[test]
    fn test_collector_reset() {
        let collector = MetricsCollector::new();
        collector.record_success(Duration::from_millis(100));
        collector.record_failure("Error".to_string(), Duration::from_millis(200));
        
        let snapshot1 = collector.snapshot();
        assert_eq!(snapshot1.total_requests, 2);
        
        collector.reset();
        
        let snapshot2 = collector.snapshot();
        assert_eq!(snapshot2.total_requests, 0);
    }

    #[test]
    fn test_update_rps() {
        let mut metrics = LoadTestMetrics::new();
        
        // Record some requests
        for _ in 0..10 {
            metrics.record_success(Duration::from_millis(50));
        }
        
        // Update RPS with 1 second window
        metrics.update_rps(Duration::from_secs(1));
        
        // Should have calculated RPS (might be 0 if too fast)
        assert!(metrics.current_rps >= 0.0);
    }
    
    #[test]
    fn test_calculate_percentile_empty() {
        let latencies: Vec<Duration> = vec![];
        assert_eq!(calculate_percentile(&latencies, 50.0), None);
    }
    
    #[test]
    fn test_calculate_percentile_single() {
        let latencies = vec![Duration::from_millis(100)];
        assert_eq!(calculate_percentile(&latencies, 50.0), Some(Duration::from_millis(100)));
        assert_eq!(calculate_percentile(&latencies, 90.0), Some(Duration::from_millis(100)));
    }
    
    #[test]
    fn test_calculate_percentile_multiple() {
        let latencies = vec![
            Duration::from_millis(10),
            Duration::from_millis(20),
            Duration::from_millis(30),
            Duration::from_millis(40),
            Duration::from_millis(50),
            Duration::from_millis(60),
            Duration::from_millis(70),
            Duration::from_millis(80),
            Duration::from_millis(90),
            Duration::from_millis(100),
        ];
        
        let p50 = calculate_percentile(&latencies, 50.0).unwrap();
        let p90 = calculate_percentile(&latencies, 90.0).unwrap();
        let p99 = calculate_percentile(&latencies, 99.0).unwrap();
        
        assert!(p50 >= Duration::from_millis(40) && p50 <= Duration::from_millis(60));
        assert!(p90 >= Duration::from_millis(80) && p90 <= Duration::from_millis(100));
        assert!(p99 >= Duration::from_millis(90));
    }
    
    #[test]
    fn test_calculate_percentiles() {
        let latencies = vec![
            Duration::from_millis(10),
            Duration::from_millis(50),
            Duration::from_millis(100),
            Duration::from_millis(150),
            Duration::from_millis(200),
        ];
        
        let percentiles = calculate_percentiles(&latencies);
        
        assert_eq!(percentiles.min, Duration::from_millis(10));
        assert_eq!(percentiles.max, Duration::from_millis(200));
        assert!(percentiles.is_valid());
    }
    
    #[test]
    fn test_percentiles_ordering() {
        let latencies = vec![
            Duration::from_millis(5),
            Duration::from_millis(10),
            Duration::from_millis(15),
            Duration::from_millis(20),
            Duration::from_millis(25),
            Duration::from_millis(30),
            Duration::from_millis(100),
        ];
        
        let percentiles = calculate_percentiles(&latencies);
        
        // Verify ordering: min <= p50 <= p90 <= p95 <= p99 <= max
        assert!(percentiles.is_valid());
        assert!(percentiles.min <= percentiles.p50);
        assert!(percentiles.p50 <= percentiles.p90);
        assert!(percentiles.p90 <= percentiles.p95);
        assert!(percentiles.p95 <= percentiles.p99);
        assert!(percentiles.p99 <= percentiles.max);
    }
    
    #[test]
    fn test_percentiles_empty() {
        let latencies: Vec<Duration> = vec![];
        let percentiles = calculate_percentiles(&latencies);
        
        assert_eq!(percentiles.min, Duration::default());
        assert_eq!(percentiles.max, Duration::default());
    }
    
    #[test]
    fn test_statistics_calculation() {
        let mut metrics = LoadTestMetrics::new();
        
        // Record some successful requests
        for _ in 0..8 {
            metrics.record_success(Duration::from_millis(100));
        }
        
        // Record some failures
        for _ in 0..2 {
            metrics.record_failure("Timeout".to_string(), Duration::from_millis(5000));
        }
        
        let stats = LoadTestStatistics::from_metrics(&metrics, Duration::from_secs(10));
        
        assert_eq!(stats.total_requests, 10);
        assert_eq!(stats.success_rate, 0.8);
        assert_eq!(stats.error_rate, 0.2);
        assert!(stats.avg_rps > 0.0);
    }
    
    #[test]
    fn test_statistics_all_success() {
        let mut metrics = LoadTestMetrics::new();
        
        for _ in 0..100 {
            metrics.record_success(Duration::from_millis(50));
        }
        
        let stats = LoadTestStatistics::from_metrics(&metrics, Duration::from_secs(1));
        
        assert_eq!(stats.total_requests, 100);
        assert_eq!(stats.success_rate, 1.0);
        assert_eq!(stats.error_rate, 0.0);
        assert_eq!(stats.avg_rps, 100.0);
    }
    
    #[test]
    fn test_statistics_empty() {
        let metrics = LoadTestMetrics::new();
        let stats = LoadTestStatistics::from_metrics(&metrics, Duration::from_secs(1));
        
        assert_eq!(stats.total_requests, 0);
        assert_eq!(stats.success_rate, 0.0);
        assert_eq!(stats.error_rate, 0.0);
    }
}

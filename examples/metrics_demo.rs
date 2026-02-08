use rest_api_tui::load_test::{
    LoadTestMetrics, MetricsCollector, calculate_percentiles, LoadTestStatistics
};
use std::time::Duration;

fn main() {
    println!("=== REST API TUI - Load Test Metrics Demo ===\n");
    
    // Example 1: Basic metrics collection
    println!("📝 Example 1: Basic Metrics Collection");
    let mut metrics = LoadTestMetrics::new();
    
    println!("Initial state:");
    println!("  Total requests: {}", metrics.total_requests);
    println!("  Successful: {}", metrics.successful_requests);
    println!("  Failed: {}", metrics.failed_requests);
    println!();
    
    // Simulate some successful requests
    for i in 0..10 {
        let latency = Duration::from_millis(50 + i * 10);
        metrics.record_success(latency);
    }
    
    println!("After 10 successful requests:");
    println!("  Total requests: {}", metrics.total_requests);
    println!("  Successful: {}", metrics.successful_requests);
    println!("  Failed: {}", metrics.failed_requests);
    println!("  Latencies recorded: {}", metrics.latencies.len());
    println!();
    
    // Example 2: Recording failures
    println!("📝 Example 2: Recording Failures");
    metrics.record_failure("Timeout".to_string(), Duration::from_millis(5000));
    metrics.record_failure("Timeout".to_string(), Duration::from_millis(5000));
    metrics.record_failure("ConnectionRefused".to_string(), Duration::from_millis(100));
    metrics.record_failure("500 Internal Server Error".to_string(), Duration::from_millis(200));
    
    println!("After recording failures:");
    println!("  Total requests: {}", metrics.total_requests);
    println!("  Successful: {}", metrics.successful_requests);
    println!("  Failed: {}", metrics.failed_requests);
    println!("\nError breakdown:");
    for (error_type, count) in &metrics.error_counts {
        println!("    {}: {}", error_type, count);
    }
    println!();
    
    // Example 3: Percentile calculation
    println!("📝 Example 3: Percentile Calculation");
    let percentiles = calculate_percentiles(&metrics.latencies);
    
    println!("Latency percentiles:");
    println!("  Min: {:?}", percentiles.min);
    println!("  P50 (median): {:?}", percentiles.p50);
    println!("  P90: {:?}", percentiles.p90);
    println!("  P95: {:?}", percentiles.p95);
    println!("  P99: {:?}", percentiles.p99);
    println!("  Max: {:?}", percentiles.max);
    println!("\nPercentiles are valid (correctly ordered): {}", percentiles.is_valid());
    println!();
    
    // Example 4: Statistics calculation
    println!("📝 Example 4: Statistics Calculation");
    let stats = LoadTestStatistics::from_metrics(&metrics, Duration::from_secs(10));
    
    println!("Load test statistics:");
    println!("  Total requests: {}", stats.total_requests);
    println!("  Success rate: {:.2}%", stats.success_rate * 100.0);
    println!("  Error rate: {:.2}%", stats.error_rate * 100.0);
    println!("  Average latency: {:?}", stats.avg_latency);
    println!("  Average RPS: {:.2}", stats.avg_rps);
    println!();
    
    // Example 5: Thread-safe metrics collector
    println!("📝 Example 5: Thread-Safe Metrics Collector");
    let collector = MetricsCollector::new();
    
    // Simulate concurrent requests
    use std::thread;
    let mut handles = vec![];
    
    for i in 0..5 {
        let collector_clone = collector.clone();
        let handle = thread::spawn(move || {
            for j in 0..10 {
                let latency = Duration::from_millis(50 + (i * 10 + j) as u64);
                collector_clone.record_success(latency);
            }
        });
        handles.push(handle);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    let snapshot = collector.snapshot();
    println!("After concurrent collection:");
    println!("  Total requests: {}", snapshot.total_requests);
    println!("  Successful: {}", snapshot.successful_requests);
    println!("  Thread-safe: ✓");
    println!();
    
    // Example 6: Real-world simulation
    println!("📝 Example 6: Real-World Load Test Simulation");
    let mut sim_metrics = LoadTestMetrics::new();
    
    // Simulate a 60-second load test with varying latencies
    println!("Simulating 60-second load test with 100 requests...");
    
    for i in 0..100 {
        if i % 20 == 0 {
            // Occasional timeout
            sim_metrics.record_failure("Timeout".to_string(), Duration::from_millis(30000));
        } else if i % 15 == 0 {
            // Occasional server error
            sim_metrics.record_failure("500 Internal Server Error".to_string(), Duration::from_millis(500));
        } else {
            // Normal request with varying latency
            let base_latency = 100;
            let variance = (i % 10) * 20;
            sim_metrics.record_success(Duration::from_millis(base_latency + variance));
        }
    }
    
    let sim_stats = LoadTestStatistics::from_metrics(&sim_metrics, Duration::from_secs(60));
    let sim_percentiles = calculate_percentiles(&sim_metrics.latencies);
    
    println!("\nLoad Test Results:");
    println!("  Duration: 60 seconds");
    println!("  Total requests: {}", sim_stats.total_requests);
    println!("  Successful: {} ({:.1}%)", sim_metrics.successful_requests, sim_stats.success_rate * 100.0);
    println!("  Failed: {} ({:.1}%)", sim_metrics.failed_requests, sim_stats.error_rate * 100.0);
    println!("  Average RPS: {:.2}", sim_stats.avg_rps);
    println!("\nLatency Statistics:");
    println!("  Average: {:?}", sim_stats.avg_latency);
    println!("  Min: {:?}", sim_percentiles.min);
    println!("  P50: {:?}", sim_percentiles.p50);
    println!("  P90: {:?}", sim_percentiles.p90);
    println!("  P95: {:?}", sim_percentiles.p95);
    println!("  P99: {:?}", sim_percentiles.p99);
    println!("  Max: {:?}", sim_percentiles.max);
    println!("\nError Breakdown:");
    for (error_type, count) in &sim_metrics.error_counts {
        let percentage = (*count as f64 / sim_metrics.total_requests as f64) * 100.0;
        println!("  {}: {} ({:.1}%)", error_type, count, percentage);
    }
    println!();
    
    // Example 7: Metrics consistency validation
    println!("📝 Example 7: Metrics Consistency Validation");
    let total = sim_metrics.successful_requests + sim_metrics.failed_requests;
    let matches = total == sim_metrics.total_requests;
    
    println!("Consistency check:");
    println!("  Successful: {}", sim_metrics.successful_requests);
    println!("  Failed: {}", sim_metrics.failed_requests);
    println!("  Sum: {}", total);
    println!("  Total: {}", sim_metrics.total_requests);
    println!("  Consistent: {} {}", if matches { "✓" } else { "✗" }, if matches { "(PASS)" } else { "(FAIL)" });
    println!();
    
    println!("=== Demo Complete ===");
    println!("\n✨ Key Features:");
    println!("  • Thread-safe metrics collection");
    println!("  • Success/failure tracking");
    println!("  • Error type classification");
    println!("  • Percentile calculation (P50, P90, P95, P99)");
    println!("  • Statistics computation (success rate, avg latency, RPS)");
    println!("  • Metrics consistency validation");
}

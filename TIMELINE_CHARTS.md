# Timeline Charts Feature

## Overview

The load test now includes real-time timeline charts showing latency and RPS trends over time.

## Features

### Time-Series Data Collection
- **Granularity**: 5-second intervals
- **History**: Last 60 seconds (12 data points)
- **Metrics Tracked**:
  - p50, p90, p95, p99 latency percentiles
  - Requests per second (RPS)
  - Request count per interval

### Visualization

**Two Sparkline Charts:**

1. **p95 Latency Chart**
   - Shows p95 latency trend over last 60 seconds
   - Color-coded:
     - Green: <100ms (good)
     - Yellow: 100-200ms (warning)
     - Red: >200ms (critical)
   - Updates every 5 seconds

2. **RPS Chart**
   - Shows requests per second over last 60 seconds
   - Cyan color
   - Updates every 5 seconds

### Layout

```
┌─ Load Test Progress ─────────────────────────────────┐
│ ████████████████████████████░░░░░░░░░░░░░░░░░░ 60%   │
└───────────────────────────────────────────────────────┘

┌─ Statistics ──────────────┬─ p95 Latency (ms) ───────┐
│ Total Requests: 1,247     │ ▁▂▃▄▅▆▇█▇▆▅▄▃▂▁          │
│ Successful: 1,245         │                          │
│ Failed: 2                 │                          │
│ Current RPS: 19.87        ├─ RPS (req/s) ────────────┤
│                           │ ▃▄▅▆▇█▇▆▅▄▃▃▃▃▃          │
│ Latency Percentiles:      │                          │
│   Avg: 45ms   p50: 42ms   │                          │
│   p90: 58ms   p95: 64ms   │                          │
│   p99: 78ms   Max: 95ms   │                          │
└───────────────────────────┴──────────────────────────┘

┌─ Results ─────────────────────────────────────────────┐
│     Success ████████████████████ 1245                 │
│     Failed  ▌ 2                                       │
└───────────────────────────────────────────────────────┘
```


## Technical Implementation

### Data Structures

**TimeSeriesDataPoint** (in `load_test.rs`):
```rust
pub struct TimeSeriesDataPoint {
    pub timestamp: Instant,
    pub elapsed_secs: u64,
    pub rps: f64,
    pub p50: Duration,
    pub p90: Duration,
    pub p95: Duration,
    pub p99: Duration,
    pub request_count: u64,
}
```

**LoadTestMetrics** (updated):
```rust
pub struct LoadTestMetrics {
    // ... existing fields ...
    pub time_series: Vec<TimeSeriesDataPoint>,  // NEW
}
```

### Data Collection

**Periodic Sampling** (every 5 seconds):
1. Calculate current percentiles from all latencies
2. Get current RPS
3. Create TimeSeriesDataPoint
4. Add to time_series vector
5. Keep only last 12 points (60 seconds)

**Implementation** (in `tui_app.rs`):
```rust
// Spawn background task for time-series collection
tokio::spawn(async move {
    while *is_running.lock().unwrap() {
        collector.add_time_series_point(start);
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
});
```

### Visualization

**Sparkline Widget** (ratatui):
- Converts time-series data to u64 array
- Renders as ASCII sparkline
- Auto-scales to fit data range
- Color-coded based on thresholds

**Layout** (in `draw_load_test`):
- Split statistics area horizontally (50/50)
- Left: Text statistics
- Right: Two sparkline charts (p95 latency, RPS)

## Benefits

1. **Trend Visibility**: See performance over time, not just current state
2. **Degradation Detection**: Spot when latency increases
3. **Ramp-up Visualization**: See when load stabilizes
4. **Pattern Recognition**: Identify periodic issues
5. **Historical Context**: Compare current vs recent performance

## Usage

No user action required - charts appear automatically during load tests once 5+ seconds have elapsed.

## Performance Impact

- **Memory**: ~1KB per data point × 12 points = ~12KB
- **CPU**: Minimal - calculations every 5 seconds
- **No impact on test accuracy**: Runs in separate thread

## Future Enhancements

- Toggle between different percentiles (p50/p90/p95/p99)
- Adjustable time window (30s, 60s, 120s)
- Export chart data to CSV
- Overlay error rate on RPS chart
- Zoom/pan functionality

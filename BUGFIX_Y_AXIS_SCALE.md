# Bug Fix: Y-Axis Scale Labels for Sparkline Charts

## Issue

The sparkline charts for p95 latency and RPS did not show Y-axis scale numbers, making it difficult to interpret the actual values.

## Root Cause

The ratatui `Sparkline` widget does not support Y-axis labels natively. It only renders the sparkline visualization without numerical scales.

## Solution

Added min/max scale values to the chart titles, so users can see the range at a glance.

### Before
```
┌─ p95 Latency (ms) ───────┐
│ ▁▂▃▄▅▆▇█▇▆▅▄▃▂▁          │
│                          │
└──────────────────────────┘
```

### After
```
┌─ p95 Latency: 42ms - 89ms ─┐
│ ▁▂▃▄▅▆▇█▇▆▅▄▃▂▁            │
│                            │
└────────────────────────────┘
```

## Implementation

Updated `draw_load_test()` in `src/tui/ui.rs`:

**p95 Latency Chart:**
```rust
let max_p95 = *p95_data.iter().max().unwrap_or(&1);
let min_p95 = *p95_data.iter().min().unwrap_or(&0);
let p95_title = format!("p95 Latency: {}ms - {}ms", min_p95, max_p95);

let p95_sparkline = Sparkline::default()
    .block(Block::default().title(p95_title).borders(Borders::ALL))
    .data(&p95_data)
    .style(sparkline_style);
```

**RPS Chart:**
```rust
let max_rps = *rps_data.iter().max().unwrap_or(&1);
let min_rps = *rps_data.iter().min().unwrap_or(&0);
let rps_title = format!("RPS: {} - {} req/s", min_rps, max_rps);

let rps_sparkline = Sparkline::default()
    .block(Block::default().title(rps_title).borders(Borders::ALL))
    .data(&rps_data)
    .style(Style::default().fg(Color::Cyan));
```

## Benefits

1. **Clear Scale**: Users can immediately see the min/max values
2. **Context**: Easier to interpret the sparkline shape
3. **No Guessing**: Exact numbers shown in title
4. **Compact**: Doesn't take extra space

## Example Output

```
┌─ Statistics ──────────────┬─ p95 Latency: 42ms - 89ms ─┐
│ Total Requests: 1,247     │ ▁▂▃▄▅▆▇█▇▆▅▄▃▂▁            │
│ Successful: 1,245         │                            │
│ Failed: 2                 │                            │
│ Current RPS: 19.87        ├─ RPS: 15 - 22 req/s ───────┤
│                           │ ▃▄▅▆▇█▇▆▅▄▃▃▃▃▃            │
│ Latency Percentiles:      │                            │
│   Avg: 45ms   p50: 42ms   │                            │
│   p90: 58ms   p95: 64ms   │                            │
│   p99: 78ms   Max: 95ms   │                            │
└───────────────────────────┴────────────────────────────┘
```

## Testing

- ✅ Build successful
- ✅ No diagnostic errors
- ✅ Scale values update dynamically
- ✅ Min/max calculated correctly

## Files Changed

- `src/tui/ui.rs` - Updated sparkline title generation

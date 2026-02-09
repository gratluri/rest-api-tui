# Load Test Configuration - Quick Reference

## Keyboard Shortcuts

### Configuration Form
| Key | Action |
|-----|--------|
| `0-9` | Type digits into current field |
| `Tab` | Move to next field (concurrency → duration → ramp-up → concurrency) |
| `Shift+Tab` | Move to previous field (backwards) |
| `Backspace` | Delete last character from current field |
| `Enter` | Start load test with configured parameters |
| `Esc` | Cancel and return to main screen |

### Load Test Running
| Key | Action |
|-----|--------|
| `Esc` | Stop test early and return to main screen |

## Configuration Fields

### Concurrency (Workers)
- **Range**: 1-1000
- **Default**: 10
- **Description**: Number of concurrent workers executing requests
- **Example**: 20 = 20 workers running in parallel

### Duration (Seconds)
- **Range**: 1-3600
- **Default**: 30
- **Description**: Total test duration in seconds
- **Example**: 60 = test runs for 1 minute

### Ramp-up (Seconds)
- **Range**: 0 to (duration - 1)
- **Default**: None (optional)
- **Description**: Time to gradually increase load from 0 to full concurrency
- **Example**: 10 = workers start gradually over 10 seconds

## Metrics Displayed

### Basic Metrics
- **Total Requests**: Total number of HTTP requests sent
- **Successful**: Number of successful requests (2xx, 3xx status codes)
- **Failed**: Number of failed requests (4xx, 5xx, timeouts, errors)
- **Current RPS**: Requests per second (updates every 500ms)

### Latency Percentiles
- **Avg**: Average (mean) latency across all requests
- **p50**: Median latency (50% of requests faster than this)
- **p90**: 90th percentile (90% of requests faster than this)
- **p95**: 95th percentile (95% of requests faster than this)
- **p99**: 99th percentile (99% of requests faster than this)
- **Max**: Maximum latency (slowest request)

## Example Usage

### Quick Test (Default Settings)
1. Press `l` on endpoint
2. Press `Enter` (uses defaults: 10 workers, 30 seconds)

### Custom Test
1. Press `l` on endpoint
2. Type `50` (concurrency)
3. Press `Tab`
4. Type `120` (duration = 2 minutes)
5. Press `Tab`
6. Type `20` (ramp-up = 20 seconds)
7. Press `Enter`

Result: 50 workers, ramping up over 20 seconds, running for 2 minutes

### Editing Values
1. Press `l` on endpoint
2. Type `100`
3. Press `Backspace` twice → "1"
4. Type `5` → "15"
5. Press `Tab` to continue

## Understanding Percentiles

### What do percentiles mean?

**p50 (Median)**: Half of your requests are faster than this
- Example: p50 = 50ms means 50% of requests took less than 50ms

**p90**: 90% of requests are faster than this
- Example: p90 = 100ms means only 10% of requests took more than 100ms

**p95**: 95% of requests are faster than this
- Example: p95 = 150ms means only 5% of requests took more than 150ms

**p99**: 99% of requests are faster than this
- Example: p99 = 300ms means only 1% of requests took more than 300ms

### Why percentiles matter?

- **Average can be misleading**: A few slow requests can skew the average
- **Percentiles show distribution**: p95 and p99 reveal outliers
- **SLA targets**: Many APIs target p95 or p99 latency (e.g., "p95 < 200ms")

### Example Interpretation

```
Avg: 45ms   p50: 42ms
p90: 58ms   p95: 64ms
p99: 78ms   Max: 95ms
```

**What this tells you:**
- Most requests (50%) complete in ~42ms
- 90% complete in under 58ms
- 95% complete in under 64ms
- Only 1% take more than 78ms
- Worst case was 95ms

**Is this good?**
- Depends on your API's requirements
- If target is "p95 < 100ms", you're meeting it (64ms < 100ms)
- If target is "p99 < 50ms", you're not meeting it (78ms > 50ms)

## Ramp-up Behavior

### Without Ramp-up
```
Time:     0s    5s    10s   15s   20s
Workers:  20    20    20    20    20
```
All 20 workers start immediately at t=0

### With 10s Ramp-up
```
Time:     0s    2.5s  5s    7.5s  10s   15s   20s
Workers:  1     5     10    15    20    20    20
```
Workers start gradually:
- 0-10s: Ramp from 1 to 20 workers
- 10s+: All 20 workers running

### Why use ramp-up?
- **Avoid thundering herd**: Don't overwhelm server at start
- **Realistic load**: Simulates gradual traffic increase
- **Identify breaking point**: See when performance degrades

## Tips

### Finding the Right Concurrency
1. Start with 10 workers
2. If p95 < target, increase to 20
3. Keep increasing until p95 exceeds target
4. That's your maximum sustainable load

### Testing for Stability
- Use longer duration (300-600 seconds)
- Watch for latency creep (p95 increasing over time)
- Check for memory leaks or resource exhaustion

### Testing for Burst Capacity
- Use high concurrency (100+)
- Short duration (30-60 seconds)
- No ramp-up (immediate load)
- See how API handles sudden traffic spike

## Troubleshooting

### RPS is 0
- **Fixed in this version!** RPS now updates every 500ms
- If still 0, check if requests are actually being sent

### All requests failing
- Check endpoint URL is correct
- Check network connectivity
- Check API is running and accessible

### Latencies very high
- Check network latency (try `ping` to server)
- Check server load (might be overloaded)
- Try reducing concurrency

### Can't type in form
- **Fixed in this version!** Form now accepts digit input
- Make sure you're in the right field (highlighted in yellow)

## Color Guide

### Configuration Form
- **Yellow**: Current field (where input goes)
- **White**: Other fields
- **Cyan**: Field labels
- **Dark Gray**: Help text

### Load Test Running
- **Green**: Success metrics, p50 (good performance)
- **Yellow**: p90 (warning threshold)
- **Magenta**: p95 (attention needed)
- **Red**: p99, Max, Failed requests (critical)
- **White**: Total requests, Avg latency

## Validation Rules

### Concurrency
- Must be 1-1000
- Error if outside range

### Duration
- Must be 1-3600 seconds (1 hour max)
- Error if outside range

### Ramp-up
- Must be less than duration
- Error if ramp-up >= duration
- Optional (can be empty)

## Saved Configuration

Configuration is saved to the endpoint after first use:
1. Configure and run test
2. Configuration is saved to endpoint
3. Next time you press `l`, form shows saved values
4. Edit and save again to update

This allows you to:
- Rerun tests with same parameters
- Compare results over time
- Share configurations with team (via collection export)

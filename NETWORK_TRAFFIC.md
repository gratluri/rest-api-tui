# Network Traffic Tracking (Wireshark-style)

## Overview

The REST API TUI now includes optional network traffic tracking, similar to Wireshark or browser DevTools Network tab. This feature provides detailed insights into HTTP requests and responses, including timing breakdowns and data transfer sizes.

## Features

### What's Captured

1. **Timing Breakdown**:
   - Request Sent: Time to send the HTTP request
   - Waiting (TTFB): Time to First Byte - server processing time
   - Content Download: Time to download the response body
   - Total: Complete request/response cycle time

2. **Request Details**:
   - HTTP method and full URL
   - All request headers
   - Request body size
   - Total request size

3. **Response Details**:
   - HTTP status code
   - Response headers count and size
   - Response body size
   - Total response size

4. **Transfer Summary**:
   - Total bytes transferred (request + response)

### Future Enhancements

The following timing details are not currently captured but could be added with custom HTTP connectors:
- DNS Lookup time
- TCP Connect time
- TLS Handshake time

## How to Use

### Toggle Network Traffic View

Press **'t'** at any time to toggle the network traffic display on/off.

- **Default**: Network traffic is **hidden**
- **When enabled**: Response panel splits into two sections:
  - Top 50%: Response body (formatted JSON/XML/plain text)
  - Bottom 50%: Network traffic details

### Workflow

1. **Execute a request** (press 'e' on a selected endpoint)
2. **View the response** in the response panel
3. **Press 't'** to show network traffic details
4. **Press 't' again** to hide and see full response body

## Visual Layout

### With Network Traffic Hidden (Default)

```
┌─────────────────────────────────────────────────────────┐
│ Response: 200 OK - 123ms - 456 bytes  [t: show traffic]│
├─────────────────────────────────────────────────────────┤
│ {                                                       │
│   "id": 1,                                              │
│   "name": "John Doe",                                   │
│   "email": "john@example.com"                           │
│ }                                                       │
│                                                         │
│ (full height for response body)                        │
└─────────────────────────────────────────────────────────┘
```

### With Network Traffic Shown

```
┌─────────────────────────────────────────────────────────┐
│ Response: 200 OK - 123ms - 456 bytes  [t: hide traffic]│
├─────────────────────────────────────────────────────────┤
│ {                                                       │
│   "id": 1,                                              │
│   "name": "John Doe"                                    │
│ }                                                       │
├─────────────────────────────────────────────────────────┤
│ ▼ Network Traffic (Wireshark-style)                    │
├─────────────────────────────────────────────────────────┤
│ Timing Breakdown:                                       │
│   Request Sent:      1ms                                │
│   Waiting (TTFB):    48ms                               │
│   Content Download:  11ms                               │
│   Total:             123ms                              │
│                                                         │
│ Request:                                                │
│   GET http://api.example.com/users/1                    │
│   Headers: 3 (145 bytes)                                │
│     Content-Type: application/json                      │
│     Authorization: Bearer eyJhbGc...                    │
│     User-Agent: rest-api-tui/0.1.0                      │
│   Body: 0 bytes                                         │
│                                                         │
│ Response:                                               │
│   Status: 200 OK                                        │
│   Headers: 4 (178 bytes)                                │
│   Body: 456 bytes                                       │
│                                                         │
│ Total Transfer: 779 bytes                               │
└─────────────────────────────────────────────────────────┘
```

## Use Cases

### 1. Performance Analysis

Identify bottlenecks in your API calls:
- High **Waiting (TTFB)**: Server is slow to process
- High **Content Download**: Large response or slow network
- High **Total**: Overall performance issue

### 2. Debugging

Verify what's actually being sent:
- Check request headers are correct
- Verify authentication headers
- Confirm request body size
- See exact URL with query parameters

### 3. Bandwidth Monitoring

Track data transfer:
- Monitor request/response sizes
- Identify large payloads
- Optimize API calls

### 4. Header Inspection

Review all HTTP headers:
- Request headers (including auth)
- Response headers (caching, content-type, etc.)
- Custom headers

## Technical Details

### Data Structures

```rust
pub struct NetworkTiming {
    pub dns_lookup: Option<Duration>,
    pub tcp_connect: Option<Duration>,
    pub tls_handshake: Option<Duration>,
    pub request_sent: Duration,
    pub waiting: Duration,
    pub content_download: Duration,
    pub total: Duration,
}

pub struct RequestDetails {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
    pub body_size: usize,
}

pub struct NetworkTraffic {
    pub timing: NetworkTiming,
    pub request: RequestDetails,
    pub response_headers_size: usize,
    pub response_body_size: usize,
}
```

### Timing Measurement

- **Request Sent**: Estimated at ~1ms (actual time is very small)
- **Waiting (TTFB)**: Measured from request send to first response byte
- **Content Download**: Measured during response body download
- **Total**: Complete request/response cycle

### Limitations

1. **DNS/TCP/TLS timing**: Not currently captured (would require custom HTTP connectors)
2. **Request sent timing**: Approximated (actual time is negligible)
3. **Header sizes**: Calculated estimates (includes key + value + separators)

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| **t** | Toggle network traffic display on/off |
| **e** | Execute request (to generate traffic data) |

## Configuration

Network traffic tracking is:
- **Opt-in**: Disabled by default
- **Per-session**: Toggle state persists during app session
- **No performance impact**: Only captures data when requests are executed

## Examples

### Example 1: Fast API

```
Timing Breakdown:
  Request Sent:      1ms
  Waiting (TTFB):    15ms    ← Fast server response
  Content Download:  3ms
  Total:             19ms

Total Transfer: 234 bytes    ← Small payload
```

### Example 2: Slow API

```
Timing Breakdown:
  Request Sent:      1ms
  Waiting (TTFB):    2500ms  ← Slow server (2.5 seconds!)
  Content Download:  50ms
  Total:             2551ms

Total Transfer: 15234 bytes  ← Large response
```

### Example 3: Large Download

```
Timing Breakdown:
  Request Sent:      1ms
  Waiting (TTFB):    45ms
  Content Download:  850ms   ← Slow download (large file)
  Total:             896ms

Total Transfer: 5242880 bytes  ← 5MB response
```

## Comparison with Other Tools

### vs Wireshark
- **Wireshark**: Packet-level analysis, all network traffic
- **REST API TUI**: HTTP-level analysis, API requests only
- **Advantage**: Simpler, focused on API testing

### vs Browser DevTools
- **DevTools**: Full browser context, JavaScript, rendering
- **REST API TUI**: Pure HTTP, no browser overhead
- **Advantage**: Faster, terminal-based, scriptable

### vs Postman
- **Postman**: GUI-based, visual charts
- **REST API TUI**: Terminal-based, keyboard-driven
- **Advantage**: Faster workflow, no mouse needed

## Future Enhancements

Potential improvements:
1. **DNS/TCP/TLS timing**: Add custom HTTP connectors
2. **Request history**: Track multiple requests
3. **Comparison view**: Compare timing across requests
4. **Export**: Save traffic data to file
5. **Filtering**: Show/hide specific headers
6. **Hex dump**: View raw bytes
7. **Compression info**: Show if gzip/deflate was used

## Troubleshooting

### Network traffic not showing?

1. Make sure you've executed a request first (press 'e')
2. Press 't' to toggle the view
3. Check that the response panel has enough space

### Timing seems inaccurate?

- DNS/TCP/TLS times are not captured (would show as None)
- Request sent time is approximated
- Waiting and download times are accurate

### Headers truncated?

- Only first 3 headers are shown by default
- Full header count is displayed
- Scroll down to see more details

## Feedback

This feature is designed to help debug and optimize API calls. If you have suggestions for improvements or additional metrics to track, please let me know!

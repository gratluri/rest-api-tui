use rest_api_tui::load_test::{LoadTestMetrics, calculate_percentiles, LoadTestStatistics};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        BarChart, Block, Borders, Gauge, Paragraph
    },
    Terminal, Frame,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::time::{Duration, Instant};
use std::thread;

struct LoadTestSimulation {
    metrics: LoadTestMetrics,
    start_time: Instant,
    duration: Duration,
    target_requests: u64,
}

impl LoadTestSimulation {
    fn new(duration: Duration, target_requests: u64) -> Self {
        Self {
            metrics: LoadTestMetrics::new(),
            start_time: Instant::now(),
            duration,
            target_requests,
        }
    }
    
    fn progress(&self) -> f64 {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        let total = self.duration.as_secs_f64();
        (elapsed / total).min(1.0)
    }
    
    fn is_complete(&self) -> bool {
        self.start_time.elapsed() >= self.duration
    }
    
    fn simulate_request(&mut self) {
        // Simulate varying latencies and occasional failures
        let request_num = self.metrics.total_requests;
        
        if request_num % 25 == 0 {
            // Occasional timeout (4% of requests)
            self.metrics.record_failure("Timeout".to_string(), Duration::from_millis(30000));
        } else if request_num % 50 == 0 {
            // Rare server error (2% of requests)
            self.metrics.record_failure("500 Server Error".to_string(), Duration::from_millis(500));
        } else {
            // Normal request with realistic latency distribution
            let base = 50;
            let variance = (request_num % 20) * 10;
            let spike = if request_num % 100 == 0 { 200 } else { 0 }; // Occasional spike
            let latency = Duration::from_millis(base + variance + spike);
            self.metrics.record_success(latency);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create simulation
    let duration = Duration::from_secs(30);
    let target_requests = 300;
    let mut simulation = LoadTestSimulation::new(duration, target_requests);
    
    let mut last_update = Instant::now();
    let update_interval = Duration::from_millis(100); // Update every 100ms
    
    // Run simulation
    loop {
        // Draw UI
        terminal.draw(|f| {
            draw_ui(f, &simulation);
        })?;
        
        // Check for quit
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    break;
                }
            }
        }
        
        // Simulate requests
        if !simulation.is_complete() && last_update.elapsed() >= update_interval {
            // Simulate multiple requests per update to reach target
            let requests_per_update = (target_requests as f64 / (duration.as_secs_f64() / update_interval.as_secs_f64())) as u64;
            for _ in 0..requests_per_update {
                if simulation.metrics.total_requests < target_requests {
                    simulation.simulate_request();
                }
            }
            last_update = Instant::now();
        }
        
        // Exit when complete
        if simulation.is_complete() && simulation.metrics.total_requests >= target_requests {
            thread::sleep(Duration::from_secs(2)); // Show final results
            break;
        }
        
        thread::sleep(Duration::from_millis(50));
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Print final summary
    println!("\n=== Load Test Complete ===\n");
    let stats = LoadTestStatistics::from_metrics(&simulation.metrics, simulation.duration);
    let percentiles = calculate_percentiles(&simulation.metrics.latencies);
    
    println!("Total Requests: {}", stats.total_requests);
    println!("Success Rate: {:.1}%", stats.success_rate * 100.0);
    println!("Error Rate: {:.1}%", stats.error_rate * 100.0);
    println!("Average RPS: {:.2}", stats.avg_rps);
    println!("\nLatency:");
    println!("  P50: {:?}", percentiles.p50);
    println!("  P90: {:?}", percentiles.p90);
    println!("  P95: {:?}", percentiles.p95);
    println!("  P99: {:?}", percentiles.p99);

    Ok(())
}

fn draw_ui(f: &mut Frame, simulation: &LoadTestSimulation) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(3),  // Progress bar
            Constraint::Length(8),  // Stats
            Constraint::Min(10),    // Charts
            Constraint::Length(3),  // Footer
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("🚀 REST API TUI - Live Load Test")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Progress bar
    let progress = simulation.progress();
    let elapsed = simulation.start_time.elapsed().as_secs();
    let total = simulation.duration.as_secs();
    let gauge = Gauge::default()
        .block(Block::default().title("Progress").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Green))
        .percent((progress * 100.0) as u16)
        .label(format!("{}/{}s ({:.0}%)", elapsed, total, progress * 100.0));
    f.render_widget(gauge, chunks[1]);

    // Stats
    draw_stats(f, chunks[2], simulation);

    // Charts
    draw_charts(f, chunks[3], simulation);

    // Footer
    let footer = Paragraph::new("Press 'q' or 'Esc' to quit")
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[4]);
}

fn draw_stats(f: &mut Frame, area: Rect, simulation: &LoadTestSimulation) {
    let stats = LoadTestStatistics::from_metrics(&simulation.metrics, simulation.start_time.elapsed());
    let percentiles = calculate_percentiles(&simulation.metrics.latencies);
    
    let stats_text = vec![
        Line::from(vec![
            Span::styled("Total: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{}", stats.total_requests), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::raw("  "),
            Span::styled("Success: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{} ({:.1}%)", simulation.metrics.successful_requests, stats.success_rate * 100.0), Style::default().fg(Color::Green)),
            Span::raw("  "),
            Span::styled("Failed: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{} ({:.1}%)", simulation.metrics.failed_requests, stats.error_rate * 100.0), Style::default().fg(Color::Red)),
        ]),
        Line::from(vec![
            Span::styled("RPS: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{:.2}", stats.avg_rps), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("  "),
            Span::styled("Avg Latency: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{:?}", stats.avg_latency), Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("P50: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{:?}", percentiles.p50), Style::default().fg(Color::White)),
            Span::raw("  "),
            Span::styled("P90: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{:?}", percentiles.p90), Style::default().fg(Color::White)),
            Span::raw("  "),
            Span::styled("P95: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{:?}", percentiles.p95), Style::default().fg(Color::White)),
            Span::raw("  "),
            Span::styled("P99: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{:?}", percentiles.p99), Style::default().fg(Color::White)),
        ]),
    ];
    
    let paragraph = Paragraph::new(stats_text)
        .block(Block::default().title("📊 Statistics").borders(Borders::ALL));
    f.render_widget(paragraph, area);
}

fn draw_charts(f: &mut Frame, area: Rect, simulation: &LoadTestSimulation) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Success/Failure bar chart
    let data = vec![
        ("Success", simulation.metrics.successful_requests),
        ("Failed", simulation.metrics.failed_requests),
    ];
    
    let bar_chart = BarChart::default()
        .block(Block::default().title("Request Status").borders(Borders::ALL))
        .data(&data)
        .bar_width(15)
        .bar_gap(2)
        .bar_style(Style::default().fg(Color::Green))
        .value_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD));
    f.render_widget(bar_chart, chunks[0]);

    // Error breakdown
    let error_text: Vec<Line> = if simulation.metrics.error_counts.is_empty() {
        vec![Line::from("No errors yet ✓")]
    } else {
        simulation.metrics.error_counts.iter()
            .map(|(error_type, count)| {
                let percentage = (*count as f64 / simulation.metrics.total_requests as f64) * 100.0;
                Line::from(vec![
                    Span::styled(format!("{}: ", error_type), Style::default().fg(Color::Red)),
                    Span::styled(format!("{} ({:.1}%)", count, percentage), Style::default().fg(Color::White)),
                ])
            })
            .collect()
    };
    
    let error_paragraph = Paragraph::new(error_text)
        .block(Block::default().title("⚠️  Error Breakdown").borders(Borders::ALL));
    f.render_widget(error_paragraph, chunks[1]);
}

use super::app::{AppState, Screen};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap, BarChart, Gauge},
    Frame, Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::time::Duration;

pub fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = AppState::new()?;
    
    // Run app loop
    let res = run_app_loop(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app_loop<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut AppState,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| draw_ui(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // Handle confirmation dialog first
                if matches!(app.current_screen, Screen::ConfirmDelete(_)) {
                    match key.code {
                        KeyCode::Char('y') | KeyCode::Char('Y') => {
                            app.confirm_delete_action();
                        }
                        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                            app.navigate_back();
                        }
                        _ => {}
                    }
                    continue;
                }
                
                // Handle edit screens - they need character input
                let in_edit_screen = matches!(app.current_screen, Screen::CollectionEdit(_) | Screen::EndpointEdit(_, _));
                
                match key.code {
                    KeyCode::Char('q') => {
                        if !in_edit_screen && matches!(app.current_screen, Screen::CollectionList) {
                            return Ok(());
                        } else if in_edit_screen {
                            // In edit screen, 'q' is just a character
                            match &app.current_screen {
                                Screen::CollectionEdit(_) => {
                                    if let Some(form) = &mut app.collection_form {
                                        form.name.push('q');
                                    }
                                }
                                Screen::EndpointEdit(_, _) => {
                                    if let Some(form) = &mut app.endpoint_form {
                                        match form.current_field {
                                            0 => form.name.push('q'),
                                            2 => form.url.push('q'),
                                            3 => form.description.push('q'),
                                            5 => form.body_template.push('q'),
                                            _ => {}
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    KeyCode::Esc => {
                        // Check if in header edit mode first
                        if matches!(app.current_screen, Screen::EndpointEdit(_, _)) {
                            if let Some(form) = &app.endpoint_form {
                                if form.header_edit_mode {
                                    app.toggle_header_edit_mode();
                                    continue;
                                }
                            }
                        }
                        
                        if matches!(app.current_screen, Screen::LoadTestRunning(_, _)) {
                            app.stop_load_test();
                            app.navigate_back();
                        } else {
                            app.navigate_back();
                        }
                    }
                    KeyCode::Char('?') => {
                        if !in_edit_screen {
                            app.current_screen = Screen::Help;
                        } else {
                            // In edit screen, '?' is just a character
                            match &app.current_screen {
                                Screen::CollectionEdit(_) => {
                                    if let Some(form) = &mut app.collection_form {
                                        form.name.push('?');
                                    }
                                }
                                Screen::EndpointEdit(_, _) => {
                                    if let Some(form) = &mut app.endpoint_form {
                                        match form.current_field {
                                            0 => form.name.push('?'),
                                            2 => form.url.push('?'),
                                            3 => form.description.push('?'),
                                            5 => form.body_template.push('?'),
                                            _ => {}
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if !in_edit_screen {
                            app.navigate_up();
                        } else if key.code == KeyCode::Char('k') {
                            // In edit screen, 'k' is just a character
                            match &app.current_screen {
                                Screen::CollectionEdit(_) => {
                                    if let Some(form) = &mut app.collection_form {
                                        form.name.push('k');
                                    }
                                }
                                Screen::EndpointEdit(_, _) => {
                                    if let Some(form) = &mut app.endpoint_form {
                                        match form.current_field {
                                            0 => form.name.push('k'),
                                            2 => form.url.push('k'),
                                            3 => form.description.push('k'),
                                            5 => form.body_template.push('k'),
                                            _ => {}
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if !in_edit_screen {
                            let max = match &app.current_screen {
                                Screen::CollectionList => app.collections.len(),
                                Screen::EndpointList(idx) => {
                                    app.collections.get(*idx).map(|c| c.endpoints.len()).unwrap_or(0)
                                }
                                _ => 0,
                            };
                            app.navigate_down(max);
                        } else if key.code == KeyCode::Char('j') {
                            // In edit screen, 'j' is just a character
                            match &app.current_screen {
                                Screen::CollectionEdit(_) => {
                                    if let Some(form) = &mut app.collection_form {
                                        form.name.push('j');
                                    }
                                }
                                Screen::EndpointEdit(_, _) => {
                                    if let Some(form) = &mut app.endpoint_form {
                                        match form.current_field {
                                            0 => form.name.push('j'),
                                            2 => form.url.push('j'),
                                            3 => form.description.push('j'),
                                            5 => form.body_template.push('j'),
                                            _ => {}
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    KeyCode::Enter => {
                        match &app.current_screen {
                            Screen::CollectionEdit(_) => {
                                app.save_collection();
                            }
                            Screen::EndpointEdit(_, _) => {
                                // Check if in header edit mode
                                if let Some(form) = &app.endpoint_form {
                                    if form.header_edit_mode {
                                        app.add_header();
                                    } else {
                                        app.save_endpoint();
                                    }
                                } else {
                                    app.save_endpoint();
                                }
                            }
                            _ => {
                                app.select();
                            }
                        }
                    }
                    KeyCode::Char(c) => {
                        // Check if in header edit mode
                        if matches!(app.current_screen, Screen::EndpointEdit(_, _)) {
                            if let Some(form) = &app.endpoint_form {
                                if form.header_edit_mode {
                                    // In header edit mode, handle text input
                                    if let Some(form) = &mut app.endpoint_form {
                                        match form.header_edit_field {
                                            0 => form.header_key.push(c),
                                            1 => form.header_value.push(c),
                                            _ => {}
                                        }
                                    }
                                    continue;
                                }
                            }
                        }
                        
                        // Special handling for 'h' in endpoint edit - toggle header mode ONLY on headers field
                        if c == 'h' && matches!(app.current_screen, Screen::EndpointEdit(_, _)) {
                            if let Some(form) = &app.endpoint_form {
                                // Only toggle header mode if on the headers field (field 4) and not already in header mode
                                if !form.header_edit_mode && form.current_field == 4 {
                                    app.toggle_header_edit_mode();
                                    continue;
                                }
                            }
                        }
                        
                        // Special handling for 'm' in endpoint edit - cycle method ONLY on method field
                        if c == 'm' && matches!(app.current_screen, Screen::EndpointEdit(_, _)) {
                            if let Some(form) = &app.endpoint_form {
                                // Only cycle method if we're on the method field (field 1)
                                if form.current_field == 1 {
                                    app.cycle_http_method();
                                } else {
                                    // Otherwise, treat 'm' as regular text input
                                    if let Some(form) = &mut app.endpoint_form {
                                        match form.current_field {
                                            0 => form.name.push(c),
                                            2 => form.url.push(c),
                                            3 => form.description.push(c),
                                            5 => form.body_template.push(c),
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                        // In edit screens, all other characters are input
                        else if in_edit_screen {
                            match &app.current_screen {
                                Screen::CollectionEdit(_) => {
                                    if let Some(form) = &mut app.collection_form {
                                        form.name.push(c);
                                    }
                                }
                                Screen::EndpointEdit(_, _) => {
                                    if let Some(form) = &mut app.endpoint_form {
                                        match form.current_field {
                                            0 => form.name.push(c),
                                            2 => form.url.push(c),
                                            3 => form.description.push(c),
                                            5 => form.body_template.push(c),
                                            _ => {}
                                        }
                                    }
                                }
                                _ => {}
                            }
                        } else {
                            // Not in edit screen, handle as commands
                            match c {
                                'n' => {
                                    // New collection or endpoint
                                    match &app.current_screen {
                                        Screen::CollectionList => {
                                            app.start_new_collection();
                                        }
                                        Screen::EndpointList(coll_idx) => {
                                            app.start_new_endpoint(*coll_idx);
                                        }
                                        _ => {}
                                    }
                                }
                                'e' => {
                                    // Edit or Execute
                                    match &app.current_screen {
                                        Screen::CollectionList => {
                                            if app.selected_index < app.collections.len() {
                                                app.start_edit_collection(app.selected_index);
                                            }
                                        }
                                        Screen::EndpointList(coll_idx) => {
                                            if let Some(collection) = app.collections.get(*coll_idx) {
                                                if app.selected_index < collection.endpoints.len() {
                                                    app.start_edit_endpoint(*coll_idx, app.selected_index);
                                                }
                                            }
                                        }
                                        Screen::EndpointDetail(coll_idx, ep_idx) => {
                                            let runtime = tokio::runtime::Runtime::new().unwrap();
                                            runtime.block_on(app.execute_request(*coll_idx, *ep_idx));
                                        }
                                        _ => {}
                                    }
                                }
                                'd' => {
                                    // Delete collection or endpoint
                                    match &app.current_screen {
                                        Screen::CollectionList => {
                                            if app.selected_index < app.collections.len() {
                                                app.confirm_delete_collection(app.selected_index);
                                            }
                                        }
                                        Screen::EndpointList(coll_idx) => {
                                            if let Some(collection) = app.collections.get(*coll_idx) {
                                                if app.selected_index < collection.endpoints.len() {
                                                    app.confirm_delete_endpoint(*coll_idx, app.selected_index);
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                'l' => {
                                    // Start load test
                                    if let Screen::EndpointDetail(coll_idx, ep_idx) = app.current_screen {
                                        app.start_load_test(coll_idx, ep_idx);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        // Delete character in forms
                        match &app.current_screen {
                            Screen::CollectionEdit(_) => {
                                if let Some(form) = &mut app.collection_form {
                                    form.name.pop();
                                }
                            }
                            Screen::EndpointEdit(_, _) => {
                                if let Some(form) = &mut app.endpoint_form {
                                    if form.header_edit_mode {
                                        // In header edit mode
                                        match form.header_edit_field {
                                            0 => { form.header_key.pop(); }
                                            1 => { form.header_value.pop(); }
                                            _ => {}
                                        }
                                    } else {
                                        // Normal field editing
                                        match form.current_field {
                                            0 => { form.name.pop(); }
                                            2 => { form.url.pop(); }
                                            3 => { form.description.pop(); }
                                            5 => { form.body_template.pop(); }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    KeyCode::Tab => {
                        // Move to next field in endpoint edit
                        if let Screen::EndpointEdit(_, _) = app.current_screen {
                            if let Some(form) = &mut app.endpoint_form {
                                if form.header_edit_mode {
                                    // In header edit mode, cycle between key and value
                                    app.cycle_header_field();
                                } else {
                                    // Normal field navigation
                                    form.current_field = (form.current_field + 1) % 6;
                                }
                            }
                        }
                    }
                    KeyCode::BackTab => {
                        // Move to previous field in endpoint edit (Shift+Tab)
                        if let Screen::EndpointEdit(_, _) = app.current_screen {
                            if let Some(form) = &mut app.endpoint_form {
                                if form.header_edit_mode {
                                    // In header edit mode, cycle between key and value (backwards)
                                    form.header_edit_field = if form.header_edit_field == 0 { 1 } else { 0 };
                                } else {
                                    // Normal field navigation
                                    form.current_field = if form.current_field == 0 {
                                        5
                                    } else {
                                        form.current_field - 1
                                    };
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn draw_ui(f: &mut Frame, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Title
    draw_title(f, chunks[0]);

    // Main content
    match &app.current_screen {
        Screen::CollectionList => draw_collection_list(f, chunks[1], app),
        Screen::CollectionEdit(_) => draw_collection_edit(f, chunks[1], app),
        Screen::EndpointList(idx) => draw_endpoint_list(f, chunks[1], app, *idx),
        Screen::EndpointEdit(coll_idx, _) => draw_endpoint_edit(f, chunks[1], app, *coll_idx),
        Screen::EndpointDetail(coll_idx, ep_idx) => draw_endpoint_detail(f, chunks[1], app, *coll_idx, *ep_idx),
        Screen::ResponseView(coll_idx, ep_idx) => draw_response_view(f, chunks[1], app, *coll_idx, *ep_idx),
        Screen::LoadTestConfig(_, _) => draw_help(f, chunks[1]), // TODO: implement config screen
        Screen::LoadTestRunning(coll_idx, ep_idx) => draw_load_test(f, chunks[1], app, *coll_idx, *ep_idx),
        Screen::ConfirmDelete(_) => draw_confirm_delete(f, chunks[1], app),
        Screen::Help => draw_help(f, chunks[1]),
    }

    // Footer
    draw_footer(f, chunks[2], app);
}

fn draw_title(f: &mut Frame, area: Rect) {
    let title = Paragraph::new("🚀 REST API TUI - Terminal API Testing Tool")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, area);
}

fn draw_footer(f: &mut Frame, area: Rect, app: &AppState) {
    let text = if let Some(err) = &app.error_message {
        Line::from(vec![
            Span::styled("Error: ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::styled(err, Style::default().fg(Color::Red)),
        ])
    } else if let Some(status) = &app.status_message {
        Line::from(vec![
            Span::styled("✓ ", Style::default().fg(Color::Green)),
            Span::styled(status, Style::default().fg(Color::Green)),
        ])
    } else {
        Line::from("Press '?' for help | 'q' to quit")
    };
    
    let footer = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, area);
}

fn draw_collection_list(f: &mut Frame, area: Rect, app: &AppState) {
    let items: Vec<ListItem> = app
        .collections
        .iter()
        .enumerate()
        .map(|(i, collection)| {
            let style = if i == app.selected_index {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            
            let content = format!("📁 {} ({} endpoints)", collection.name, collection.endpoints.len());
            ListItem::new(content).style(style)
        })
        .collect();

    let title = "API Collections [n: new | e: edit | d: delete | Enter: open]";
    let list = List::new(items)
        .block(Block::default().title(title).borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("▶ ");

    f.render_widget(list, area);
}

fn draw_endpoint_list(f: &mut Frame, area: Rect, app: &AppState, coll_idx: usize) {
    if let Some(collection) = app.collections.get(coll_idx) {
        let items: Vec<ListItem> = collection
            .endpoints
            .iter()
            .enumerate()
            .map(|(i, endpoint)| {
                let style = if i == app.selected_index {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                
                let method_color = match endpoint.method {
                    crate::models::HttpMethod::GET => Color::Green,
                    crate::models::HttpMethod::POST => Color::Blue,
                    crate::models::HttpMethod::PUT => Color::Yellow,
                    crate::models::HttpMethod::DELETE => Color::Red,
                    _ => Color::White,
                };
                
                let content = Line::from(vec![
                    Span::styled(format!("{:?} ", endpoint.method), Style::default().fg(method_color).add_modifier(Modifier::BOLD)),
                    Span::raw(&endpoint.name),
                ]);
                
                ListItem::new(content).style(style)
            })
            .collect();

        let title = format!("Endpoints - {} [n: new | e: edit | d: delete | Enter: view]", collection.name);
        let list = List::new(items)
            .block(Block::default().title(title).borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("▶ ");

        f.render_widget(list, area);
    }
}

fn draw_endpoint_detail(f: &mut Frame, area: Rect, app: &AppState, coll_idx: usize, ep_idx: usize) {
    if let Some(collection) = app.collections.get(coll_idx) {
        if let Some(endpoint) = collection.endpoints.get(ep_idx) {
            let text = vec![
                Line::from(vec![
                    Span::styled("Method: ", Style::default().fg(Color::Gray)),
                    Span::styled(format!("{:?}", endpoint.method), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(vec![
                    Span::styled("URL: ", Style::default().fg(Color::Gray)),
                    Span::raw(&endpoint.url),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Headers:", Style::default().fg(Color::Yellow)),
                ]),
            ];
            
            let mut all_lines = text;
            for (key, value) in &endpoint.headers {
                all_lines.push(Line::from(format!("  {}: {}", key, value)));
            }
            
            if let Some(auth) = &endpoint.auth {
                all_lines.push(Line::from(""));
                all_lines.push(Line::from(vec![
                    Span::styled("Authentication: ", Style::default().fg(Color::Yellow)),
                    Span::raw(format!("{:?}", auth)),
                ]));
            }
            
            all_lines.push(Line::from(""));
            all_lines.push(Line::from(vec![
                Span::styled("Actions:", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            ]));
            all_lines.push(Line::from("  [e] Execute Request"));
            all_lines.push(Line::from("  [l] Start Load Test"));

            let paragraph = Paragraph::new(all_lines)
                .block(Block::default().title(format!("Endpoint: {}", endpoint.name)).borders(Borders::ALL))
                .wrap(Wrap { trim: true });

            f.render_widget(paragraph, area);
        }
    }
}

fn draw_response_view(f: &mut Frame, area: Rect, app: &AppState, _coll_idx: usize, _ep_idx: usize) {
    if let Some(response) = &app.last_response {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(5), Constraint::Min(0)])
            .split(area);

        // Response metadata
        let metadata = vec![
            Line::from(vec![
                Span::styled("Status: ", Style::default().fg(Color::Gray)),
                Span::styled(format!("{}", response.status), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("Duration: ", Style::default().fg(Color::Gray)),
                Span::raw(format!("{:?}", response.duration)),
            ]),
            Line::from(vec![
                Span::styled("Size: ", Style::default().fg(Color::Gray)),
                Span::raw(format!("{} bytes", response.body.len())),
            ]),
        ];

        let meta_paragraph = Paragraph::new(metadata)
            .block(Block::default().title("Response").borders(Borders::ALL));
        f.render_widget(meta_paragraph, chunks[0]);

        // Response body
        if let Some(formatted) = &app.last_response_formatted {
            let body_paragraph = Paragraph::new(formatted.as_str())
                .block(Block::default().title("Body").borders(Borders::ALL))
                .wrap(Wrap { trim: false });
            f.render_widget(body_paragraph, chunks[1]);
        }
    }
}

fn draw_load_test(f: &mut Frame, area: Rect, app: &AppState, _coll_idx: usize, _ep_idx: usize) {
    if let Some(metrics) = app.get_load_test_metrics() {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(8),
                Constraint::Min(0),
            ])
            .split(area);

        // Progress
        let progress = if let Some(engine) = &app.load_test_engine {
            let elapsed = engine.elapsed().as_secs_f64();
            let total = app.load_test_config.duration.as_secs_f64();
            (elapsed / total * 100.0).min(100.0) as u16
        } else {
            0
        };

        let gauge = Gauge::default()
            .block(Block::default().title("Load Test Progress").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Green))
            .percent(progress);
        f.render_widget(gauge, chunks[0]);

        // Stats
        let stats_text = vec![
            Line::from(vec![
                Span::styled("Total Requests: ", Style::default().fg(Color::Gray)),
                Span::styled(format!("{}", metrics.total_requests), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("Successful: ", Style::default().fg(Color::Gray)),
                Span::styled(format!("{}", metrics.successful_requests), Style::default().fg(Color::Green)),
                Span::raw("  "),
                Span::styled("Failed: ", Style::default().fg(Color::Gray)),
                Span::styled(format!("{}", metrics.failed_requests), Style::default().fg(Color::Red)),
            ]),
            Line::from(vec![
                Span::styled("Current RPS: ", Style::default().fg(Color::Gray)),
                Span::styled(format!("{:.2}", metrics.current_rps), Style::default().fg(Color::Yellow)),
            ]),
        ];

        let stats_paragraph = Paragraph::new(stats_text)
            .block(Block::default().title("Statistics").borders(Borders::ALL));
        f.render_widget(stats_paragraph, chunks[1]);

        // Chart
        let data = vec![
            ("Success", metrics.successful_requests),
            ("Failed", metrics.failed_requests),
        ];

        let chart = BarChart::default()
            .block(Block::default().title("Results").borders(Borders::ALL))
            .data(&data)
            .bar_width(15)
            .bar_style(Style::default().fg(Color::Green))
            .value_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD));
        f.render_widget(chart, chunks[2]);
    }
}

fn draw_collection_edit(f: &mut Frame, area: Rect, app: &AppState) {
    if let Some(form) = &app.collection_form {
        let title = if form.editing_index.is_some() {
            "Edit Collection [Enter: save | Esc: cancel]"
        } else {
            "New Collection [Enter: save | Esc: cancel]"
        };
        
        let text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("Collection Name: ", Style::default().fg(Color::Cyan)),
                Span::styled(&form.name, Style::default().fg(Color::Yellow)),
                Span::styled("_", Style::default().fg(Color::Yellow).add_modifier(Modifier::SLOW_BLINK)),
            ]),
            Line::from(""),
            Line::from("Type to enter name, press Enter to save"),
        ];
        
        let paragraph = Paragraph::new(text)
            .block(Block::default().title(title).borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        
        f.render_widget(paragraph, area);
    }
}

fn draw_endpoint_edit(f: &mut Frame, area: Rect, app: &AppState, _coll_idx: usize) {
    if let Some(form) = &app.endpoint_form {
        let title = if form.editing_index.is_some() {
            "Edit Endpoint [Tab: next field | Enter: save | Esc: cancel]"
        } else {
            "New Endpoint [Tab: next field | Enter: save | Esc: cancel]"
        };
        
        let field_style = |field_num: usize| {
            if form.current_field == field_num {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            }
        };
        
        let cursor = |field_num: usize| {
            if form.current_field == field_num {
                "_"
            } else {
                ""
            }
        };
        
        let mut text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("Name: ", Style::default().fg(Color::Cyan)),
                Span::styled(&form.name, field_style(0)),
                Span::styled(cursor(0), field_style(0).add_modifier(Modifier::SLOW_BLINK)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Method: ", Style::default().fg(Color::Cyan)),
                Span::styled(format!("{:?}", form.method), field_style(1)),
                Span::styled(" (press 'm' to cycle)", Style::default().fg(Color::DarkGray)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("URL: ", Style::default().fg(Color::Cyan)),
                Span::styled(&form.url, field_style(2)),
                Span::styled(cursor(2), field_style(2).add_modifier(Modifier::SLOW_BLINK)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Description: ", Style::default().fg(Color::Cyan)),
                Span::styled(&form.description, field_style(3)),
                Span::styled(cursor(3), field_style(3).add_modifier(Modifier::SLOW_BLINK)),
            ]),
            Line::from(""),
        ];
        
        // Show headers section
        if form.header_edit_mode {
            // Header edit mode - show input fields
            text.push(Line::from(vec![
                Span::styled("Headers (Edit Mode): ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            ]));
            text.push(Line::from(""));
            text.push(Line::from(vec![
                Span::styled("  Key: ", Style::default().fg(Color::Cyan)),
                Span::styled(&form.header_key, if form.header_edit_field == 0 { field_style(4) } else { Style::default() }),
                Span::styled(if form.header_edit_field == 0 { "_" } else { "" }, field_style(4).add_modifier(Modifier::SLOW_BLINK)),
            ]));
            text.push(Line::from(vec![
                Span::styled("  Value: ", Style::default().fg(Color::Cyan)),
                Span::styled(&form.header_value, if form.header_edit_field == 1 { field_style(4) } else { Style::default() }),
                Span::styled(if form.header_edit_field == 1 { "_" } else { "" }, field_style(4).add_modifier(Modifier::SLOW_BLINK)),
            ]));
            text.push(Line::from(""));
            text.push(Line::from(vec![
                Span::styled("  Tab: switch field | Enter: add | Esc: cancel", Style::default().fg(Color::DarkGray)),
            ]));
        } else {
            // Normal mode - show existing headers
            text.push(Line::from(vec![
                Span::styled("Headers: ", Style::default().fg(Color::Cyan)),
                Span::styled(format!("({}) ", form.headers.len()), Style::default().fg(Color::DarkGray)),
                if form.current_field == 4 {
                    Span::styled("[press 'h' to add]", field_style(4))
                } else {
                    Span::styled("[Tab to this field, then 'h' to add]", Style::default().fg(Color::DarkGray))
                },
            ]));
            if !form.headers.is_empty() {
                for (key, value) in &form.headers {
                    text.push(Line::from(vec![
                        Span::styled("  ", Style::default()),
                        Span::styled(key, Style::default().fg(Color::Yellow)),
                        Span::styled(": ", Style::default()),
                        Span::styled(value, Style::default().fg(Color::White)),
                    ]));
                }
            }
        }
        
        text.push(Line::from(""));
        text.push(Line::from(vec![
            Span::styled("Body Template: ", Style::default().fg(Color::Cyan)),
            Span::styled(&form.body_template, field_style(5)),
            Span::styled(cursor(5), field_style(5).add_modifier(Modifier::SLOW_BLINK)),
        ]));
        text.push(Line::from(""));
        
        if !form.header_edit_mode {
            text.push(Line::from("Tab: next field | h: add header | Enter: save"));
        }
        
        let paragraph = Paragraph::new(text)
            .block(Block::default().title(title).borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        
        f.render_widget(paragraph, area);
    }
}

fn draw_confirm_delete(f: &mut Frame, area: Rect, app: &AppState) {
    if let Some(message) = app.get_delete_confirmation_message() {
        // Create a centered dialog
        let dialog_width = 60;
        let dialog_height = 10;
        
        let dialog_area = Rect {
            x: (area.width.saturating_sub(dialog_width)) / 2,
            y: (area.height.saturating_sub(dialog_height)) / 2,
            width: dialog_width.min(area.width),
            height: dialog_height.min(area.height),
        };
        
        // Clear the background
        let background = Block::default()
            .style(Style::default().bg(Color::Black));
        f.render_widget(background, area);
        
        let text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("⚠️  CONFIRM DELETE", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(message.clone()),
            Line::from(""),
            Line::from(""),
            Line::from(vec![
                Span::styled("Press ", Style::default().fg(Color::Gray)),
                Span::styled("Y", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::styled(" to confirm or ", Style::default().fg(Color::Gray)),
                Span::styled("N/Esc", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(" to cancel", Style::default().fg(Color::Gray)),
            ]),
        ];
        
        let dialog = Paragraph::new(text)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red))
                .style(Style::default().bg(Color::Black)))
            .wrap(Wrap { trim: true })
            .alignment(ratatui::layout::Alignment::Center);
        
        f.render_widget(dialog, dialog_area);
    }
}

fn draw_help(f: &mut Frame, area: Rect) {
    let help_text = vec![
        Line::from(vec![Span::styled("Keyboard Shortcuts", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))]),
        Line::from(""),
        Line::from("Navigation:"),
        Line::from("  ↑/k        - Move up"),
        Line::from("  ↓/j        - Move down"),
        Line::from("  Enter      - Select item"),
        Line::from("  Esc        - Go back"),
        Line::from("  q          - Quit (from main screen)"),
        Line::from(""),
        Line::from("Collection Management:"),
        Line::from("  n          - New collection/endpoint"),
        Line::from("  e          - Edit collection/endpoint"),
        Line::from("  d          - Delete collection/endpoint"),
        Line::from(""),
        Line::from("Endpoint Actions:"),
        Line::from("  e          - Execute request (from detail)"),
        Line::from("  l          - Start load test"),
        Line::from(""),
        Line::from("Form Editing:"),
        Line::from("  Tab        - Next field"),
        Line::from("  m          - Cycle HTTP method"),
        Line::from("  Backspace  - Delete character"),
        Line::from("  Enter      - Save"),
        Line::from(""),
        Line::from("Other:"),
        Line::from("  ?          - Show this help"),
        Line::from(""),
        Line::from("Press any key to close help"),
    ];

    let paragraph = Paragraph::new(help_text)
        .block(Block::default().title("Help").borders(Borders::ALL))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

use super::app::{AppState, Screen};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap, BarChart, Gauge, Sparkline, BorderType},
    Frame, Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::time::Duration;

/// Get spinner character based on elapsed time
fn get_spinner(elapsed_millis: u128) -> &'static str {
    let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let index = (elapsed_millis / 100) % frames.len() as u128;
    frames[index as usize]
}

/// Get pulsing color for active elements
fn get_pulse_color(elapsed_millis: u128) -> Color {
    let cycle = (elapsed_millis / 500) % 2;
    if cycle == 0 {
        Color::Cyan
    } else {
        Color::LightCyan
    }
}

/// Colorize JSON text with syntax highlighting and rainbow bracket matching
fn colorize_json(json_text: &str) -> Vec<Line<'_>> {
    let mut lines = Vec::new();
    let mut current_line = Vec::new();
    let mut brace_stack: Vec<Color> = Vec::new();
    let brace_colors = [
        Color::Cyan,
        Color::Yellow,
        Color::Magenta,
        Color::Green,
        Color::Blue,
        Color::LightCyan,
        Color::LightYellow,
        Color::LightMagenta,
    ];
    
    let mut chars = json_text.chars().peekable();
    let mut in_string = false;
    let mut in_key = false;
    let mut escape_next = false;
    let mut buffer = String::new();
    
    while let Some(ch) = chars.next() {
        if escape_next {
            buffer.push(ch);
            escape_next = false;
            continue;
        }
        
        if ch == '\\' && in_string {
            buffer.push(ch);
            escape_next = true;
            continue;
        }
        
        // Handle strings
        if ch == '"' {
            if in_string {
                // End of string
                buffer.push(ch);
                let color = if in_key {
                    Color::LightBlue
                } else {
                    Color::Green
                };
                current_line.push(Span::styled(buffer.clone(), Style::default().fg(color)));
                buffer.clear();
                in_string = false;
                in_key = false;
            } else {
                // Start of string
                if !buffer.is_empty() {
                    current_line.push(Span::raw(buffer.clone()));
                    buffer.clear();
                }
                in_string = true;
                // Check if this is a key (followed by :)
                let mut temp_chars = chars.clone();
                while let Some(next_ch) = temp_chars.next() {
                    if next_ch == ':' {
                        in_key = true;
                        break;
                    } else if !next_ch.is_whitespace() {
                        break;
                    }
                }
                buffer.push(ch);
            }
            continue;
        }
        
        if in_string {
            buffer.push(ch);
            continue;
        }
        
        // Handle braces and brackets
        if ch == '{' || ch == '[' {
            if !buffer.is_empty() {
                current_line.push(Span::raw(buffer.clone()));
                buffer.clear();
            }
            let color = brace_colors[brace_stack.len() % brace_colors.len()];
            brace_stack.push(color);
            current_line.push(Span::styled(ch.to_string(), Style::default().fg(color).add_modifier(Modifier::BOLD)));
        } else if ch == '}' || ch == ']' {
            if !buffer.is_empty() {
                current_line.push(Span::raw(buffer.clone()));
                buffer.clear();
            }
            let color = brace_stack.pop().unwrap_or(Color::White);
            current_line.push(Span::styled(ch.to_string(), Style::default().fg(color).add_modifier(Modifier::BOLD)));
        } else if ch == ':' {
            if !buffer.is_empty() {
                current_line.push(Span::raw(buffer.clone()));
                buffer.clear();
            }
            current_line.push(Span::styled(":", Style::default().fg(Color::DarkGray)));
        } else if ch == ',' {
            if !buffer.is_empty() {
                current_line.push(Span::raw(buffer.clone()));
                buffer.clear();
            }
            current_line.push(Span::styled(",", Style::default().fg(Color::DarkGray)));
        } else if ch == '\n' {
            if !buffer.is_empty() {
                // Check if buffer contains numbers, booleans, or null
                let trimmed = buffer.trim();
                if trimmed == "true" || trimmed == "false" {
                    current_line.push(Span::styled(buffer.clone(), Style::default().fg(Color::Yellow)));
                } else if trimmed == "null" {
                    current_line.push(Span::styled(buffer.clone(), Style::default().fg(Color::Red)));
                } else if trimmed.parse::<f64>().is_ok() {
                    current_line.push(Span::styled(buffer.clone(), Style::default().fg(Color::Magenta)));
                } else {
                    current_line.push(Span::raw(buffer.clone()));
                }
                buffer.clear();
            }
            lines.push(Line::from(current_line.clone()));
            current_line.clear();
        } else {
            buffer.push(ch);
        }
    }
    
    // Flush remaining buffer
    if !buffer.is_empty() {
        let trimmed = buffer.trim();
        if trimmed == "true" || trimmed == "false" {
            current_line.push(Span::styled(buffer.clone(), Style::default().fg(Color::Yellow)));
        } else if trimmed == "null" {
            current_line.push(Span::styled(buffer.clone(), Style::default().fg(Color::Red)));
        } else if trimmed.parse::<f64>().is_ok() {
            current_line.push(Span::styled(buffer.clone(), Style::default().fg(Color::Magenta)));
        } else {
            current_line.push(Span::raw(buffer));
        }
    }
    
    if !current_line.is_empty() {
        lines.push(Line::from(current_line));
    }
    
    lines
}

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
                
                // Handle Ctrl+ijkl for panel navigation (not in edit screens)
                if !in_edit_screen && key.modifiers.contains(KeyModifiers::CONTROL) {
                    match key.code {
                        KeyCode::Char('k') => {
                            // Ctrl+k: Navigate up in current panel
                            let max = match app.panel_focus {
                                crate::tui_app::PanelFocus::Collections => app.collections.len(),
                                crate::tui_app::PanelFocus::Endpoints => {
                                    app.collections.get(app.selected_collection_index)
                                        .map(|c| c.endpoints.len())
                                        .unwrap_or(0)
                                }
                            };
                            if max > 0 {
                                app.navigate_up();
                            }
                            continue;
                        }
                        KeyCode::Char('j') => {
                            // Ctrl+j: Navigate down in current panel
                            let max = match app.panel_focus {
                                crate::tui_app::PanelFocus::Collections => app.collections.len(),
                                crate::tui_app::PanelFocus::Endpoints => {
                                    app.collections.get(app.selected_collection_index)
                                        .map(|c| c.endpoints.len())
                                        .unwrap_or(0)
                                }
                            };
                            if max > 0 {
                                app.navigate_down(max);
                            }
                            continue;
                        }
                        KeyCode::Char('h') => {
                            // Ctrl+h: Switch to collections panel (left)
                            app.panel_focus = crate::tui_app::PanelFocus::Collections;
                            continue;
                        }
                        KeyCode::Char('l') => {
                            // Ctrl+l: Switch to endpoints panel (right)
                            app.panel_focus = crate::tui_app::PanelFocus::Endpoints;
                            continue;
                        }
                        KeyCode::Char('i') => {
                            // Ctrl+i: Toggle between panels (like Tab)
                            app.toggle_panel_focus();
                            continue;
                        }
                        _ => {}
                    }
                }
                
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
                            let max = match app.panel_focus {
                                crate::tui_app::PanelFocus::Collections => app.collections.len(),
                                crate::tui_app::PanelFocus::Endpoints => {
                                    app.collections.get(app.selected_collection_index)
                                        .map(|c| c.endpoints.len())
                                        .unwrap_or(0)
                                }
                            };
                            if max > 0 {
                                app.navigate_up();
                            }
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
                            let max = match app.panel_focus {
                                crate::tui_app::PanelFocus::Collections => app.collections.len(),
                                crate::tui_app::PanelFocus::Endpoints => {
                                    app.collections.get(app.selected_collection_index)
                                        .map(|c| c.endpoints.len())
                                        .unwrap_or(0)
                                }
                            };
                            if max > 0 {
                                app.navigate_down(max);
                            }
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
                            Screen::LoadTestConfig(_, _) => {
                                // Start load test with configured parameters
                                app.execute_load_test();
                            }
                            Screen::CollectionList => {
                                // In new layout, Enter selects endpoint to view
                                if app.panel_focus == crate::tui_app::PanelFocus::Endpoints {
                                    if let Some(collection) = app.collections.get(app.selected_collection_index) {
                                        if app.selected_endpoint_index < collection.endpoints.len() {
                                            app.current_screen = Screen::EndpointDetail(
                                                app.selected_collection_index,
                                                app.selected_endpoint_index
                                            );
                                        }
                                    }
                                } else {
                                    // Switch to endpoints panel
                                    app.panel_focus = crate::tui_app::PanelFocus::Endpoints;
                                }
                            }
                            _ => {
                                app.select();
                            }
                        }
                    }
                    KeyCode::Char(c) => {
                        // Check if in LoadTestConfig screen - handle numeric input
                        if matches!(app.current_screen, Screen::LoadTestConfig(_, _)) {
                            if let Some(form) = &mut app.load_test_config_form {
                                // Only allow digits for numeric fields
                                if c.is_ascii_digit() {
                                    match form.current_field {
                                        0 => form.concurrency.push(c),
                                        1 => form.duration.push(c),
                                        2 => form.ramp_up.push(c),
                                        _ => {}
                                    }
                                }
                            }
                            continue;
                        }
                        
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
                                    // New collection or endpoint based on panel focus
                                    match app.panel_focus {
                                        crate::tui_app::PanelFocus::Collections => {
                                            app.start_new_collection();
                                        }
                                        crate::tui_app::PanelFocus::Endpoints => {
                                            app.start_new_endpoint(app.selected_collection_index);
                                        }
                                    }
                                }
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
                                        // Edit collection or endpoint based on panel focus
                                        match app.panel_focus {
                                            crate::tui_app::PanelFocus::Collections => {
                                                if app.selected_collection_index < app.collections.len() {
                                                    app.start_edit_collection(app.selected_collection_index);
                                                }
                                            }
                                            crate::tui_app::PanelFocus::Endpoints => {
                                                if let Some(collection) = app.collections.get(app.selected_collection_index) {
                                                    if app.selected_endpoint_index < collection.endpoints.len() {
                                                        app.start_edit_endpoint(
                                                            app.selected_collection_index,
                                                            app.selected_endpoint_index
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                'd' => {
                                    // Delete collection or endpoint based on panel focus
                                    match app.panel_focus {
                                        crate::tui_app::PanelFocus::Collections => {
                                            if app.selected_collection_index < app.collections.len() {
                                                app.confirm_delete_collection(app.selected_collection_index);
                                            }
                                        }
                                        crate::tui_app::PanelFocus::Endpoints => {
                                            if let Some(collection) = app.collections.get(app.selected_collection_index) {
                                                if app.selected_endpoint_index < collection.endpoints.len() {
                                                    app.confirm_delete_endpoint(
                                                        app.selected_collection_index,
                                                        app.selected_endpoint_index
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
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
                                't' => {
                                    // Toggle network traffic display
                                    app.toggle_network_traffic();
                                }
                                _ => {}
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        // Delete character in forms
                        match &app.current_screen {
                            Screen::LoadTestConfig(_, _) => {
                                if let Some(form) = &mut app.load_test_config_form {
                                    match form.current_field {
                                        0 => { form.concurrency.pop(); }
                                        1 => { form.duration.pop(); }
                                        2 => { form.ramp_up.pop(); }
                                        _ => {}
                                    }
                                }
                            }
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
                        } else if let Screen::LoadTestConfig(_, _) = app.current_screen {
                            if let Some(form) = &mut app.load_test_config_form {
                                form.current_field = (form.current_field + 1) % 3;
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
                        } else if let Screen::LoadTestConfig(_, _) = app.current_screen {
                            if let Some(form) = &mut app.load_test_config_form {
                                form.current_field = if form.current_field == 0 {
                                    2
                                } else {
                                    form.current_field - 1
                                };
                            }
                        }
                    }
                    KeyCode::PageUp => {
                        // Scroll response up (10 lines)
                        if !in_edit_screen {
                            app.scroll_response_up(10);
                        }
                    }
                    KeyCode::PageDown => {
                        // Scroll response down (10 lines)
                        if !in_edit_screen {
                            app.scroll_response_down(10);
                        }
                    }
                    KeyCode::Home => {
                        // Scroll to top of response
                        if !in_edit_screen {
                            app.reset_response_scroll();
                        }
                    }
                    KeyCode::End => {
                        // Scroll to bottom of response
                        if !in_edit_screen {
                            app.scroll_response_to_end();
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn draw_ui(f: &mut Frame, app: &AppState) {
    // For full-screen modes (edit, help, dialogs), use old layout
    let use_split_layout = matches!(
        app.current_screen,
        Screen::CollectionList | Screen::EndpointList(_) | Screen::EndpointDetail(_, _) | Screen::ResponseView(_, _)
    );
    
    if !use_split_layout {
        // Old full-screen layout for edit screens, help, etc.
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(f.area());

        draw_title(f, chunks[0]);
        
        match &app.current_screen {
            Screen::CollectionEdit(_) => draw_collection_edit(f, chunks[1], app),
            Screen::EndpointEdit(coll_idx, _) => draw_endpoint_edit(f, chunks[1], app, *coll_idx),
            Screen::LoadTestConfig(_, _) => draw_load_test_config(f, chunks[1], app),
            Screen::LoadTestRunning(coll_idx, ep_idx) => draw_load_test(f, chunks[1], app, *coll_idx, *ep_idx),
            Screen::ConfirmDelete(_) => draw_confirm_delete(f, chunks[1], app),
            Screen::Help => draw_help(f, chunks[1]),
            _ => {}
        }
        
        draw_footer(f, chunks[2], app);
        return;
    }
    
    // New split-panel layout (Option B)
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(0),     // Main content
            Constraint::Length(3),  // Footer
        ])
        .split(f.area());

    draw_title(f, main_chunks[0]);

    // Split main area horizontally: left (definition) and right (collections)
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(65),  // Left: API definition
            Constraint::Percentage(35),  // Right: Collections & Endpoints
        ])
        .split(main_chunks[1]);

    // Split left panel vertically: definition (top) and response (bottom)
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),  // API definition
            Constraint::Percentage(50),  // Response
        ])
        .split(horizontal_chunks[0]);

    // Draw the three panels
    draw_definition_panel(f, left_chunks[0], app);
    draw_response_panel(f, left_chunks[1], app);
    draw_collections_panel(f, horizontal_chunks[1], app);

    draw_footer(f, main_chunks[2], app);
}

fn draw_title(f: &mut Frame, area: Rect) {
    let title = Paragraph::new("🚀 REST API TUI - Terminal API Testing Tool ⚡")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(Color::Cyan)));
    f.render_widget(title, area);
}

fn draw_footer(f: &mut Frame, area: Rect, app: &AppState) {
    let text = if let Some(err) = &app.error_message {
        Line::from(vec![
            Span::styled("✗ Error: ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::styled(err, Style::default().fg(Color::Red)),
        ])
    } else if let Some(status) = &app.status_message {
        Line::from(vec![
            Span::styled("✓ ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled(status, Style::default().fg(Color::Green)),
        ])
    } else {
        Line::from("⌨ Ctrl+h/l: panels | Ctrl+j/k: nav | PgUp/PgDn: scroll | t: traffic | ?: help")
    };
    
    let footer = Paragraph::new(text)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::DarkGray)));
    f.render_widget(footer, area);
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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
                Constraint::Length(24),  // Increased from 12 to fit charts
                Constraint::Min(0),
            ])
            .split(area);

        // Progress with animation
        let _progress = if let Some(engine) = &app.load_test_engine {
            let elapsed = engine.elapsed();
            let elapsed_secs = elapsed.as_secs_f64();
            let total = app.load_test_config.duration.as_secs_f64();
            let percent = (elapsed_secs / total * 100.0).min(100.0) as u16;
            
            // Animated spinner and pulse color
            let spinner = get_spinner(elapsed.as_millis());
            let pulse_color = get_pulse_color(elapsed.as_millis());
            
            // Create gradient progress bar
            let elapsed_str = format!("{}s", elapsed.as_secs());
            let total_str = format!("{}s", app.load_test_config.duration.as_secs());
            let title = format!("🚀 {} Load Test Progress - {} / {} ⚡", spinner, elapsed_str, total_str);
            
            let gauge = Gauge::default()
                .block(Block::default()
                    .title(title)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(pulse_color)))
                .gauge_style(Style::default().fg(Color::Green).bg(Color::DarkGray))
                .percent(percent);
            f.render_widget(gauge, chunks[0]);
            
            percent
        } else {
            0
        };

        // Calculate percentiles
        let percentiles = crate::load_test::calculate_percentiles(&metrics.latencies);
        let avg_latency = if !metrics.latencies.is_empty() {
            let total: std::time::Duration = metrics.latencies.iter().sum();
            total / metrics.latencies.len() as u32
        } else {
            std::time::Duration::default()
        };

        // Calculate success rate
        let success_rate = if metrics.total_requests > 0 {
            (metrics.successful_requests as f64 / metrics.total_requests as f64) * 100.0
        } else {
            0.0
        };
        
        let failure_rate = if metrics.total_requests > 0 {
            (metrics.failed_requests as f64 / metrics.total_requests as f64) * 100.0
        } else {
            0.0
        };

        // Stats with percentiles, icons, and percentages
        let mut stats_text = vec![
            Line::from(vec![
                Span::styled("📨 Total Requests: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled(format!("{}", metrics.total_requests), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("✓ Successful: ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::styled(format!("{}", metrics.successful_requests), Style::default().fg(Color::Green)),
                Span::styled(format!(" ({:.1}%)", success_rate), Style::default().fg(Color::DarkGray)),
            ]),
            Line::from(vec![
                Span::styled("✗ Failed: ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::styled(format!("{}", metrics.failed_requests), Style::default().fg(Color::Red)),
                Span::styled(format!(" ({:.1}%)", failure_rate), Style::default().fg(Color::DarkGray)),
            ]),
            Line::from(vec![
                Span::styled("⚡ Current RPS: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(format!("{:.2}", metrics.current_rps), Style::default().fg(Color::Yellow)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("📊 Latency Percentiles:", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("  Avg: ", Style::default().fg(Color::Gray)),
                Span::styled(format!("{:?}", avg_latency), Style::default().fg(Color::White)),
                Span::raw("  "),
                Span::styled("p50: ", Style::default().fg(Color::Gray)),
                Span::styled(format!("{:?}", percentiles.p50), Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::styled("  p90: ", Style::default().fg(Color::Gray)),
                Span::styled(format!("{:?}", percentiles.p90), Style::default().fg(Color::Yellow)),
                Span::raw("  "),
                Span::styled("p95: ", Style::default().fg(Color::Gray)),
                Span::styled(format!("{:?}", percentiles.p95), Style::default().fg(Color::Magenta)),
            ]),
            Line::from(vec![
                Span::styled("  p99: ", Style::default().fg(Color::Gray)),
                Span::styled(format!("{:?}", percentiles.p99), Style::default().fg(Color::Red)),
                Span::raw("  "),
                Span::styled("Max: ", Style::default().fg(Color::Gray)),
                Span::styled(format!("{:?}", percentiles.max), Style::default().fg(Color::Red)),
            ]),
        ];

        // Add time-series charts if we have data
        if !metrics.time_series.is_empty() {
            stats_text.push(Line::from(""));
            stats_text.push(Line::from(vec![
                Span::styled("📈 Trends (5s intervals):", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]));
        }

        // Split stats area to show text and sparklines side by side
        let stats_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),  // Text stats
                Constraint::Percentage(50),  // Sparklines
            ])
            .split(chunks[1]);

        // Draw text stats on the left with rounded border
        let stats_paragraph = Paragraph::new(stats_text)
            .block(Block::default()
                .title("📊 Statistics")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Magenta)));
        f.render_widget(stats_paragraph, stats_chunks[0]);

        // Draw sparklines on the right
        if !metrics.time_series.is_empty() {
            let sparkline_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(7),   // p95 latency chart
                    Constraint::Length(7),   // RPS chart
                    Constraint::Min(0),      // Spacer
                ])
                .split(stats_chunks[1]);

            // p95 Latency Sparkline with Y-axis labels
            let p95_data: Vec<u64> = metrics.time_series.iter()
                .map(|dp| dp.p95.as_millis() as u64)
                .collect();
            
            if !p95_data.is_empty() {
                let max_p95 = *p95_data.iter().max().unwrap_or(&1);
                let min_p95 = *p95_data.iter().min().unwrap_or(&0);
                
                // Split area for Y-axis labels and sparkline
                let p95_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Length(6),   // Y-axis labels
                        Constraint::Min(0),      // Sparkline
                    ])
                    .split(sparkline_chunks[0]);
                
                // Draw Y-axis labels
                let y_labels = vec![
                    Line::from(""),
                    Line::from(format!("{}ms", max_p95)),
                    Line::from(""),
                    Line::from(format!("{}ms", (max_p95 + min_p95) / 2)),
                    Line::from(""),
                    Line::from(format!("{}ms", min_p95)),
                ];
                let y_axis = Paragraph::new(y_labels)
                    .style(Style::default().fg(Color::DarkGray))
                    .alignment(ratatui::layout::Alignment::Right);
                f.render_widget(y_axis, p95_layout[0]);
                
                // Determine color based on latency
                let (sparkline_style, border_color) = if max_p95 < 100 {
                    (Style::default().fg(Color::Green), Color::Green)
                } else if max_p95 < 200 {
                    (Style::default().fg(Color::Yellow), Color::Yellow)
                } else {
                    (Style::default().fg(Color::Red), Color::Red)
                };
                
                let p95_sparkline = Sparkline::default()
                    .block(Block::default()
                        .title("📈 p95 Latency")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::default().fg(border_color)))
                    .data(&p95_data)
                    .style(sparkline_style);
                f.render_widget(p95_sparkline, p95_layout[1]);
            }

            // RPS Sparkline with Y-axis labels
            let rps_data: Vec<u64> = metrics.time_series.iter()
                .map(|dp| dp.rps as u64)
                .collect();
            
            if !rps_data.is_empty() {
                let max_rps = *rps_data.iter().max().unwrap_or(&1);
                let min_rps = *rps_data.iter().min().unwrap_or(&0);
                
                // Split area for Y-axis labels and sparkline
                let rps_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Length(6),   // Y-axis labels
                        Constraint::Min(0),      // Sparkline
                    ])
                    .split(sparkline_chunks[1]);
                
                // Draw Y-axis labels
                let y_labels = vec![
                    Line::from(""),
                    Line::from(format!("{}", max_rps)),
                    Line::from(""),
                    Line::from(format!("{}", (max_rps + min_rps) / 2)),
                    Line::from(""),
                    Line::from(format!("{}", min_rps)),
                ];
                let y_axis = Paragraph::new(y_labels)
                    .style(Style::default().fg(Color::DarkGray))
                    .alignment(ratatui::layout::Alignment::Right);
                f.render_widget(y_axis, rps_layout[0]);
                
                let rps_sparkline = Sparkline::default()
                    .block(Block::default()
                        .title("⚡ RPS")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::default().fg(Color::Cyan)))
                    .data(&rps_data)
                    .style(Style::default().fg(Color::Cyan));
                f.render_widget(rps_sparkline, rps_layout[1]);
            }
        }

        // Enhanced Chart with icons and percentages
        let data = vec![
            ("✓ Success", metrics.successful_requests),
            ("✗ Failed", metrics.failed_requests),
        ];

        let chart = BarChart::default()
            .block(Block::default()
                .title("📊 Results")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Green)))
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
            "✏️ Edit Collection [Enter: save | Esc: cancel]"
        } else {
            "➕ New Collection [Enter: save | Esc: cancel]"
        };
        
        let text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("📁 Collection Name: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled(&form.name, Style::default().fg(Color::Yellow)),
                Span::styled("_", Style::default().fg(Color::Yellow).add_modifier(Modifier::SLOW_BLINK)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("⌨️  Type to enter name, press Enter to save", Style::default().fg(Color::DarkGray)),
            ]),
        ];
        
        let paragraph = Paragraph::new(text)
            .block(Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Cyan)))
            .wrap(Wrap { trim: true });
        
        f.render_widget(paragraph, area);
    }
}

fn draw_endpoint_edit(f: &mut Frame, area: Rect, app: &AppState, _coll_idx: usize) {
    if let Some(form) = &app.endpoint_form {
        let title = if form.editing_index.is_some() {
            "✏️ Edit Endpoint [Tab: next field | Enter: save | Esc: cancel]"
        } else {
            "➕ New Endpoint [Tab: next field | Enter: save | Esc: cancel]"
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
        
        let method_icon = match form.method {
            crate::models::HttpMethod::GET => "📥",
            crate::models::HttpMethod::POST => "📤",
            crate::models::HttpMethod::PUT => "✏️",
            crate::models::HttpMethod::DELETE => "🗑️",
            crate::models::HttpMethod::PATCH => "🔧",
            _ => "📨",
        };
        
        let mut text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("📝 Name: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled(&form.name, field_style(0)),
                Span::styled(cursor(0), field_style(0).add_modifier(Modifier::SLOW_BLINK)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(format!("{} Method: ", method_icon), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled(format!("{:?}", form.method), field_style(1)),
                Span::styled(" (press 'm' to cycle)", Style::default().fg(Color::DarkGray)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("🌐 URL: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled(&form.url, field_style(2)),
                Span::styled(cursor(2), field_style(2).add_modifier(Modifier::SLOW_BLINK)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("📄 Description: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled(&form.description, field_style(3)),
                Span::styled(cursor(3), field_style(3).add_modifier(Modifier::SLOW_BLINK)),
            ]),
            Line::from(""),
        ];
        
        // Show headers section
        if form.header_edit_mode {
            // Header edit mode - show input fields
            text.push(Line::from(vec![
                Span::styled("📋 Headers (Edit Mode): ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            ]));
            text.push(Line::from(""));
            text.push(Line::from(vec![
                Span::styled("  🔑 Key: ", Style::default().fg(Color::Cyan)),
                Span::styled(&form.header_key, if form.header_edit_field == 0 { field_style(4) } else { Style::default() }),
                Span::styled(if form.header_edit_field == 0 { "_" } else { "" }, field_style(4).add_modifier(Modifier::SLOW_BLINK)),
            ]));
            text.push(Line::from(vec![
                Span::styled("  💎 Value: ", Style::default().fg(Color::Cyan)),
                Span::styled(&form.header_value, if form.header_edit_field == 1 { field_style(4) } else { Style::default() }),
                Span::styled(if form.header_edit_field == 1 { "_" } else { "" }, field_style(4).add_modifier(Modifier::SLOW_BLINK)),
            ]));
            text.push(Line::from(""));
            text.push(Line::from(vec![
                Span::styled("  ⌨️  Tab: switch field | Enter: add | Esc: cancel", Style::default().fg(Color::DarkGray)),
            ]));
        } else {
            // Normal mode - show existing headers
            text.push(Line::from(vec![
                Span::styled("📋 Headers: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
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
            Span::styled("📦 Body Template: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled(&form.body_template, field_style(5)),
            Span::styled(cursor(5), field_style(5).add_modifier(Modifier::SLOW_BLINK)),
        ]));
        text.push(Line::from(""));
        
        if !form.header_edit_mode {
            text.push(Line::from(vec![
                Span::styled("⌨️  Tab: next field | h: add header | Enter: save", Style::default().fg(Color::DarkGray)),
            ]));
        }
        
        let paragraph = Paragraph::new(text)
            .block(Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Cyan)))
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
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Red))
                .style(Style::default().bg(Color::Black)))
            .wrap(Wrap { trim: true })
            .alignment(ratatui::layout::Alignment::Center);
        
        f.render_widget(dialog, dialog_area);
    }
}

fn draw_help(f: &mut Frame, area: Rect) {
    let help_text = vec![
        Line::from(vec![Span::styled("⌨️  Keyboard Shortcuts", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))]),
        Line::from(""),
        Line::from(vec![Span::styled("🧭 Navigation:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
        Line::from("  ↑/k        - Move up"),
        Line::from("  ↓/j        - Move down"),
        Line::from("  Enter      - Select item"),
        Line::from("  Esc        - Go back"),
        Line::from("  q          - Quit (from main screen)"),
        Line::from(""),
        Line::from(vec![Span::styled("📁 Collection Management:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
        Line::from("  n          - New collection/endpoint"),
        Line::from("  e          - Edit collection/endpoint"),
        Line::from("  d          - Delete collection/endpoint"),
        Line::from(""),
        Line::from(vec![Span::styled("🚀 Endpoint Actions:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
        Line::from("  e          - Execute request (from detail)"),
        Line::from("  l          - Start load test"),
        Line::from(""),
        Line::from(vec![Span::styled("✏️ Form Editing:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
        Line::from("  Tab        - Next field"),
        Line::from("  m          - Cycle HTTP method"),
        Line::from("  Backspace  - Delete character"),
        Line::from("  Enter      - Save"),
        Line::from(""),
        Line::from(vec![Span::styled("🔧 Other:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
        Line::from("  ?          - Show this help"),
        Line::from(""),
        Line::from(vec![Span::styled("Press any key to close help", Style::default().fg(Color::DarkGray))]),
    ];

    let paragraph = Paragraph::new(help_text)
        .block(Block::default()
            .title("❓ Help")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Yellow)))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

fn draw_load_test_config(f: &mut Frame, area: Rect, app: &AppState) {
    if let Some(form) = &app.load_test_config_form {
        let title = "⚙️ Load Test Configuration [Tab: next field | Enter: start | Esc: cancel]";
        
        let concurrency_style = if form.current_field == 0 {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        
        let duration_style = if form.current_field == 1 {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        
        let ramp_up_style = if form.current_field == 2 {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        
        let cursor = if form.current_field == 0 && !form.concurrency.is_empty() 
            || form.current_field == 1 && !form.duration.is_empty()
            || form.current_field == 2 && !form.ramp_up.is_empty() {
            ""
        } else {
            "_"
        };
        
        let mut text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("🔧 Configure load test parameters:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("👥 Concurrency (workers): ", Style::default().fg(Color::Cyan)),
                Span::styled(&form.concurrency, concurrency_style),
                Span::styled(if form.current_field == 0 { cursor } else { "" }, concurrency_style.add_modifier(Modifier::SLOW_BLINK)),
            ]),
            Line::from(vec![
                Span::styled("   Number of concurrent workers (1-1000)", Style::default().fg(Color::DarkGray)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("⏱️  Duration (seconds): ", Style::default().fg(Color::Cyan)),
                Span::styled(&form.duration, duration_style),
                Span::styled(if form.current_field == 1 { cursor } else { "" }, duration_style.add_modifier(Modifier::SLOW_BLINK)),
            ]),
            Line::from(vec![
                Span::styled("   Total test duration (1-3600)", Style::default().fg(Color::DarkGray)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("📈 Ramp-up (seconds): ", Style::default().fg(Color::Cyan)),
                Span::styled(if form.ramp_up.is_empty() { "(optional)" } else { &form.ramp_up }, ramp_up_style),
                Span::styled(if form.current_field == 2 { cursor } else { "" }, ramp_up_style.add_modifier(Modifier::SLOW_BLINK)),
            ]),
            Line::from(vec![
                Span::styled("   Gradually increase load over this period", Style::default().fg(Color::DarkGray)),
            ]),
            Line::from(""),
            Line::from(""),
        ];
        
        // Show preview
        let concurrency_val = form.concurrency.parse::<usize>().unwrap_or(10);
        let duration_val = form.duration.parse::<u64>().unwrap_or(30);
        let ramp_up_val = if form.ramp_up.is_empty() {
            None
        } else {
            form.ramp_up.parse::<u64>().ok()
        };
        
        text.push(Line::from(vec![
            Span::styled("👁️  Preview:", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ]));
        text.push(Line::from(format!("   {} workers will execute requests for {} seconds", concurrency_val, duration_val)));
        
        if let Some(ramp_up) = ramp_up_val {
            text.push(Line::from(format!("   Load will ramp up over {} seconds", ramp_up)));
            text.push(Line::from(format!("   Expected total requests: ~{}", concurrency_val * (duration_val - ramp_up / 2) as usize)));
        } else {
            text.push(Line::from(format!("   Expected total requests: ~{}", concurrency_val * duration_val as usize)));
        }
        
        let paragraph = Paragraph::new(text)
            .block(Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Magenta)))
            .wrap(Wrap { trim: true });
        
        f.render_widget(paragraph, area);
    }
}


// New split-panel drawing functions for Option B layout

fn draw_definition_panel(f: &mut Frame, area: Rect, app: &AppState) {
    use crate::tui_app::PanelFocus;
    
    let is_focused = app.panel_focus == PanelFocus::Collections || app.panel_focus == PanelFocus::Endpoints;
    let border_style = if !is_focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    
    // Check if an endpoint is selected
    if let Some(collection) = app.collections.get(app.selected_collection_index) {
        if let Some(endpoint) = collection.endpoints.get(app.selected_endpoint_index) {
            // Show endpoint details
            let method_icon = match endpoint.method {
                crate::models::HttpMethod::GET => "📥",
                crate::models::HttpMethod::POST => "📤",
                crate::models::HttpMethod::PUT => "✏️",
                crate::models::HttpMethod::DELETE => "🗑️",
                crate::models::HttpMethod::PATCH => "🔧",
                _ => "📨",
            };
            
            let mut text = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled(format!("{} Method: ", method_icon), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                    Span::styled(format!("{:?}", endpoint.method), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(vec![
                    Span::styled("🌐 URL: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                    Span::raw(&endpoint.url),
                ]),
                Line::from(""),
            ];
            
            if let Some(desc) = &endpoint.description {
                text.push(Line::from(vec![
                    Span::styled("📄 Description: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                    Span::raw(desc),
                ]));
                text.push(Line::from(""));
            }
            
            text.push(Line::from(vec![
                Span::styled("📋 Headers:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]));
            
            if endpoint.headers.is_empty() {
                text.push(Line::from("  (none)"));
            } else {
                for (key, value) in &endpoint.headers {
                    text.push(Line::from(format!("  {}: {}", key, value)));
                }
            }
            
            if let Some(auth) = &endpoint.auth {
                text.push(Line::from(""));
                text.push(Line::from(vec![
                    Span::styled("🔐 Authentication: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                    Span::raw(format!("{:?}", auth)),
                ]));
            }
            
            if let Some(body) = &endpoint.body_template {
                text.push(Line::from(""));
                text.push(Line::from(vec![
                    Span::styled("📦 Body:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                ]));
                text.push(Line::from(format!("  {}", body)));
            }
            
            text.push(Line::from(""));
            text.push(Line::from(""));
            text.push(Line::from(vec![
                Span::styled("🚀 Actions:", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            ]));
            text.push(Line::from("  [e] Execute Request"));
            text.push(Line::from("  [l] Start Load Test"));

            let paragraph = Paragraph::new(text)
                .block(Block::default()
                    .title(format!("📍 Endpoint: {}", endpoint.name))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(border_style))
                .wrap(Wrap { trim: true });

            f.render_widget(paragraph, area);
            return;
        }
    }
    
    // No endpoint selected - show placeholder
    let text = vec![
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled("📭 No endpoint selected", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Select a collection and endpoint from the right panel", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("⌨️  Use Ctrl+h/l to switch panels", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("⌨️  Use Ctrl+j/k to navigate", Style::default().fg(Color::DarkGray)),
        ]),
    ];
    
    let paragraph = Paragraph::new(text)
        .block(Block::default()
            .title("📍 API Definition")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_style))
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(paragraph, area);
}

fn draw_response_panel(f: &mut Frame, area: Rect, app: &AppState) {
    if let Some(response) = &app.last_response {
        // Show response with optional network traffic
        let traffic_toggle = if app.show_network_traffic { "hide" } else { "show" };
        let status_icon = if response.status.is_success() {
            "✓"
        } else if response.status.is_client_error() || response.status.is_server_error() {
            "✗"
        } else {
            "ℹ"
        };
        
        // Check if response is JSON
        let is_json = response.headers.iter()
            .any(|(k, v)| k.to_lowercase() == "content-type" && v.to_lowercase().contains("json"));
        
        let json_indicator = if is_json { " 🎨 JSON" } else { "" };
        
        let header_text = format!(
            "{} Response: {} - {:?} - {} bytes{} [t: {} traffic | PgUp/PgDn: scroll]",
            status_icon,
            response.status,
            response.duration,
            response.body.len(),
            json_indicator,
            traffic_toggle
        );
        
        if app.show_network_traffic && response.traffic.is_some() {
            // Split panel: response body (top) and network traffic (bottom)
            let sections = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(50),  // Response body
                    Constraint::Percentage(50),  // Network traffic
                ])
                .split(area);
            
            // Draw response body with scrolling
            let formatted_body = app.last_response_formatted.as_ref()
                .map(|s| s.as_str())
                .unwrap_or("(unable to format response)");
            
            // Get visible lines with optional JSON colorization
            let visible_lines = if is_json {
                let colored_lines = colorize_json(formatted_body);
                let total_lines = colored_lines.len();
                let visible_height = sections[0].height.saturating_sub(2) as usize;
                let max_scroll = if total_lines > visible_height {
                    total_lines - visible_height
                } else {
                    0
                };
                let scroll_offset = app.response_scroll_offset.min(max_scroll);
                
                colored_lines.into_iter()
                    .skip(scroll_offset)
                    .take(visible_height)
                    .collect::<Vec<Line>>()
            } else {
                let lines: Vec<&str> = formatted_body.lines().collect();
                let total_lines = lines.len();
                let visible_height = sections[0].height.saturating_sub(2) as usize;
                let max_scroll = if total_lines > visible_height {
                    total_lines - visible_height
                } else {
                    0
                };
                let scroll_offset = app.response_scroll_offset.min(max_scroll);
                
                lines.iter()
                    .skip(scroll_offset)
                    .take(visible_height)
                    .map(|line| Line::from(*line))
                    .collect()
            };
            
            // Calculate total lines for scroll indicator
            let total_lines = if is_json {
                colorize_json(formatted_body).len()
            } else {
                formatted_body.lines().count()
            };
            let visible_height = sections[0].height.saturating_sub(2) as usize;
            let scroll_offset = app.response_scroll_offset.min(if total_lines > visible_height {
                total_lines - visible_height
            } else {
                0
            });
            
            // Add scroll indicator if needed
            let title_with_scroll = if total_lines > visible_height {
                format!("{} [{}-{}/{}]", header_text, scroll_offset + 1, (scroll_offset + visible_height).min(total_lines), total_lines)
            } else {
                header_text.clone()
            };
            
            let body_paragraph = Paragraph::new(visible_lines)
                .block(Block::default()
                    .title(title_with_scroll)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Green)))
                .wrap(Wrap { trim: false });

            f.render_widget(body_paragraph, sections[0]);
            
            // Draw network traffic
            draw_network_traffic(f, sections[1], response);
        } else {
            // Show only response body with scrolling
            let formatted_body = app.last_response_formatted.as_ref()
                .map(|s| s.as_str())
                .unwrap_or("(unable to format response)");
            
            // Get visible lines with optional JSON colorization
            let visible_lines = if is_json {
                let colored_lines = colorize_json(formatted_body);
                let total_lines = colored_lines.len();
                let visible_height = area.height.saturating_sub(2) as usize;
                let max_scroll = if total_lines > visible_height {
                    total_lines - visible_height
                } else {
                    0
                };
                let scroll_offset = app.response_scroll_offset.min(max_scroll);
                
                colored_lines.into_iter()
                    .skip(scroll_offset)
                    .take(visible_height)
                    .collect::<Vec<Line>>()
            } else {
                let lines: Vec<&str> = formatted_body.lines().collect();
                let total_lines = lines.len();
                let visible_height = area.height.saturating_sub(2) as usize;
                let max_scroll = if total_lines > visible_height {
                    total_lines - visible_height
                } else {
                    0
                };
                let scroll_offset = app.response_scroll_offset.min(max_scroll);
                
                lines.iter()
                    .skip(scroll_offset)
                    .take(visible_height)
                    .map(|line| Line::from(*line))
                    .collect()
            };
            
            // Calculate total lines for scroll indicator
            let total_lines = if is_json {
                colorize_json(formatted_body).len()
            } else {
                formatted_body.lines().count()
            };
            let visible_height = area.height.saturating_sub(2) as usize;
            let scroll_offset = app.response_scroll_offset.min(if total_lines > visible_height {
                total_lines - visible_height
            } else {
                0
            });
            
            // Add scroll indicator if needed
            let title_with_scroll = if total_lines > visible_height {
                format!("{} [{}-{}/{}]", header_text, scroll_offset + 1, (scroll_offset + visible_height).min(total_lines), total_lines)
            } else {
                header_text
            };
            
            let paragraph = Paragraph::new(visible_lines)
                .block(Block::default()
                    .title(title_with_scroll)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Green)))
                .wrap(Wrap { trim: false });

            f.render_widget(paragraph, area);
        }
    } else {
        // No response yet
        let text = vec![
            Line::from(""),
            Line::from(""),
            Line::from(vec![
                Span::styled("📭 No response yet", Style::default().fg(Color::DarkGray)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Execute a request to see the response here", Style::default().fg(Color::DarkGray)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("⌨️  Press 't' to toggle network traffic view", Style::default().fg(Color::DarkGray)),
            ]),
        ];
        
        let paragraph = Paragraph::new(text)
            .block(Block::default()
                .title("📨 Response")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray)))
            .alignment(ratatui::layout::Alignment::Center);

        f.render_widget(paragraph, area);
    }
}

fn draw_network_traffic(f: &mut Frame, area: Rect, response: &crate::http::HttpResponse) {
    if let Some(traffic) = &response.traffic {
        let mut lines = vec![
            Line::from(vec![
                Span::styled("📡 Network Traffic ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled("(Wireshark-style)", Style::default().fg(Color::DarkGray)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("⏱️  Timing Breakdown:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
        ];
        
        // Timing information
        if let Some(dns) = traffic.timing.dns_lookup {
            lines.push(Line::from(format!("  🔍 DNS Lookup:        {:?}", dns)));
        }
        if let Some(tcp) = traffic.timing.tcp_connect {
            lines.push(Line::from(format!("  🔌 TCP Connect:       {:?}", tcp)));
        }
        if let Some(tls) = traffic.timing.tls_handshake {
            lines.push(Line::from(format!("  🔐 TLS Handshake:     {:?}", tls)));
        }
        lines.push(Line::from(format!("  📤 Request Sent:      {:?}", traffic.timing.request_sent)));
        lines.push(Line::from(format!("  ⏳ Waiting (TTFB):    {:?}", traffic.timing.waiting)));
        lines.push(Line::from(format!("  📥 Content Download:  {:?}", traffic.timing.content_download)));
        lines.push(Line::from(vec![
            Span::styled("  ⚡ Total:             ", Style::default()),
            Span::styled(format!("{:?}", traffic.timing.total), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ]));
        
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("📤 Request:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]));
        lines.push(Line::from(format!("  {} {}", traffic.request.method, traffic.request.url)));
        lines.push(Line::from(format!("  📋 Headers: {} ({} bytes)", 
            traffic.request.headers.len(),
            traffic.request.headers.iter().map(|(k, v)| k.len() + v.len() + 4).sum::<usize>()
        )));
        
        // Show first few headers
        let mut header_count = 0;
        for (key, value) in &traffic.request.headers {
            if header_count < 3 {
                let display_value = if value.len() > 50 {
                    format!("{}...", &value[..50])
                } else {
                    value.clone()
                };
                lines.push(Line::from(format!("    {}: {}", key, display_value)));
                header_count += 1;
            }
        }
        if traffic.request.headers.len() > 3 {
            lines.push(Line::from(format!("    ... and {} more", traffic.request.headers.len() - 3)));
        }
        
        lines.push(Line::from(format!("  📦 Body: {} bytes", traffic.request.body_size)));
        
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("📥 Response:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]));
        lines.push(Line::from(format!("  ✓ Status: {}", response.status)));
        lines.push(Line::from(format!("  📋 Headers: {} ({} bytes)", 
            response.headers.len(),
            traffic.response_headers_size
        )));
        lines.push(Line::from(format!("  📦 Body: {} bytes", traffic.response_body_size)));
        
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("📊 Total Transfer: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{} bytes", 
                    traffic.request.body_size + traffic.response_headers_size + traffic.response_body_size
                ),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            ),
        ]));
        
        let paragraph = Paragraph::new(lines)
            .block(Block::default()
                .title("📡 Network Traffic")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Cyan)))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }
}

fn draw_collections_panel(f: &mut Frame, area: Rect, app: &AppState) {
    use crate::tui_app::PanelFocus;
    
    // Split into two sections: collections (top) and endpoints (bottom)
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),  // Collections
            Constraint::Percentage(60),  // Endpoints
        ])
        .split(area);
    
    // Draw collections section
    let collections_focused = app.panel_focus == PanelFocus::Collections;
    let collections_border_style = if collections_focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    
    let collection_items: Vec<ListItem> = app
        .collections
        .iter()
        .enumerate()
        .map(|(i, collection)| {
            let style = if i == app.selected_collection_index && collections_focused {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else if i == app.selected_collection_index {
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            
            let content = format!("📁 {} ({} endpoints)", collection.name, collection.endpoints.len());
            ListItem::new(content).style(style)
        })
        .collect();

    let collections_title = if collections_focused {
        "📁 Collections [n: new | e: edit | d: delete]"
    } else {
        "📁 Collections"
    };
    
    let collections_list = List::new(collection_items)
        .block(Block::default()
            .title(collections_title)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(collections_border_style))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("▶ ");

    f.render_widget(collections_list, sections[0]);
    
    // Draw endpoints section
    let endpoints_focused = app.panel_focus == PanelFocus::Endpoints;
    let endpoints_border_style = if endpoints_focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    
    if let Some(collection) = app.collections.get(app.selected_collection_index) {
        let endpoint_items: Vec<ListItem> = collection
            .endpoints
            .iter()
            .enumerate()
            .map(|(i, endpoint)| {
                let style = if i == app.selected_endpoint_index && endpoints_focused {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else if i == app.selected_endpoint_index {
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                
                let (method_icon, method_color) = match endpoint.method {
                    crate::models::HttpMethod::GET => ("📥", Color::Green),
                    crate::models::HttpMethod::POST => ("📤", Color::Blue),
                    crate::models::HttpMethod::PUT => ("✏️", Color::Yellow),
                    crate::models::HttpMethod::DELETE => ("🗑️", Color::Red),
                    crate::models::HttpMethod::PATCH => ("🔧", Color::Magenta),
                    _ => ("📨", Color::White),
                };
                
                let content = Line::from(vec![
                    Span::styled(format!("{} {:?} ", method_icon, endpoint.method), Style::default().fg(method_color).add_modifier(Modifier::BOLD)),
                    Span::raw(&endpoint.name),
                ]);
                
                ListItem::new(content).style(style)
            })
            .collect();

        let endpoints_title = if endpoints_focused {
            format!("🔗 Endpoints - {} [n: new | e: edit | d: delete]", collection.name)
        } else {
            format!("🔗 Endpoints - {}", collection.name)
        };
        
        let endpoints_list = List::new(endpoint_items)
            .block(Block::default()
                .title(endpoints_title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(endpoints_border_style))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("→ ");

        f.render_widget(endpoints_list, sections[1]);
    } else {
        // No collection selected
        let text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("📭 No collections", Style::default().fg(Color::DarkGray)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Press 'n' to create a new collection", Style::default().fg(Color::DarkGray)),
            ]),
        ];
        
        let paragraph = Paragraph::new(text)
            .block(Block::default()
                .title("🔗 Endpoints")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(endpoints_border_style))
            .alignment(ratatui::layout::Alignment::Center);

        f.render_widget(paragraph, sections[1]);
    }
}

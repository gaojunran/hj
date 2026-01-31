use std::io::stdout;
use std::process::{Command, Stdio};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap, BorderType},
};
use ansi_to_tui::IntoText;

/// The display mode for the upper panel
#[derive(Clone, Copy, PartialEq, Eq)]
enum UpperPanelMode {
    Log,
    Show,
}

impl UpperPanelMode {
    fn next(self) -> Self {
        match self {
            UpperPanelMode::Log => UpperPanelMode::Show,
            UpperPanelMode::Show => UpperPanelMode::Log,
        }
    }

    fn title(self) -> &'static str {
        match self {
            UpperPanelMode::Log => " 📋 hj log ",
            UpperPanelMode::Show => " 🔍 hj show ",
        }
    }

    fn command(self) -> Vec<&'static str> {
        match self {
            UpperPanelMode::Log => vec!["log", "--color=always"],
            UpperPanelMode::Show => vec!["show", "--color=always"],
        }
    }
}

/// Application state
struct App {
    /// Current input string
    input: String,
    /// Cursor position in input
    cursor_position: usize,
    /// Output from the last command
    command_output: String,
    /// Upper panel display mode
    upper_mode: UpperPanelMode,
    /// Upper panel content
    upper_content: String,
    /// Scroll offset for upper panel
    upper_scroll: u16,
    /// Scroll offset for lower panel
    lower_scroll: u16,
}

impl App {
    fn new() -> Self {
        let mut app = App {
            input: String::new(),
            cursor_position: 0,
            command_output: String::from(" Welcome to hj TUI! Type a command and press Enter.\n\n Examples:\n   log          - Show commit log\n   show         - Show current changes\n   desc -m \"msg\" - Set commit message"),
            upper_mode: UpperPanelMode::Log,
            upper_content: String::new(),
            upper_scroll: 0,
            lower_scroll: 0,
        };
        // Initialize upper panel content
        app.refresh_upper_panel();
        app
    }

    fn refresh_upper_panel(&mut self) {
        self.upper_content = run_hj_command(&self.upper_mode.command());
        self.upper_scroll = 0;
    }

    fn execute_command(&mut self) {
        if self.input.trim().is_empty() {
            return;
        }

        // Parse the input into arguments, respecting quotes
        let args = parse_args(&self.input);
        let args_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        self.command_output = run_hj_command(&args_refs);
        self.lower_scroll = 0;

        // Clear input
        self.input.clear();
        self.cursor_position = 0;

        // Refresh upper panel after command execution
        self.refresh_upper_panel();
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.cursor_position)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        if self.cursor_position == 0 {
            return;
        }

        let current_index = self.cursor_position;
        let from_left_to_current_index = current_index - 1;

        let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
        let after_char_to_delete = self.input.chars().skip(current_index);

        self.input = before_char_to_delete.chain(after_char_to_delete).collect();
        self.move_cursor_left();
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn toggle_upper_mode(&mut self) {
        self.upper_mode = self.upper_mode.next();
        self.refresh_upper_panel();
    }
}

/// Parse command line arguments, respecting quoted strings
fn parse_args(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_double_quote = false;
    let mut in_single_quote = false;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' if !in_single_quote => {
                in_double_quote = !in_double_quote;
            }
            '\'' if !in_double_quote => {
                in_single_quote = !in_single_quote;
            }
            ' ' | '\t' if !in_double_quote && !in_single_quote => {
                if !current_arg.is_empty() {
                    args.push(current_arg.clone());
                    current_arg.clear();
                }
            }
            '\\' if in_double_quote => {
                // Handle escape sequences in double quotes
                if let Some(&next_c) = chars.peek() {
                    match next_c {
                        '"' | '\\' => {
                            current_arg.push(chars.next().unwrap());
                        }
                        _ => {
                            current_arg.push(c);
                        }
                    }
                } else {
                    current_arg.push(c);
                }
            }
            _ => {
                current_arg.push(c);
            }
        }
    }

    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    args
}

/// Run an hj command and capture its output
fn run_hj_command(args: &[&str]) -> String {
    // Get the current executable path
    let exe_path = std::env::current_exe().unwrap_or_else(|_| "hj".into());

    let output = Command::new(&exe_path)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            if !stderr.is_empty() {
                format!("{}\n{}", stdout, stderr)
            } else {
                stdout.to_string()
            }
        }
        Err(e) => format!("Error executing command: {}", e),
    }
}

pub fn command_tui() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut app = App::new();
    let result = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    result
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => return Ok(()),
                    KeyCode::Enter => app.execute_command(),
                    KeyCode::Char(c) => app.enter_char(c),
                    KeyCode::Backspace => app.delete_char(),
                    KeyCode::Left => app.move_cursor_left(),
                    KeyCode::Right => app.move_cursor_right(),
                    KeyCode::Tab => app.toggle_upper_mode(),
                    KeyCode::Up => {
                        app.upper_scroll = app.upper_scroll.saturating_sub(1);
                    }
                    KeyCode::Down => {
                        app.upper_scroll = app.upper_scroll.saturating_add(1);
                    }
                    KeyCode::PageUp => {
                        app.lower_scroll = app.lower_scroll.saturating_sub(10);
                    }
                    KeyCode::PageDown => {
                        app.lower_scroll = app.lower_scroll.saturating_add(10);
                    }
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    // Color scheme
    let title_style = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);
    let border_style = Style::default().fg(Color::Blue);
    let highlight_border_style = Style::default().fg(Color::Magenta);

    let outer_area = f.area();

    match app.upper_mode {
        UpperPanelMode::Show => {
            // Full screen mode for hj show
            let upper_text = app.upper_content.as_bytes().into_text().unwrap_or_default();
            let upper_paragraph = Paragraph::new(upper_text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(border_style)
                        .title(app.upper_mode.title())
                        .title_style(title_style)
                        .title_bottom(Line::from(vec![
                            Span::styled(" ⇥ Tab", Style::default().fg(Color::Yellow)),
                            Span::styled(": back to log │ ", Style::default().fg(Color::DarkGray)),
                            Span::styled("↑↓", Style::default().fg(Color::Yellow)),
                            Span::styled(": scroll │ ", Style::default().fg(Color::DarkGray)),
                            Span::styled("Esc", Style::default().fg(Color::Red)),
                            Span::styled(": quit ", Style::default().fg(Color::DarkGray)),
                        ]))
                )
                .wrap(Wrap { trim: false })
                .scroll((app.upper_scroll, 0));
            f.render_widget(upper_paragraph, outer_area);
        }
        UpperPanelMode::Log => {
            // Split screen mode for hj log
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(outer_area);

            // Upper panel: hj log
            let upper_text = app.upper_content.as_bytes().into_text().unwrap_or_default();
            let upper_paragraph = Paragraph::new(upper_text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(border_style)
                        .title(app.upper_mode.title())
                        .title_style(title_style)
                        .title_bottom(Line::from(vec![
                            Span::styled(" ⇥ Tab", Style::default().fg(Color::Yellow)),
                            Span::styled(": show │ ", Style::default().fg(Color::DarkGray)),
                            Span::styled("↑↓", Style::default().fg(Color::Yellow)),
                            Span::styled(": scroll │ ", Style::default().fg(Color::DarkGray)),
                            Span::styled("Esc", Style::default().fg(Color::Red)),
                            Span::styled(": quit ", Style::default().fg(Color::DarkGray)),
                        ]))
                )
                .wrap(Wrap { trim: false })
                .scroll((app.upper_scroll, 0));
            f.render_widget(upper_paragraph, chunks[0]);

            // Split lower panel into output and input
            let lower_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(3),
                    Constraint::Length(3),
                ])
                .split(chunks[1]);

            // Lower panel output area
            let output_text = app.command_output.as_bytes().into_text().unwrap_or_default();
            let output_paragraph = Paragraph::new(output_text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(border_style)
                        .title(" 📝 Output ")
                        .title_style(title_style)
                        .title_bottom(Line::from(vec![
                            Span::styled(" PgUp/PgDn", Style::default().fg(Color::Yellow)),
                            Span::styled(": scroll ", Style::default().fg(Color::DarkGray)),
                        ]))
                )
                .wrap(Wrap { trim: false })
                .scroll((app.lower_scroll, 0));
            f.render_widget(output_paragraph, lower_chunks[0]);

            // Input area with special styling
            let input_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(highlight_border_style)
                .title(" ⌨ Input ")
                .title_style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
                .title_bottom(Line::from(vec![
                    Span::styled(" hj ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                    Span::styled("<command> │ ", Style::default().fg(Color::DarkGray)),
                    Span::styled("Enter", Style::default().fg(Color::Yellow)),
                    Span::styled(": run ", Style::default().fg(Color::DarkGray)),
                ]));

            // Show input with prompt
            let input_text = Line::from(vec![
                Span::styled("❯ ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::styled(&app.input, Style::default().fg(Color::White)),
            ]);
            
            let input_paragraph = Paragraph::new(input_text)
                .block(input_block);
            f.render_widget(input_paragraph, lower_chunks[1]);

            // Show cursor (offset by 2 for the prompt "❯ ")
            let cursor_x = lower_chunks[1].x + app.cursor_position as u16 + 3; // +1 for border, +2 for prompt
            let cursor_y = lower_chunks[1].y + 1;
            f.set_cursor_position((cursor_x, cursor_y));
        }
    }
}

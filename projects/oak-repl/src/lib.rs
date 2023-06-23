#![warn(missing_docs)]
//! Oak REPL (Read-Eval-Print Loop) framework.
//!
//! A REPL framework deeply integrated with Oak language features.
//! Supports multi-line input, syntax integrity checking, and custom highlighting.

use crossterm::{
    cursor::MoveToColumn,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType},
};
use oak_highlight::{AnsiExporter, Exporter, HighlightResult, OakHighlighter};
use std::io::{self, Write};

use std::{
    error::Error,
    fmt::{Display, Formatter},
};

/// Errors that can occur during REPL execution.
#[derive(Debug)]
pub enum ReplError {
    /// An I/O error occurred.
    Io(std::io::Error),
    /// A custom error occurred.
    Other(String),
}

impl Display for ReplError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReplError::Io(e) => write!(f, "IO error: {}", e),
            ReplError::Other(s) => write!(f, "{}", s),
        }
    }
}

impl Error for ReplError {}

impl From<std::io::Error> for ReplError {
    fn from(e: std::io::Error) -> Self {
        ReplError::Io(e)
    }
}

impl From<String> for ReplError {
    fn from(s: String) -> Self {
        ReplError::Other(s)
    }
}

impl From<&str> for ReplError {
    fn from(s: &str) -> Self {
        ReplError::Other(s.to_string())
    }
}

/// The result of handling a line in the REPL.
pub enum HandleResult {
    /// Continue the REPL session.
    Continue,
    /// Exit the REPL session.
    Exit,
}

/// Interface for language integration in the REPL.
pub trait ReplHandler {
    /// Get syntax highlighting for the given code.
    fn highlight<'a>(&self, _code: &'a str) -> Option<HighlightResult<'a>> {
        None
    }

    /// The prompt to display. `is_continuation` is true for multi-line input.
    fn prompt(&self, is_continuation: bool) -> &str;

    /// Check if the input is complete (e.g., all brackets are closed).
    /// If it returns false, the REPL will enter multi-line input mode.
    fn is_complete(&self, code: &str) -> bool;

    /// Execute the given line of code.
    fn handle_line(&mut self, line: &str) -> Result<HandleResult, ReplError>;

    /// Get the current indentation level for the next line in multi-line mode.
    fn get_indent(&self, _code: &str) -> usize {
        // No indentation by default
        0
    }
}

/// A buffer for managing lines of text in the REPL.
pub struct LineBuffer {
    /// The lines of text in the buffer.
    lines: Vec<String>,
    /// The index of the current line being edited.
    current_line: usize,
    /// The cursor position within the current line.
    cursor_pos: usize,
}

impl LineBuffer {
    /// Create a new empty line buffer.
    pub fn new() -> Self {
        Self { lines: vec![String::new()], current_line: 0, cursor_pos: 0 }
    }

    /// Insert a character at the current cursor position.
    pub fn insert(&mut self, ch: char) {
        self.lines[self.current_line].insert(self.cursor_pos, ch);
        self.cursor_pos += 1;
    }

    /// Remove a character before the current cursor position.
    pub fn backspace(&mut self) -> bool {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.lines[self.current_line].remove(self.cursor_pos);
            true
        }
        else if self.current_line > 0 {
            // Merge with the previous line
            let current = self.lines.remove(self.current_line);
            self.current_line -= 1;
            self.cursor_pos = self.lines[self.current_line].chars().count();
            self.lines[self.current_line].push_str(&current);
            true
        }
        else {
            false
        }
    }

    /// Get the full text content of the buffer.
    pub fn full_text(&self) -> String {
        self.lines.join("\n")
    }

    /// Clear the buffer.
    pub fn clear(&mut self) {
        self.lines = vec![String::new()];
        self.current_line = 0;
        self.cursor_pos = 0;
    }

    /// Returns true if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.lines.len() == 1 && self.lines[0].is_empty()
    }
}

/// The main REPL engine.
pub struct OakRepl<H: ReplHandler> {
    /// The handler that implements language-specific logic.
    handler: H,
}

impl<H: ReplHandler> OakRepl<H> {
    /// Create a new Oak REPL with the given handler.
    pub fn new(handler: H) -> Self {
        Self { handler }
    }

    /// Run the REPL loop.
    pub fn run(&mut self) -> Result<(), ReplError> {
        let mut stdout = io::stdout();
        let mut line_buf = LineBuffer::new();
        let mut is_continuation = false;
        let _highlighter = OakHighlighter::new();
        let exporter = AnsiExporter;

        terminal::enable_raw_mode()?;

        loop {
            // Draw the current line
            execute!(stdout, MoveToColumn(0), Clear(ClearType::CurrentLine))?;
            let prompt = self.handler.prompt(is_continuation);

            let current_line_text = &line_buf.lines[line_buf.current_line];

            // Syntax highlighting
            let displayed_text = if let Some(highlighted) = self.handler.highlight(current_line_text) { exporter.export(&highlighted) } else { current_line_text.clone() };

            write!(stdout, "{}{}", prompt, displayed_text)?;

            let cursor_col = (prompt.chars().count() + line_buf.cursor_pos) as u16;
            execute!(stdout, MoveToColumn(cursor_col))?;
            stdout.flush()?;

            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                        println!("\nInterrupted");
                        line_buf.clear();
                        is_continuation = false;
                        continue;
                    }
                    KeyCode::Char('d') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                        if line_buf.is_empty() {
                            println!("\nEOF");
                            break;
                        }
                    }
                    KeyCode::Char(ch) => {
                        line_buf.insert(ch);
                    }
                    KeyCode::Enter => {
                        let full_code = line_buf.full_text();

                        if self.handler.is_complete(&full_code) {
                            terminal::disable_raw_mode()?;
                            println!();

                            match self.handler.handle_line(&full_code) {
                                Ok(HandleResult::Exit) => break,
                                Ok(HandleResult::Continue) => {}
                                Err(e) => eprintln!("Error: {}", e),
                            }

                            line_buf.clear();
                            is_continuation = false;
                            terminal::enable_raw_mode()?;
                        }
                        else {
                            // Continue multi-line input
                            println!();
                            line_buf.lines.push(String::new());
                            line_buf.current_line += 1;
                            line_buf.cursor_pos = 0;
                            is_continuation = true;

                            // Auto-indent
                            let indent_size = self.handler.get_indent(&full_code);
                            for _ in 0..indent_size {
                                line_buf.insert(' ');
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        line_buf.backspace();
                    }
                    KeyCode::Left => {
                        if line_buf.cursor_pos > 0 {
                            line_buf.cursor_pos -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if line_buf.cursor_pos < line_buf.lines[line_buf.current_line].chars().count() {
                            line_buf.cursor_pos += 1
                        }
                    }
                    _ => {}
                }
            }
        }

        terminal::disable_raw_mode()?;
        Ok(())
    }
}

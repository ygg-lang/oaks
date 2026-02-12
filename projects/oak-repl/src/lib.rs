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
///
/// This enum covers I/O errors from the terminal and custom errors
/// from the language integration layer.
#[derive(Debug)]
pub enum ReplError {
    /// An I/O error occurred during terminal communication or file access.
    Io(std::io::Error),
    /// A custom error occurred within the language-specific handler.
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
///
/// This indicates whether the REPL should continue running or terminate
/// after processing the current input.
pub enum HandleResult {
    /// Continue the REPL session and wait for the next input.
    Continue,
    /// Exit the REPL session immediately.
    Exit,
}

/// Interface for language integration in the REPL.
///
/// Implement this trait to provide language-specific behavior like
/// syntax highlighting, completion checking, and code execution.
///
/// # Usage Scenario
///
/// The `ReplHandler` is used by [`OakRepl`] to:
/// 1. Customize the prompt based on whether it's a new command or a continuation.
/// 2. Provide syntax highlighting for the current input line.
/// 3. Determine if a multi-line input is complete and ready for execution.
/// 4. Execute the collected code and decide whether to continue the REPL loop.
///
/// # Example
///
/// ```rust
/// use oak_repl::{HandleResult, ReplError, ReplHandler};
///
/// struct MyHandler;
///
/// impl ReplHandler for MyHandler {
///     fn prompt(&self, is_continuation: bool) -> &str {
///         if is_continuation { "... " } else { ">>> " }
///     }
///
///     fn is_complete(&self, code: &str) -> bool {
///         code.ends_with(';')
///     }
///
///     fn handle_line(&mut self, line: &str) -> Result<HandleResult, ReplError> {
///         println!("Executing: {}", line);
///         Ok(HandleResult::Continue)
///     }
/// }
/// ```
pub trait ReplHandler {
    /// Get syntax highlighting for the given code.
    ///
    /// Returns `None` if no highlighting should be applied.
    fn highlight<'a>(&self, _code: &'a str) -> Option<HighlightResult<'a>> {
        None
    }

    /// Returns the prompt string to display.
    ///
    /// `is_continuation` is true if the REPL is in multi-line input mode
    /// (i.e., the previous line was not complete).
    fn prompt(&self, is_continuation: bool) -> &str;

    /// Checks if the current input buffer represents a complete statement.
    ///
    /// If this returns `false`, the REPL will enter multi-line mode and
    /// allow the user to continue typing.
    fn is_complete(&self, code: &str) -> bool;

    /// Executes the given line (or multiple lines) of code.
    ///
    /// Returns a `HandleResult` indicating whether to continue or exit.
    fn handle_line(&mut self, line: &str) -> Result<HandleResult, ReplError>;

    /// Gets the current indentation level for the next line in multi-line mode.
    ///
    /// This is used for auto-indentation when the user presses Enter in
    /// the middle of a multi-line block.
    fn get_indent(&self, _code: &str) -> usize {
        // No indentation by default
        0
    }
}

/// A buffer for managing lines of text in the REPL.
///
/// `LineBuffer` handles single and multi-line input, cursor positioning,
/// and basic editing operations like insertion and backspace.
pub struct LineBuffer {
    /// The lines of text in the buffer.
    lines: Vec<String>,
    /// The index of the current line being edited.
    current_line: usize,
    /// The cursor position (character offset) within the current line.
    cursor_pos: usize,
}

impl LineBuffer {
    /// Creates a new empty `LineBuffer`.
    pub fn new() -> Self {
        Self { lines: vec![String::new()], current_line: 0, cursor_pos: 0 }
    }

    /// Inserts a character at the current cursor position.
    pub fn insert(&mut self, ch: char) {
        self.lines[self.current_line].insert(self.cursor_pos, ch);
        self.cursor_pos += 1;
    }

    /// Removes the character before the current cursor position (backspace).
    ///
    /// Returns `true` if a character or line was removed, `false` if the
    /// buffer was already at the very beginning.
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

    /// Returns the full text content of the buffer as a single string.
    ///
    /// Multiple lines are joined with newline characters.
    pub fn full_text(&self) -> String {
        self.lines.join("\n")
    }

    /// Clears the buffer and resets the cursor to the beginning.
    pub fn clear(&mut self) {
        self.lines = vec![String::new()];
        self.current_line = 0;
        self.cursor_pos = 0;
    }

    /// Returns `true` if the buffer is completely empty.
    pub fn is_empty(&self) -> bool {
        self.lines.len() == 1 && self.lines[0].is_empty()
    }
}

/// The main REPL engine.
///
/// `OakRepl` manages the terminal interface, handles user input,
/// and coordinates with a `ReplHandler` to provide language-specific
/// functionality.
///
/// # Example
///
/// ```no_run
/// use oak_repl::{HandleResult, OakRepl, ReplError, ReplHandler};
///
/// struct MyHandler;
/// impl ReplHandler for MyHandler {
///     fn prompt(&self, _: bool) -> &str {
///         "> "
///     }
///     fn is_complete(&self, code: &str) -> bool {
///         true
///     }
///     fn handle_line(&mut self, line: &str) -> Result<HandleResult, ReplError> {
///         println!("You typed: {}", line);
///         Ok(HandleResult::Continue)
///     }
/// }
///
/// let mut repl = OakRepl::new(MyHandler);
/// repl.run().expect("REPL failed");
/// ```
pub struct OakRepl<H: ReplHandler> {
    /// The handler that implements language-specific logic.
    handler: H,
}

impl<H: ReplHandler> OakRepl<H> {
    /// Creates a new `OakRepl` with the given handler.
    pub fn new(handler: H) -> Self {
        Self { handler }
    }

    /// Runs the REPL loop.
    ///
    /// This method takes control of the terminal (enabling raw mode)
    /// and blocks until the user exits or an unrecoverable error occurs.
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

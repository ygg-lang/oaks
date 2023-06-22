//! Oak REPL (Custom Implementation)
//!
//! 深度集成 Oak 语言特性的 REPL 框架。
//! 支持多行输入、语法完整性检查、以及自定义高亮。

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

#[derive(Debug)]
pub enum ReplError {
    Io(std::io::Error),
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

/// REPL 处理结果
pub enum HandleResult {
    /// 继续 REPL
    Continue,
    /// 退出 REPL
    Exit,
}

/// 语言集成接口
pub trait ReplHandler {
    /// 获取语法高亮结果
    fn highlight<'a>(&self, _code: &'a str) -> Option<HighlightResult<'a>> {
        None
    }

    /// 提示符
    fn prompt(&self, is_continuation: bool) -> &str;

    /// 检查输入是否已完整（例如括号是否闭合）
    /// 如果返回 false，REPL 将进入多行输入模式
    fn is_complete(&self, code: &str) -> bool;

    /// 执行代码行
    fn handle_line(&mut self, line: &str) -> Result<HandleResult, ReplError>;

    /// 获取当前缩进级别
    fn get_indent(&self, _code: &str) -> usize {
        // 默认不缩进
        0
    }
}

pub struct LineBuffer {
    lines: Vec<String>,
    current_line: usize,
    cursor_pos: usize,
}

impl LineBuffer {
    pub fn new() -> Self {
        Self { lines: vec![String::new()], current_line: 0, cursor_pos: 0 }
    }

    pub fn insert(&mut self, ch: char) {
        self.lines[self.current_line].insert(self.cursor_pos, ch);
        self.cursor_pos += 1;
    }

    pub fn backspace(&mut self) -> bool {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.lines[self.current_line].remove(self.cursor_pos);
            true
        }
        else if self.current_line > 0 {
            // 合并到上一行
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

    pub fn full_text(&self) -> String {
        self.lines.join("\n")
    }

    pub fn clear(&mut self) {
        self.lines = vec![String::new()];
        self.current_line = 0;
        self.cursor_pos = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.lines.len() == 1 && self.lines[0].is_empty()
    }
}

pub struct OakRepl<H: ReplHandler> {
    handler: H,
}

impl<H: ReplHandler> OakRepl<H> {
    pub fn new(handler: H) -> Self {
        Self { handler }
    }

    pub fn run(&mut self) -> Result<(), ReplError> {
        let mut stdout = io::stdout();
        let mut line_buf = LineBuffer::new();
        let mut is_continuation = false;
        let _highlighter = OakHighlighter::new();
        let exporter = AnsiExporter;

        terminal::enable_raw_mode()?;

        loop {
            // 绘制当前行
            execute!(stdout, MoveToColumn(0), Clear(ClearType::CurrentLine))?;
            let prompt = self.handler.prompt(is_continuation);

            let current_line_text = &line_buf.lines[line_buf.current_line];

            // 语法高亮
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
                            // 继续多行输入
                            println!();
                            line_buf.lines.push(String::new());
                            line_buf.current_line += 1;
                            line_buf.cursor_pos = 0;
                            is_continuation = true;

                            // 自动缩进
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
                            line_buf.cursor_pos += 1;
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

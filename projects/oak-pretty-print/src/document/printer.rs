use crate::{config::FormatConfig, document::Document};
use alloc::string::String;

/// Responsible for rendering a Document into a string
pub struct Printer {
    config: FormatConfig,
    output: String,
    indent_level: usize,
    column: usize,
}

impl Printer {
    /// Creates a new printer with the given configuration
    pub fn new(config: FormatConfig) -> Self {
        Self { config, output: String::new(), indent_level: 0, column: 0 }
    }

    /// Prints the document to a string
    pub fn print(mut self, doc: &Document<'_>) -> String {
        self.render(doc, false);
        self.finalize();
        self.output
    }

    fn finalize(&mut self) {
        if self.config.trim_trailing_whitespace {
            self.output = self.output.trim_end_matches([' ', '\t']).to_string()
        }
        if self.config.insert_final_newline && !self.output.is_empty() && !self.output.ends_with('\n') {
            self.output.push_str(self.config.line_ending_string())
        }
    }

    fn render(&mut self, doc: &Document<'_>, is_broken: bool) {
        match doc {
            Document::Nil => {}
            Document::Text(s) => {
                self.output.push_str(s);
                self.column += s.len()
            }
            Document::Concat(docs) => {
                for d in docs {
                    self.render(d, is_broken)
                }
            }
            Document::Group(d) => {
                // No longer forcibly inherit the broken state from the parent, but decide based on whether the current content overflows
                // This allows for more granular layout: when the parent is expanded, the child can still remain on a single line if it fits
                let should_break = self.will_break(d);
                self.render(d, should_break)
            }
            Document::Indent(d) => {
                self.indent_level += 1;
                self.render(d, is_broken);
                self.indent_level -= 1
            }
            Document::Line => {
                if is_broken {
                    self.newline()
                }
                else {
                    self.output.push(' ');
                    self.column += 1
                }
            }
            Document::SoftLine => {
                if is_broken {
                    self.newline()
                }
            }
            Document::SoftLineSpace => {
                if is_broken {
                    self.newline()
                }
                else {
                    self.output.push(' ');
                    self.column += 1
                }
            }
            Document::HardLine => self.newline(),
        }
    }

    fn newline(&mut self) {
        if self.config.trim_trailing_whitespace {
            while self.output.ends_with(' ') || self.output.ends_with('\t') {
                let _ = self.output.pop();
            }
        }
        self.output.push_str(self.config.line_ending_string());
        self.write_indent();
        self.column = self.indent_level * self.config.indent_size
    }

    fn write_indent(&mut self) {
        for _ in 0..self.indent_level {
            self.output.push_str(&self.config.indent_text)
        }
    }

    /// Simple width prediction logic
    fn will_break(&self, doc: &Document<'_>) -> bool {
        let mut width = self.column;
        self.check_width(doc, &mut width)
    }

    fn check_width(&self, doc: &Document<'_>, width: &mut usize) -> bool {
        if *width > self.config.max_width {
            return true;
        }

        match doc {
            Document::Nil => false,
            Document::Text(s) => {
                *width += s.len();
                *width > self.config.max_width
            }
            Document::Concat(docs) => {
                for d in docs {
                    if self.check_width(d, width) {
                        return true;
                    }
                }
                false
            }
            Document::Group(d) => self.check_width(d, width),
            Document::Indent(d) => self.check_width(d, width),
            Document::Line => {
                // In non-expanded mode, Line behaves as a space
                *width += 1;
                *width > self.config.max_width
            }
            Document::SoftLine => {
                // In non-expanded mode, SoftLine takes no space
                false
            }
            Document::SoftLineSpace => {
                // In non-expanded mode, SoftLineSpace behaves as a space
                *width += 1;
                *width > self.config.max_width
            }
            Document::HardLine => true, // HardLine means it must be expanded
        }
    }
}

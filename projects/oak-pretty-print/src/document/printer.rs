use crate::{config::FormatConfig, document::Doc};
use alloc::string::String;

/// 负责将 Document 渲染为字符串
pub struct Printer {
    config: FormatConfig,
    output: String,
    indent_level: usize,
    column: usize,
}

impl Printer {
    pub fn new(config: FormatConfig) -> Self {
        Self { config, output: String::new(), indent_level: 0, column: 0 }
    }

    pub fn print(mut self, doc: &Doc) -> String {
        self.render(doc, false);
        self.finalize();
        self.output
    }

    fn finalize(&mut self) {
        if self.config.trim_trailing_whitespace {
            self.output = self.output.trim_end_matches([' ', '\t']).to_string();
        }
        if self.config.insert_final_newline && !self.output.is_empty() && !self.output.ends_with('\n') {
            self.output.push_str(self.config.line_ending_string());
        }
    }

    fn render(&mut self, doc: &Doc, is_broken: bool) {
        match doc {
            Doc::Nil => {}
            Doc::Text(s) => {
                self.output.push_str(s);
                self.column += s.len();
            }
            Doc::Concat(docs) => {
                for d in docs {
                    self.render(d, is_broken);
                }
            }
            Doc::Group(d) => {
                // 不再强制继承父级的 broken 状态，而是根据当前内容是否溢出来决定
                // 这样可以实现更精细的布局：父级展开时，子级如果能放下则仍可保持单行
                let should_break = self.will_break(d);
                self.render(d, should_break);
            }
            Doc::Indent(d) => {
                self.indent_level += 1;
                self.render(d, is_broken);
                self.indent_level -= 1;
            }
            Doc::Line => {
                if is_broken {
                    self.newline();
                }
                else {
                    self.output.push(' ');
                    self.column += 1;
                }
            }
            Doc::SoftLine => {
                if is_broken {
                    self.newline();
                }
            }
            Doc::SoftLineSpace => {
                if is_broken {
                    self.newline();
                }
                else {
                    self.output.push(' ');
                    self.column += 1;
                }
            }
            Doc::HardLine => {
                self.newline();
            }
        }
    }

    fn newline(&mut self) {
        if self.config.trim_trailing_whitespace {
            while self.output.ends_with(' ') || self.output.ends_with('\t') {
                self.output.pop();
            }
        }
        self.output.push_str(self.config.line_ending_string());
        self.write_indent();
        self.column = self.indent_level * self.config.indent_size;
    }

    fn write_indent(&mut self) {
        for _ in 0..self.indent_level {
            self.output.push_str(&self.config.indent_text);
        }
    }

    /// 简单的宽度预测逻辑
    fn will_break(&self, doc: &Doc) -> bool {
        let mut width = self.column;
        self.check_width(doc, &mut width)
    }

    fn check_width(&self, doc: &Doc, width: &mut usize) -> bool {
        if *width > self.config.max_width {
            return true;
        }

        match doc {
            Doc::Nil => false,
            Doc::Text(s) => {
                *width += s.len();
                *width > self.config.max_width
            }
            Doc::Concat(docs) => {
                for d in docs {
                    if self.check_width(d, width) {
                        return true;
                    }
                }
                false
            }
            Doc::Group(d) => self.check_width(d, width),
            Doc::Indent(d) => self.check_width(d, width),
            Doc::Line => {
                // 在未展开模式下，Line 表现为空格
                *width += 1;
                *width > self.config.max_width
            }
            Doc::SoftLine => {
                // 在未展开模式下，SoftLine 不占空间
                false
            }
            Doc::SoftLineSpace => {
                // 在未展开模式下，SoftLineSpace 表现为空格
                *width += 1;
                *width > self.config.max_width
            }
            Doc::HardLine => true, // 强制换行意味着必须展开
        }
    }
}

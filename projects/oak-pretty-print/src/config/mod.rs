use alloc::string::{String, ToString};

/// 缩进样式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndentStyle {
    /// 使用空格
    Spaces(u8),
    /// 使用制表符
    Tabs,
}

impl Default for IndentStyle {
    fn default() -> Self {
        IndentStyle::Spaces(4)
    }
}

/// 行结束符
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnding {
    /// Unix 风格 (\n)
    Unix,
    /// Windows 风格 (\r\n)
    Windows,
    /// 自动检测
    Auto,
}

impl Default for LineEnding {
    fn default() -> Self {
        LineEnding::Auto
    }
}

/// 格式化配置
#[derive(Debug, Clone)]
pub struct FormatConfig {
    /// 缩进样式
    pub indent_style: IndentStyle,
    /// 行结束符
    pub line_ending: LineEnding,
    /// 最大行长度
    pub max_line_length: usize,
    /// 是否在文件末尾插入换行符
    pub insert_final_newline: bool,
    /// 是否修剪行尾空白
    pub trim_trailing_whitespace: bool,
    /// 是否保留空行
    pub preserve_blank_lines: bool,
    /// 最大连续空行数
    pub max_blank_lines: usize,
    /// 是否格式化注释
    pub format_comments: bool,
    /// 是否格式化字符串
    pub format_strings: bool,
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            indent_style: IndentStyle::default(),
            line_ending: LineEnding::default(),
            max_line_length: 100,
            insert_final_newline: true,
            trim_trailing_whitespace: true,
            preserve_blank_lines: true,
            max_blank_lines: 2,
            format_comments: true,
            format_strings: false,
        }
    }
}

impl FormatConfig {
    /// 创建新的配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置缩进样式
    pub fn with_indent_style(mut self, style: IndentStyle) -> Self {
        self.indent_style = style;
        self
    }

    /// 设置行结束符
    pub fn with_line_ending(mut self, ending: LineEnding) -> Self {
        self.line_ending = ending;
        self
    }

    /// 设置最大行长度
    pub fn with_max_line_length(mut self, length: usize) -> Self {
        self.max_line_length = length;
        self
    }

    /// 获取缩进字符串
    pub fn indent_string(&self) -> String {
        match self.indent_style {
            IndentStyle::Spaces(count) => " ".repeat(count as usize),
            IndentStyle::Tabs => "\t".to_string(),
        }
    }

    /// 获取行结束符字符串
    pub fn line_ending_string(&self) -> &'static str {
        match self.line_ending {
            LineEnding::Unix => "\n",
            LineEnding::Windows => "\r\n",
            LineEnding::Auto => {
                // 在实际使用中，应该根据输入文件检测
                #[cfg(windows)]
                return "\r\n";
                #[cfg(not(windows))]
                return "\n";
            }
        }
    }
}

use alloc::borrow::Cow;

/// 缩进样式
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
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
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatConfig {
    /// 缩进样式
    pub indent_style: IndentStyle,
    /// 缩进文本（缓存的单级缩进字符串）
    pub indent_text: Cow<'static, str>,
    /// 行结束符
    pub line_ending: LineEnding,
    /// 最大行长度
    pub max_width: usize,
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
    /// 缩进大小（用于列计算）
    pub indent_size: usize,
}

impl Default for FormatConfig {
    fn default() -> Self {
        let indent_style = IndentStyle::default();
        let (indent_text, indent_size) = match indent_style {
            IndentStyle::Spaces(count) => (" ".repeat(count as usize).into(), count as usize),
            IndentStyle::Tabs => ("\t".into(), 4), // Default tab size for column calculation
        };

        Self {
            indent_style,
            indent_text,
            line_ending: LineEnding::default(),
            max_width: 100,
            insert_final_newline: true,
            trim_trailing_whitespace: true,
            preserve_blank_lines: true,
            max_blank_lines: 2,
            format_comments: true,
            format_strings: false,
            indent_size,
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
        let (indent_text, indent_size) = match style {
            IndentStyle::Spaces(count) => (" ".repeat(count as usize).into(), count as usize),
            IndentStyle::Tabs => ("\t".into(), 4),
        };
        self.indent_text = indent_text;
        self.indent_size = indent_size;
        self
    }

    /// 设置行结束符
    pub fn with_line_ending(mut self, ending: LineEnding) -> Self {
        self.line_ending = ending;
        self
    }

    /// 设置最大行长度
    pub fn with_max_width(mut self, length: usize) -> Self {
        self.max_width = length;
        self
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

use alloc::borrow::Cow;

/// Indent style
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IndentStyle {
    /// Use spaces
    Spaces(u8),
    /// Use tabs
    Tabs,
}

impl Default for IndentStyle {
    fn default() -> Self {
        IndentStyle::Spaces(4)
    }
}

/// Line ending
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LineEnding {
    /// Unix style (\n)
    Unix,
    /// Windows style (\r\n)
    Windows,
    /// Auto detect
    Auto,
}

impl Default for LineEnding {
    fn default() -> Self {
        LineEnding::Auto
    }
}

/// Formatting configuration
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatConfig {
    /// Indent style
    pub indent_style: IndentStyle,
    /// Indent text (cached single-level indent string)
    pub indent_text: Cow<'static, str>,
    /// Line ending
    pub line_ending: LineEnding,
    /// Maximum line length
    pub max_width: usize,
    /// Whether to insert a final newline at the end of the file
    pub insert_final_newline: bool,
    /// Whether to trim trailing whitespace
    pub trim_trailing_whitespace: bool,
    /// Whether to preserve blank lines
    pub preserve_blank_lines: bool,
    /// Maximum consecutive blank lines
    pub max_blank_lines: usize,
    /// Whether to format comments
    pub format_comments: bool,
    /// Whether to format strings
    pub format_strings: bool,
    /// Indent size (used for column calculation)
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
    /// Creates a new configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the indent style
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

    /// Sets the line ending
    pub fn with_line_ending(mut self, ending: LineEnding) -> Self {
        self.line_ending = ending;
        self
    }

    /// Sets the maximum line length
    pub fn with_max_width(mut self, length: usize) -> Self {
        self.max_width = length;
        self
    }

    /// Gets the line ending string
    pub fn line_ending_string(&self) -> &'static str {
        match self.line_ending {
            LineEnding::Unix => "\n",
            LineEnding::Windows => "\r\n",
            LineEnding::Auto => {
                // In actual use, it should be detected based on the input file
                #[cfg(windows)]
                return "\r\n";
                #[cfg(not(windows))]
                return "\n";
            }
        }
    }
}

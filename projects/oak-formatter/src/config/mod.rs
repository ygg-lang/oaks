use alloc::borrow::Cow;

/// Indent style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "value"))]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
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

/// Common formatting configuration that can be shared across languages
///
/// This struct provides common formatting options that are applicable to most
/// programming languages. Language-specific formatters can use this as a base
/// and add their own specific options.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct CommonFormatterConfig {
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
}

impl Default for CommonFormatterConfig {
    fn default() -> Self {
        Self { indent_style: IndentStyle::default(), indent_text: Cow::Borrowed("    "), line_ending: LineEnding::default(), max_width: 80, insert_final_newline: true, trim_trailing_whitespace: true, preserve_blank_lines: true, max_blank_lines: 2 }
    }
}

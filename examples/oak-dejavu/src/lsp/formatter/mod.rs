#![doc = include_str!("readme.md")]
#[doc = include_str!("../readme.md")]

/// A code formatter for Dejavu source code.
///
/// This struct represents a Dejavu code formatter that can be used to
/// format and pretty-print Dejavu source code according to Dejavu style guidelines.
/// This is currently a placeholder implementation that will be extended with
/// proper formatting capabilities.
///
/// # Examples
///
/// ```rust
/// use oak_dejavu::formatter::DejavuFormatter;
///
/// let formatter = DejavuFormatter::new();
/// // Formatting functionality would be used here
/// ```
pub struct DejavuFormatter {}

impl DejavuFormatter {
    /// Creates a new Dejavu formatter instance.
    ///
    /// # Returns
    ///
    /// A new `DejavuFormatter` instance ready for use.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use oak_dejavu::formatter::DejavuFormatter;
    ///
    /// let formatter = DejavuFormatter::new();
    /// ```
    pub fn new() -> Self {
        Self {}
    }

    /// Formats the given Dejavu source code.
    ///
    /// This method takes a string containing Dejavu source code and returns
    /// a formatted version according to Dejavu style guidelines.
    ///
    /// # Arguments
    ///
    /// * `source` - A string slice containing the Dejavu source code to format
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted Dejavu source code.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use oak_dejavu::formatter::DejavuFormatter;
    ///
    /// let formatter = DejavuFormatter::new();
    /// let formatted = formatter.format("namespace Test { micro main() { let x = 42 } }");
    /// ```
    pub fn format(&self, source: &str) -> String {
        // TODO: Implement proper Dejavu code formatting
        // For now, return the source as-is
        source.to_string()
    }

    /// Formats a Dejavu namespace declaration.
    ///
    /// # Arguments
    ///
    /// * `name` - The namespace name
    /// * `body` - The namespace body content
    ///
    /// # Returns
    ///
    /// A formatted namespace declaration string.
    pub fn format_namespace(&self, name: &str, body: &str) -> String {
        format!("namespace {} {{\n{}\n}}", name, self.indent_lines(body))
    }

    /// Formats a Dejavu micro function declaration.
    ///
    /// # Arguments
    ///
    /// * `name` - The micro function name
    /// * `params` - The function parameters
    /// * `body` - The function body content
    ///
    /// # Returns
    ///
    /// A formatted micro function declaration string.
    pub fn format_micro_fn(&self, name: &str, params: &str, body: &str) -> String {
        format!("micro {}({}) {{\n{}\n}}", name, params, self.indent_lines(body))
    }

    /// Formats a Dejavu micro definition.
    ///
    /// # Arguments
    ///
    /// * `name` - The micro name
    /// * `value` - The micro value expression
    ///
    /// # Returns
    ///
    /// A formatted micro definition string.
    pub fn format_micro_val(&self, name: &str, value: &str) -> String {
        format!("micro {} = {}", name, value)
    }

    /// Indents each line of the given text by 4 spaces.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to indent
    ///
    /// # Returns
    ///
    /// The indented text.
    fn indent_lines(&self, text: &str) -> String {
        text.lines().map(|line| if line.trim().is_empty() { String::new() } else { format!("    {}", line) }).collect::<Vec<_>>().join("\n")
    }
}

impl Default for DejavuFormatter {
    fn default() -> Self {
        Self::new()
    }
}

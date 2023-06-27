use alloc::string::String;
use alloc::vec::Vec;

/// Whitespace processor
/// 
/// This struct is responsible for handling whitespace and blank lines during formatting.
pub struct WhitespaceProcessor {
    /// Whether to preserve blank lines
    pub preserve_blank_lines: bool,
    /// Maximum number of consecutive blank lines
    pub max_blank_lines: usize,
    /// Whether to trim trailing whitespace
    pub trim_trailing_whitespace: bool,
}

impl WhitespaceProcessor {
    /// Creates a new whitespace processor
    pub fn new() -> Self {
        Self {
            preserve_blank_lines: true,
            max_blank_lines: 2,
            trim_trailing_whitespace: true,
        }
    }

    /// Sets whether to preserve blank lines
    pub fn with_preserve_blank_lines(mut self, preserve: bool) -> Self {
        self.preserve_blank_lines = preserve;
        self
    }

    /// Sets the maximum number of consecutive blank lines
    pub fn with_max_blank_lines(mut self, max: usize) -> Self {
        self.max_blank_lines = max;
        self
    }

    /// Sets whether to trim trailing whitespace
    pub fn with_trim_trailing_whitespace(mut self, trim: bool) -> Self {
        self.trim_trailing_whitespace = trim;
        self
    }

    /// Processes whitespace in the formatted content
    pub fn process(&self, content: &str) -> String {
        let mut lines = content.lines().collect::<Vec<_>>();
        
        // Process each line
        let processed_lines = self.process_lines(&lines);
        
        // Join the lines back together
        processed_lines.join("\n")
    }

    /// Processes lines to handle whitespace and blank lines
    fn process_lines(&self, lines: &[&str]) -> Vec<String> {
        let mut processed_lines = Vec::new();
        let mut blank_line_count = 0;

        for line in lines {
            // Trim trailing whitespace if enabled
            let processed_line = if self.trim_trailing_whitespace {
                line.trim_end()
            } else {
                line
            };

            // Handle blank lines
            if processed_line.is_empty() {
                if self.preserve_blank_lines {
                    blank_line_count += 1;
                    if blank_line_count <= self.max_blank_lines {
                        processed_lines.push(String::new());
                    }
                }
            } else {
                // Reset blank line count
                blank_line_count = 0;
                processed_lines.push(processed_line.to_string());
            }
        }

        processed_lines
    }

    /// Normalizes indentation in the content
    pub fn normalize_indentation(&self, content: &str, indent: &str) -> String {
        let mut lines = content.lines().collect::<Vec<_>>();
        let mut processed_lines = Vec::new();

        for line in lines {
            // Remove existing indentation
            let trimmed_line = line.trim_start();
            
            // Add new indentation if the line is not empty
            if !trimmed_line.is_empty() {
                let mut new_line = String::new();
                new_line.push_str(indent);
                new_line.push_str(trimmed_line);
                processed_lines.push(new_line);
            } else {
                processed_lines.push(String::new());
            }
        }

        processed_lines.join("\n")
    }

    /// Adds consistent spacing around operators
    pub fn add_spacing_around_operators(&self, content: &str) -> String {
        // Simple implementation for demonstration
        // In a real implementation, this would be more sophisticated
        content
            .replace("=", " = ")
            .replace("+", " + ")
            .replace("-", " - ")
            .replace("*", " * ")
            .replace("/", " / ")
            .replace("%", " % ")
            .replace("==", " == ")
            .replace("!=", " != ")
            .replace(">=", " >= ")
            .replace("<=", " <= ")
            .replace(">", " > ")
            .replace("<", " < ")
            .replace("&&", " && ")
            .replace("||", " || ")
    }

    /// Removes unnecessary spacing
    pub fn remove_unnecessary_spacing(&self, content: &str) -> String {
        // Simple implementation for demonstration
        // In a real implementation, this would be more sophisticated
        content
            .replace("  ", " ")
            .replace(" (", "(")
            .replace(") ", ")")
            .replace(" [", "[")
            .replace("] ", "]")
            .replace(" {", "{")
            .replace("} ", "}")
    }
}

impl Default for WhitespaceProcessor {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SolidityFormatter;

impl SolidityFormatter {
    pub fn new() -> Self {
        Self
    }

    pub fn format(&self, source: &str) -> String {
        source.to_string()
    }
}

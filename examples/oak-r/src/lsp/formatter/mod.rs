#![doc = include_str!("readme.md")]
pub struct RFormatter {}

impl RFormatter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn format(&self, source: &str) -> String {
        source.to_string()
    }
}

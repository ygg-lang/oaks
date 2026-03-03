use std::collections::HashMap;
use alloc::string::String;

/// Default format state implementation
/// 
/// This struct provides a default implementation of the format state.
#[derive(Debug, Clone, Default)]
pub struct DefaultFormatState {
    /// Local configuration overrides
    pub local_config: HashMap<String, serde_json::Value>,
    /// Custom state values
    pub custom_state: HashMap<String, serde_json::Value>,
    /// Current indentation level
    pub indent_level: usize,
    /// Whether to force single line formatting
    pub force_single_line: bool,
    /// Whether to align elements
    pub align_elements: bool,
}

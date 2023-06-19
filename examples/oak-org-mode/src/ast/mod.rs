use std::{range::Range, string::String, vec::Vec};

/// Org-mode AST 根节点
#[derive(Debug, Clone)]
pub struct OrgModeRoot {
    pub items: Vec<OrgModeItem>,
    pub range: Range<usize>,
}

/// Org-mode 项目
#[derive(Debug, Clone)]
pub enum OrgModeItem {
    Heading { level: usize, title: String, content: Vec<OrgModeItem>, range: Range<usize> },
    Text { content: String, range: Range<usize> },
    Link { url: String, description: Option<String>, range: Range<usize> },
    Comment { content: String, range: Range<usize> },
}

use crate::language::SqlLanguage;

pub struct SqlFormatter<'config> {
    _config: &'config SqlLanguage,
}

impl<'config> SqlFormatter<'config> {
    pub fn new(config: &'config SqlLanguage) -> Self {
        Self { _config: config }
    }

    pub fn format(&self, _node: &oak_core::tree::RedNode<SqlLanguage>) -> String {
        // TODO: Implement SQL formatting
        String::new()
    }
}

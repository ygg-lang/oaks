use crate::SqlLanguage;

/// JSON 解析
pub struct SqlParser<'config> {
    pub(crate) config: &'config SqlLanguage,
}

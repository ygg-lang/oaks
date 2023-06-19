use crate::StylusLanguage;

/// TOML 解析
pub struct StylusParser<'config> {
    pub(crate) config: &'config StylusLanguage,
}

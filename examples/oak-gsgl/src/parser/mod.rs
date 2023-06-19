use crate::{language::GsglLanguage, syntax::GsglSyntaxKind};
use oak_core::{Arc, IncrementalCache, Parser, parser::ParseOutput, source::Source};

/// GSGL 解析器
pub struct GsglParser<'a> {
    _language: &'a GsglLanguage,
}

impl<'a> GsglParser<'a> {
    /// 创建新的 GSGL 解析器
    pub fn new(language: &'a GsglLanguage) -> Self {
        Self { _language: language }
    }
}

impl<'a> Parser<GsglLanguage> for GsglParser<'a> {
    fn parse_incremental(
        &self,
        _text: impl Source,
        _changed: usize,
        _cache: IncrementalCache<GsglLanguage>,
    ) -> ParseOutput<GsglLanguage> {
        // 这是一个基本的实现，只返回一个空的解析结果
        // 在实际项目中，这里应该包含完整的解析逻辑

        // 创建一个空的绿色节点作为占位符
        use oak_core::GreenBuilder;
        let mut builder: GreenBuilder<GsglLanguage> = GreenBuilder::new(0);
        let green_node = builder.finish(GsglSyntaxKind::Root);

        ParseOutput::<GsglLanguage> { result: Ok(green_node), diagnostics: Vec::new() }
    }
}

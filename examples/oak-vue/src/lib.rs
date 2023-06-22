pub mod kind;
pub mod lexer;
pub mod parser;

pub use kind::{VueLanguage, VueSyntaxKind};
pub use lexer::VueLexer;
pub use parser::VueParser;

use oak_core::{
    parser::{ParseOutput, ParseSession},
    source::SourceText,
};

/// 辅助解析函数，用于测试和简单解析。
/// 注意：返回的 GreenNode 的生命周期受限于传入的 cache 中的 arena。
pub fn parse<'a>(source: &'a SourceText, cache: &'a mut ParseSession<VueLanguage>) -> ParseOutput<'a, VueLanguage> {
    let language = VueLanguage::default();
    let lexer = VueLexer::new(&language);
    let parser = VueParser::new(&language);
    oak_core::parser::parse(&parser, &lexer, source, &[], cache)
}

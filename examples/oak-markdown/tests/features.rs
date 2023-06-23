use oak_markdown::{MarkdownLanguage, MarkdownLexer};

#[test]
fn test_feature_flags() {
    let mut config = MarkdownLanguage::default();

    // 测试数学公式
    let source = "$a^2 + b^2 = c^2$";

    // 开启时
    config.allow_math = true;
    let lexer = MarkdownLexer::new(&config);
    let tokens = lexer.lex_internal(&source);
    assert!(tokens.result.as_ref().unwrap().iter().any(|t| t.kind == oak_markdown::MarkdownTokenType::MathInline));

    // 关闭时
    config.allow_math = false;
    let lexer = MarkdownLexer::new(&config);
    let tokens = lexer.lex_internal(&source);
    assert!(!tokens.result.as_ref().unwrap().iter().any(|t| t.kind == oak_markdown::MarkdownTokenType::MathInline))
}

#[test]
fn test_front_matter() {
    let mut config = MarkdownLanguage::default();
    let source = "---\ntitle: test\n---\nContent";

    // 开启时
    config.allow_front_matter = true;
    let lexer = MarkdownLexer::new(&config);
    let tokens = lexer.lex_internal(&source);
    assert!(tokens.result.as_ref().unwrap().iter().any(|t| t.kind == oak_markdown::MarkdownTokenType::FrontMatter));

    // 关闭时
    config.allow_front_matter = false;
    let lexer = MarkdownLexer::new(&config);
    let tokens = lexer.lex_internal(&source);
    assert!(!tokens.result.as_ref().unwrap().iter().any(|t| t.kind == oak_markdown::MarkdownTokenType::FrontMatter))
}

#[test]
fn test_footnotes() {
    let mut config = MarkdownLanguage::default();
    let source = "Refer[^1]\n\n[^1]: Note";

    // 开启时
    config.allow_footnotes = true;
    let lexer = MarkdownLexer::new(&config);
    let tokens = lexer.lex_internal(&source);
    assert!(tokens.result.as_ref().unwrap().iter().any(|t| t.kind == oak_markdown::MarkdownTokenType::FootnoteReference));
    assert!(tokens.result.as_ref().unwrap().iter().any(|t| t.kind == oak_markdown::MarkdownTokenType::FootnoteDefinition));

    // 关闭时
    config.allow_footnotes = false;
    let lexer = MarkdownLexer::new(&config);
    let tokens = lexer.lex_internal(&source);
    assert!(!tokens.result.as_ref().unwrap().iter().any(|t| t.kind == oak_markdown::MarkdownTokenType::FootnoteReference))
}

#[test]
fn test_indented_code_blocks() {
    let mut config = MarkdownLanguage::default();
    let source = "    code block";

    // 开启时
    config.allow_indented_code_blocks = true;
    let lexer = MarkdownLexer::new(&config);
    let tokens = lexer.lex_internal(&source);
    assert!(tokens.result.as_ref().unwrap().iter().any(|t| t.kind == oak_markdown::MarkdownTokenType::CodeBlock));

    // 关闭时
    config.allow_indented_code_blocks = false;
    let lexer = MarkdownLexer::new(&config);
    let tokens = lexer.lex_internal(&source);
    assert!(!tokens.result.as_ref().unwrap().iter().any(|t| t.kind == oak_markdown::MarkdownTokenType::CodeBlock))
}

#[test]
fn test_headings() {
    let mut config = MarkdownLanguage::default();
    let source = "# Heading";

    // 开启时
    config.allow_headings = true;
    let lexer = MarkdownLexer::new(&config);
    let tokens = lexer.lex_internal(&source);
    assert!(tokens.result.as_ref().unwrap().iter().any(|t| matches!(t.kind, oak_markdown::MarkdownTokenType::Heading1)));

    // 关闭时
    config.allow_headings = false;
    let lexer = MarkdownLexer::new(&config);
    let tokens = lexer.lex_internal(&source);
    assert!(!tokens.result.as_ref().unwrap().iter().any(|t| matches!(t.kind, oak_markdown::MarkdownTokenType::Heading1)))
}

#[test]
fn test_blockquotes() {
    let mut config = MarkdownLanguage::default();
    let source = "> Quote";

    // 开启时
    config.allow_blockquotes = true;
    let lexer = MarkdownLexer::new(&config);
    let tokens = lexer.lex_internal(&source);
    assert!(tokens.result.as_ref().unwrap().iter().any(|t| t.kind == oak_markdown::MarkdownTokenType::BlockquoteMarker));

    // 关闭时
    config.allow_blockquotes = false;
    let lexer = MarkdownLexer::new(&config);
    let tokens = lexer.lex_internal(&source);
    assert!(!tokens.result.as_ref().unwrap().iter().any(|t| t.kind == oak_markdown::MarkdownTokenType::BlockquoteMarker))
}

#[test]
fn test_html_tags() {
    let mut config = MarkdownLanguage::default();
    let source = "<div>content</div>";

    // 开启时
    config.allow_html = true;
    let lexer = MarkdownLexer::new(&config);
    let tokens = lexer.lex_internal(&source);
    assert!(tokens.result.as_ref().unwrap().iter().any(|t| t.kind == oak_markdown::MarkdownTokenType::HtmlTag));

    // 关闭时
    config.allow_html = false;
    let lexer = MarkdownLexer::new(&config);
    let tokens = lexer.lex_internal(&source);
    assert!(!tokens.result.as_ref().unwrap().iter().any(|t| t.kind == oak_markdown::MarkdownTokenType::HtmlTag))
}

#[test]
fn test_xml_tags() {
    let mut config = MarkdownLanguage::default();
    let source = "<xml>content</xml>";

    // 开启时 (关闭 HTML 以确保识别为 XML)
    config.allow_html = false;
    config.allow_xml = true;
    let lexer = MarkdownLexer::new(&config);
    let tokens = lexer.lex_internal(&source);
    assert!(tokens.result.as_ref().unwrap().iter().any(|t| t.kind == oak_markdown::MarkdownTokenType::XmlTag));

    // 关闭时
    config.allow_xml = false;
    let lexer = MarkdownLexer::new(&config);
    let tokens = lexer.lex_internal(&source);
    assert!(!tokens.result.as_ref().unwrap().iter().any(|t| t.kind == oak_markdown::MarkdownTokenType::XmlTag))
}

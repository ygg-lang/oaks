use oak_core::SourceText;
use oak_markdown::{MarkdownLanguage, MarkdownLexer, MarkdownSyntaxKind};

#[test]
fn test_basic_text() {
    let language = MarkdownLanguage {};
    let lexer = MarkdownLexer::new(&language);
    let source = SourceText::new("Hello world");

    let result = lexer.lex(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());

    // 应该包含文本和EOF kind
    assert_eq!(tokens[0].kind, MarkdownSyntaxKind::Text);
    assert_eq!(tokens[tokens.len() - 1].kind, MarkdownSyntaxKind::Eof);
}

#[test]
fn test_headings() {
    let language = MarkdownLanguage {};
    let lexer = MarkdownLexer::new(&language);

    // 测试不同级别的标
    let test_cases = vec![
        ("# Heading 1", MarkdownSyntaxKind::Heading1),
        ("## Heading 2", MarkdownSyntaxKind::Heading2),
        ("### Heading 3", MarkdownSyntaxKind::Heading3),
        ("#### Heading 4", MarkdownSyntaxKind::Heading4),
        ("##### Heading 5", MarkdownSyntaxKind::Heading5),
        ("###### Heading 6", MarkdownSyntaxKind::Heading6),
    ];

    for (input, expected_kind) in test_cases {
        let source = SourceText::new(input);
        let result = lexer.lex(&source);
        assert!(result.result.is_ok());

        let tokens = result.result.unwrap();
        assert!(!tokens.is_empty());
        assert_eq!(tokens[0].kind, expected_kind);
    }
}

#[test]
fn test_emphasis_and_strong() {
    let language = MarkdownLanguage {};
    let lexer = MarkdownLexer::new(&language);

    let test_cases = vec![
        ("*italic*", MarkdownSyntaxKind::Emphasis),
        ("_italic_", MarkdownSyntaxKind::Emphasis),
        ("**bold**", MarkdownSyntaxKind::Strong),
        ("__bold__", MarkdownSyntaxKind::Strong),
    ];

    for (input, expected_kind) in test_cases {
        let source = SourceText::new(input);
        let result = lexer.lex(&source);
        assert!(result.result.is_ok());

        let tokens = result.result.unwrap();
        // 查找强调或加token
        let found = tokens.iter().any(|token| token.kind == expected_kind);
        assert!(found, "Expected {:?} token in input: {}", expected_kind, input);
    }
}

#[test]
fn test_inline_code() {
    let language = MarkdownLanguage {};
    let lexer = MarkdownLexer::new(&language);
    let source = SourceText::new("`code`");

    let result = lexer.lex(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let found = tokens.iter().any(|token| token.kind == MarkdownSyntaxKind::InlineCode);
    assert!(found, "Expected InlineCode token");
}

#[test]
fn test_code_block() {
    let language = MarkdownLanguage {};
    let lexer = MarkdownLexer::new(&language);
    let source = SourceText::new("```rust\nfn main() {}\n```");

    let result = lexer.lex(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let has_fence = tokens.iter().any(|token| token.kind == MarkdownSyntaxKind::CodeFence);
    let has_language = tokens.iter().any(|token| token.kind == MarkdownSyntaxKind::CodeLanguage);

    assert!(has_fence, "Expected CodeFence token");
    assert!(has_language, "Expected CodeLanguage token");
}

#[test]
fn test_links_and_images() {
    let language = MarkdownLanguage {};
    let lexer = MarkdownLexer::new(&language);

    let test_cases = vec![("[link](url)", MarkdownSyntaxKind::Link), ("![image](url)", MarkdownSyntaxKind::Image)];

    for (input, expected_kind) in test_cases {
        let source = SourceText::new(input);
        let result = lexer.lex(&source);
        assert!(result.result.is_ok());

        let tokens = result.result.unwrap();
        let found = tokens.iter().any(|token| token.kind == expected_kind);
        assert!(found, "Expected {:?} token in input: {}", expected_kind, input);
    }
}

#[test]
fn test_lists() {
    let language = MarkdownLanguage {};
    let lexer = MarkdownLexer::new(&language);

    let test_cases = vec!["- item", "* item", "+ item", "1. item", "42. item"];

    for input in test_cases {
        let source = SourceText::new(input);
        let result = lexer.lex(&source);
        assert!(result.result.is_ok());

        let tokens = result.result.unwrap();
        let found = tokens.iter().any(|token| token.kind == MarkdownSyntaxKind::ListMarker);
        assert!(found, "Expected ListMarker token in input: {}", input);
    }
}

#[test]
fn test_task_lists() {
    let language = MarkdownLanguage {};
    let lexer = MarkdownLexer::new(&language);

    let test_cases = vec!["- [ ] unchecked", "- [x] checked", "- [X] checked"];

    for input in test_cases {
        let source = SourceText::new(input);
        let result = lexer.lex(&source);
        assert!(result.result.is_ok());

        let tokens = result.result.unwrap();
        let has_list_marker = tokens.iter().any(|token| token.kind == MarkdownSyntaxKind::ListMarker);
        let has_task_marker = tokens.iter().any(|token| token.kind == MarkdownSyntaxKind::TaskMarker);

        assert!(has_list_marker, "Expected ListMarker token in input: {}", input);
        assert!(has_task_marker, "Expected TaskMarker token in input: {}", input);
    }
}

#[test]
fn test_blockquote() {
    let language = MarkdownLanguage {};
    let lexer = MarkdownLexer::new(&language);
    let source = SourceText::new("> This is a quote");

    let result = lexer.lex(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let found = tokens.iter().any(|token| token.kind == MarkdownSyntaxKind::BlockquoteMarker);
    assert!(found, "Expected BlockquoteMarker token");
}

#[test]
fn test_horizontal_rule() {
    let language = MarkdownLanguage {};
    let lexer = MarkdownLexer::new(&language);

    let test_cases = vec!["---", "***", "___", "- - -", "* * *", "_ _ _"];

    for input in test_cases {
        let source = SourceText::new(input);
        let result = lexer.lex(&source);
        assert!(result.result.is_ok());

        let tokens = result.result.unwrap();
        let found = tokens.iter().any(|token| token.kind == MarkdownSyntaxKind::HorizontalRule);
        assert!(found, "Expected HorizontalRule token in input: {}", input);
    }
}

#[test]
fn test_strikethrough() {
    let language = MarkdownLanguage {};
    let lexer = MarkdownLexer::new(&language);
    let source = SourceText::new("~~strikethrough~~");

    let result = lexer.lex(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let found = tokens.iter().any(|token| token.kind == MarkdownSyntaxKind::Strikethrough);
    assert!(found, "Expected Strikethrough token");
}

#[test]
fn test_whitespace_and_newlines() {
    let language = MarkdownLanguage {};
    let lexer = MarkdownLexer::new(&language);
    let source = SourceText::new("hello   \n  world");

    let result = lexer.lex(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let has_whitespace = tokens.iter().any(|token| token.kind == MarkdownSyntaxKind::Whitespace);
    let has_newline = tokens.iter().any(|token| token.kind == MarkdownSyntaxKind::Newline);

    assert!(has_whitespace, "Expected Whitespace token");
    assert!(has_newline, "Expected Newline token");
}

#[test]
fn test_special_characters() {
    let language = MarkdownLanguage {};
    let lexer = MarkdownLexer::new(&language);
    let source = SourceText::new("[]()<>*_`~#|-+.:!\\");

    let result = lexer.lex(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();

    // 检查是否包含各种特殊字token
    let expected_tokens = vec![
        MarkdownSyntaxKind::LeftBracket,
        MarkdownSyntaxKind::RightBracket,
        MarkdownSyntaxKind::LeftParen,
        MarkdownSyntaxKind::RightParen,
        MarkdownSyntaxKind::LeftAngle,
        MarkdownSyntaxKind::RightAngle,
        MarkdownSyntaxKind::Asterisk,
        MarkdownSyntaxKind::Underscore,
        MarkdownSyntaxKind::Backtick,
        MarkdownSyntaxKind::Tilde,
        MarkdownSyntaxKind::Hash,
        MarkdownSyntaxKind::Pipe,
        MarkdownSyntaxKind::Dash,
        MarkdownSyntaxKind::Plus,
        MarkdownSyntaxKind::Dot,
        MarkdownSyntaxKind::Colon,
        MarkdownSyntaxKind::Exclamation,
        MarkdownSyntaxKind::Escape,
    ];

    for expected_token in expected_tokens {
        let found = tokens.iter().any(|token| token.kind == expected_token);
        assert!(found, "Expected {:?} token", expected_token);
    }
}

#[test]
fn test_complex_markdown() {
    let language = MarkdownLanguage {};
    let lexer = MarkdownLexer::new(&language);
    let source = SourceText::new(
        r#"# Title

This is a **bold** and *italic* text with `inline code`.

## List

- Item 1
- [ ] Task item
- [x] Completed task

> This is a blockquote

```rust
fn main() {
    println!("Hello, world!");
}
```

---

[Link](https://example.com) and ![Image](image.png)
"#,
    );

    let result = lexer.lex(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());

    // 验证包含各种类型token
    let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();

    assert!(token_kinds.contains(&MarkdownSyntaxKind::Heading1));
    assert!(token_kinds.contains(&MarkdownSyntaxKind::Heading2));
    assert!(token_kinds.contains(&MarkdownSyntaxKind::Strong));
    assert!(token_kinds.contains(&MarkdownSyntaxKind::Emphasis));
    assert!(token_kinds.contains(&MarkdownSyntaxKind::InlineCode));
    assert!(token_kinds.contains(&MarkdownSyntaxKind::ListMarker));
    assert!(token_kinds.contains(&MarkdownSyntaxKind::TaskMarker));
    assert!(token_kinds.contains(&MarkdownSyntaxKind::BlockquoteMarker));
    assert!(token_kinds.contains(&MarkdownSyntaxKind::CodeFence));
    assert!(token_kinds.contains(&MarkdownSyntaxKind::CodeLanguage));
    assert!(token_kinds.contains(&MarkdownSyntaxKind::HorizontalRule));
    assert!(token_kinds.contains(&MarkdownSyntaxKind::Link));
    assert!(token_kinds.contains(&MarkdownSyntaxKind::Image));
    assert!(token_kinds.contains(&MarkdownSyntaxKind::Eof));
}

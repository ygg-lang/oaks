#[test]
fn ready() {
    println!("oak-vampire tests ready!");
}

#[cfg(test)]
mod tests {
    use oak_core::{ParseSession, SourceText, lexer::Lexer};
    use oak_vampire::{VampireLanguage, VampireLexer, kind::VampireSyntaxKind};

    #[test]
    fn test_vampire_lexer_basic_tokens() {
        let config = VampireLanguage::default();
        let lexer = VampireLexer::new(&config);
        let source = SourceText::new("cnf(test, axiom, p(X) | ~q(Y)).");

        let mut session = ParseSession::<VampireLanguage>::new(1024);
        let result = lexer.lex(&source, &[], &mut session);
        assert!(result.result.is_ok());

        let tokens = result.result.unwrap();
        assert!(!tokens.is_empty());

        // 打印所有 token 类型以便调试
        let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();
        println!("Token kinds: {:?}", token_kinds);

        // 检查是否包含基本的 token 类型
        assert!(token_kinds.iter().any(|&k| k == VampireSyntaxKind::CnfKw));
        assert!(token_kinds.iter().any(|&k| k == VampireSyntaxKind::LeftParen));
        assert!(token_kinds.iter().any(|&k| k == VampireSyntaxKind::RightParen));
    }

    #[test]
    fn test_vampire_lexer_identifiers() {
        let config = VampireLanguage::default();
        let lexer = VampireLexer::new(&config);
        let source = SourceText::new("p(X, Y) :- q(X), r(Y).");

        let mut session = ParseSession::<VampireLanguage>::new(1024);
        let result = lexer.lex(&source, &[], &mut session);
        assert!(result.result.is_ok());

        let tokens = result.result.unwrap();
        let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();
        println!("Identifier test token kinds: {:?}", token_kinds);
        assert!(token_kinds.iter().any(|&k| k == VampireSyntaxKind::Identifier));
    }

    #[test]
    fn test_vampire_lexer_operators() {
        let config = VampireLanguage::default();
        let lexer = VampireLexer::new(&config);
        let source = SourceText::new("p(X) | ~q(Y) & r(Z).");

        let mut session = ParseSession::<VampireLanguage>::new(1024);
        let result = lexer.lex(&source, &[], &mut session);
        assert!(result.result.is_ok());

        let tokens = result.result.unwrap();
        let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();
        println!("Operator test token kinds: {:?}", token_kinds);
        assert!(token_kinds.iter().any(|&k| k == VampireSyntaxKind::Pipe));
        assert!(token_kinds.iter().any(|&k| k == VampireSyntaxKind::Tilde));
        assert!(token_kinds.iter().any(|&k| k == VampireSyntaxKind::Ampersand));
    }

    #[test]
    fn test_vampire_lexer_comments() {
        let config = VampireLanguage::default();
        let lexer = VampireLexer::new(&config);
        let source = SourceText::new("% This is a comment\ncnf(test, axiom, p(X)).");

        let mut session = ParseSession::<VampireLanguage>::new(1024);
        let result = lexer.lex(&source, &[], &mut session);
        assert!(result.result.is_ok());

        let tokens = result.result.unwrap();
        let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();
        println!("Comment test token kinds: {:?}", token_kinds);
        assert!(token_kinds.iter().any(|&k| k == VampireSyntaxKind::LineComment));
        assert!(token_kinds.iter().any(|&k| k == VampireSyntaxKind::CnfKw));
    }

    #[test]
    fn test_vampire_lexer_whitespace() {
        let config = VampireLanguage::default();
        let lexer = VampireLexer::new(&config);
        let source = SourceText::new("  cnf  (  test  ,  axiom  ,  p  (  X  )  )  .  ");

        let mut session = ParseSession::<VampireLanguage>::new(1024);
        let result = lexer.lex(&source, &[], &mut session);
        assert!(result.result.is_ok());

        let tokens = result.result.unwrap();
        let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();
        println!("Whitespace test token kinds: {:?}", token_kinds);
        assert!(token_kinds.iter().any(|&k| k == VampireSyntaxKind::Whitespace));
        assert!(token_kinds.iter().any(|&k| k == VampireSyntaxKind::CnfKw));
    }
}

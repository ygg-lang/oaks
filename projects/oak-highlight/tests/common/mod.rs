use oak_core::{
    Arc, Range,
    errors::{OakDiagnostics, OakError},
    language::{ElementRole, ElementType, Language, TokenRole, TokenType, UniversalElementRole, UniversalTokenRole},
    lexer::{LexOutput, Lexer, LexerCache, Token, Tokens},
    parser::{ParseCache, ParseOutput, Parser},
    source::{Source, TextEdit},
    tree::{GreenLeaf, GreenNode, GreenTree},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MockTokenType {
    Keyword,
    Identifier,
    Whitespace,
    Eof,
    Punctuation,
    String,
}

impl TokenType for MockTokenType {
    type Role = UniversalTokenRole;
    fn role(&self) -> Self::Role {
        match self {
            MockTokenType::Keyword => UniversalTokenRole::Keyword,
            MockTokenType::Identifier => UniversalTokenRole::Name,
            MockTokenType::Whitespace => UniversalTokenRole::Whitespace,
            MockTokenType::Eof => UniversalTokenRole::Eof,
            MockTokenType::Punctuation => UniversalTokenRole::Punctuation,
            MockTokenType::String => UniversalTokenRole::Literal,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MockElementType {
    Root,
    Error,
}

impl ElementType for MockElementType {
    type Role = UniversalElementRole;
    fn role(&self) -> Self::Role {
        match self {
            MockElementType::Root => UniversalElementRole::Root,
            MockElementType::Error => UniversalElementRole::Error,
        }
    }
    fn is_root(&self) -> bool {
        matches!(self, MockElementType::Root)
    }
    fn is_error(&self) -> bool {
        matches!(self, MockElementType::Error)
    }
}

impl From<MockTokenType> for MockElementType {
    fn from(_: MockTokenType) -> Self {
        MockElementType::Error
    }
}

pub struct MockLanguage;
impl Language for MockLanguage {
    const NAME: &'static str = "mock";
    type TokenType = MockTokenType;
    type ElementType = MockElementType;
    type TypedRoot = MockElementType;
}

pub struct MockLexer;
impl Lexer<MockLanguage> for MockLexer {
    fn lex<'a, S: Source + ?Sized>(&self, text: &S, _edits: &[TextEdit], _cache: &'a mut impl LexerCache<MockLanguage>) -> LexOutput<MockLanguage> {
        let source = text.get_text_from(0);
        let mut tokens = Vec::new();
        let chars: Vec<char> = source.chars().collect();

        for (i, c) in chars.iter().enumerate() {
            let kind = match c {
                'k' => MockTokenType::Keyword,
                'i' => MockTokenType::Identifier,
                's' => MockTokenType::String,
                ' ' | '\n' | '\t' => MockTokenType::Whitespace,
                _ => MockTokenType::Punctuation,
            };

            tokens.push(Token { kind, span: Range { start: i, end: i + 1 } });
        }

        OakDiagnostics::new(Ok(Tokens::from(tokens)))
    }
}

pub struct MockParser;
impl Parser<MockLanguage> for MockParser {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<MockLanguage>) -> ParseOutput<'a, MockLanguage> {
        let lexer = MockLexer;
        let output = lexer.lex(text, edits, cache);
        let tokens = output.result.as_ref().unwrap();

        let mut leaves = Vec::new();
        for token in tokens.iter() {
            let leaf = GreenLeaf::new(token.kind, (token.span.end - token.span.start) as u32);
            leaves.push(GreenTree::Leaf(leaf));
        }

        let children = cache.arena().alloc_slice_copy(&leaves);
        let root = GreenNode::new(MockElementType::Root, children);
        let root_ref = cache.arena().alloc(root);

        OakDiagnostics::new(Ok(root_ref))
    }
}

use core::range::Range;
use oak_core::{
    ElementType, Language, TokenType, UniversalElementRole, UniversalTokenRole,
    lexer::{LexOutput, Token},
    memory::arena::SyntaxArena,
    parser::state::ParserState,
    source::TextEdit,
    tree::{GreenNode, GreenTree},
};
use triomphe::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MockToken {
    Item,
    Whitespace,
    End,
}

impl TokenType for MockToken {
    const END_OF_STREAM: Self = MockToken::End;
    type Role = UniversalTokenRole;
    fn role(&self) -> Self::Role {
        UniversalTokenRole::None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MockElement {
    Root,
    Item,
}

impl ElementType for MockElement {
    type Role = UniversalElementRole;
    fn role(&self) -> Self::Role {
        UniversalElementRole::None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct MockLanguage;
impl Language for MockLanguage {
    const NAME: &'static str = "mock";
    type TokenType = MockToken;
    type ElementType = MockElement;
    type TypedRoot = ();
}

// Simple manual lexer for the mock language
fn mock_lex(text: &str) -> LexOutput<MockLanguage> {
    let mut tokens = Vec::new();
    let mut pos = 0;
    let bytes = text.as_bytes();

    while pos < bytes.len() {
        if bytes[pos] == b' ' {
            tokens.push(Token { kind: MockToken::Whitespace, span: Range { start: pos, end: pos + 1 } });
            pos += 1;
        }
        else if text[pos..].starts_with("item") {
            let start = pos;
            pos += 4;
            while pos < bytes.len() && bytes[pos].is_ascii_alphanumeric() {
                pos += 1;
            }
            tokens.push(Token { kind: MockToken::Item, span: Range { start, end: pos } });
        }
        else {
            pos += 1; // skip unknown
        }
    }

    tokens.push(Token { kind: MockToken::End, span: Range { start: pos, end: pos } });

    LexOutput::<MockLanguage> { result: Ok(Arc::from(tokens)), diagnostics: Vec::new() }
}

impl From<MockToken> for MockElement {
    fn from(_: MockToken) -> Self {
        MockElement::Item
    }
}

// Simple parser for the mock language
fn parse<'a>(state: &mut ParserState<'a, MockLanguage>) -> &'a GreenNode<'a, MockLanguage> {
    let cp_root = state.checkpoint();

    while state.not_at_end() && state.peek_kind() != Some(MockToken::End) {
        if state.peek_kind() == Some(MockToken::Item) {
            // Try to reuse Item node
            if !state.try_reuse(MockElement::Item) {
                let cp_item = state.checkpoint();
                state.bump(); // eat the Item token
                state.finish_at(cp_item, MockElement::Item);
            }
        }
        else {
            state.bump(); // skip whitespace etc
        }
    }

    state.finish_at(cp_root, MockElement::Root)
}

#[test]
fn test_incremental_reuse() {
    let arena = SyntaxArena::default();
    let text1_str = "item1 item2 item3";
    let source1 = oak_core::source::SourceText::new(text1_str);
    let lex1 = mock_lex(text1_str);
    let mut state1 = ParserState::new(&arena, lex1, &source1, 1024);
    let tree1 = parse(&mut state1);

    // text2 has "itemX" instead of "item2"
    let text2_str = "item1 itemX item3";
    let source2 = oak_core::source::SourceText::new(text2_str);
    let lex2 = mock_lex(text2_str);

    // The edit is from "item2" (index 6-11) to "itemX" (index 6-11)
    let edits = vec![TextEdit { span: Range { start: 6, end: 11 }, text: "itemX".to_string() }];

    let arena2 = SyntaxArena::default();
    let mut state2 = ParserState::new(&arena2, lex2, &source2, 1024);
    state2.set_incremental(tree1, &edits);

    let tree2 = parse(&mut state2);

    // Check reuse
    // children: Item1, WS, ItemX, WS, Item3
    // Wait, the mock parser above doesn't handle WS as nodes.
    // It should have: Root -> [Item1, ItemX, Item3] if WS are skipped.

    let mut item_nodes = Vec::new();
    for child in tree2.children {
        if let GreenTree::Node(n) = child {
            if n.kind == MockElement::Item {
                item_nodes.push(n);
            }
        }
    }

    assert_eq!(item_nodes.len(), 3);

    for (i, node) in item_nodes.iter().enumerate() {
        println!("Node {}: kind={:?}, len={}", i, node.kind, node.text_len());
    }

    // Item1 should be reused
    assert_eq!(item_nodes[0].kind, MockElement::Item);
    assert_eq!(item_nodes[0].text_len(), 5);

    // ItemX should be new
    assert_eq!(item_nodes[1].kind, MockElement::Item);
    assert_eq!(item_nodes[1].text_len(), 5);

    // Item3 should be reused
    assert_eq!(item_nodes[2].kind, MockElement::Item);
    assert_eq!(item_nodes[2].text_len(), 5);
}

#![feature(new_range_api)]

use core::range::Range;
use oak_core::{
    ElementType, Language, TokenType, TreeSink, UniversalElementRole, UniversalTokenRole,
    memory::arena::SyntaxArena,
    tree::{GreenTree, ProvenancePart, TokenProvenance},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TestToken {
    Id,
    End,
}

impl TokenType for TestToken {
    const END_OF_STREAM: Self = TestToken::End;
    type Role = UniversalTokenRole;
    fn role(&self) -> Self::Role {
        UniversalTokenRole::None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TestElement {
    Root,
}

impl ElementType for TestElement {
    type Role = UniversalElementRole;
    fn role(&self) -> Self::Role {
        UniversalElementRole::None
    }
}

struct TestLanguage;

impl Language for TestLanguage {
    const NAME: &'static str = "test";
    type TokenType = TestToken;
    type ElementType = TestElement;
    type TypedRoot = ();
}

#[test]
fn test_provenance_persistence() {
    let arena = SyntaxArena::new(1);
    let mut sink = TreeSink::<TestLanguage>::new(&arena, 10);

    let checkpoint = sink.checkpoint();
    let range = Range { start: 10, end: 15 };
    let provenance = TokenProvenance { parts: vec![ProvenancePart::Synthesized("get_".to_string()), ProvenancePart::Source(range)] };

    sink.push_leaf_with_metadata(TestToken::Id, 9, provenance.clone());

    let node = sink.finish_node(checkpoint, TestElement::Root);

    if let GreenTree::Leaf(leaf) = node.children[0] {
        assert_eq!(leaf.kind, TestToken::Id);
        assert!(leaf.metadata.is_some());

        let stored_provenance = arena.get_metadata(leaf.metadata.unwrap()).unwrap();
        assert_eq!(stored_provenance, &provenance)
    }
    else {
        panic!("Expected leaf")
    }
}

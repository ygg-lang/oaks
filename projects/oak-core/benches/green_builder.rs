#![feature(test)]

extern crate test;

use oak_core::{
    Language, SyntaxArena, TokenType, TreeSink,
    language::{ElementType, UniversalElementRole, UniversalTokenRole},
};
use test::Bencher;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MockToken {
    End,
    Text,
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
    Child,
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

impl From<MockToken> for MockElement {
    fn from(_: MockToken) -> Self {
        MockElement::Child
    }
}

#[bench]
fn bench_finish_snapshot_32_tokens(b: &mut Bencher) {
    let arena = SyntaxArena::new(1);
    b.iter(|| {
        let mut sink = TreeSink::<MockLanguage>::new(&arena, 32);
        for _ in 0..32 {
            sink.push_leaf(MockToken::Text, 1);
        }
        test::black_box(sink.finish_node(0, MockElement::Root));
    });
}

#[bench]
fn bench_finish_at_nested_8x4_tokens(b: &mut Bencher) {
    let arena = SyntaxArena::new(1);
    b.iter(|| {
        let mut sink = TreeSink::<MockLanguage>::new(&arena, 64);
        let root_cp = 0;
        for _ in 0..8 {
            let cp = sink.checkpoint();
            for _ in 0..4 {
                sink.push_leaf(MockToken::Text, 1);
            }
            sink.finish_node(cp, MockElement::Child);
        }
        test::black_box(sink.finish_node(root_cp, MockElement::Root));
    });
}

#[bench]
fn bench_hash_consing_same_shape_repeated(b: &mut Bencher) {
    let arena = SyntaxArena::new(1);
    b.iter(|| {
        let mut sink = TreeSink::<MockLanguage>::new(&arena, 8);
        for _ in 0..4 {
            sink.push_leaf(MockToken::Text, 5);
        }
        test::black_box(sink.finish_node(0, MockElement::Child));
    });
}

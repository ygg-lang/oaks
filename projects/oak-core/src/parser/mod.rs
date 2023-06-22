#![doc = include_str!("readme.md")]

/// Pratt parser implementation for operator precedence parsing.
pub mod pratt;
/// Parser memory pool management.
pub mod session;
/// Internal parser state and checkpointing.
pub mod state;

pub use self::{
    pratt::{Associativity, OperatorInfo, Pratt, PrattParser, binary, postfix, unary},
    session::{ParseCache, ParseSession},
    state::ParserState,
};

pub use triomphe::Arc;

pub use crate::{
    Language, Lexer,
    errors::{OakDiagnostics, OakError},
    source::{Source, TextEdit},
    tree::GreenNode,
};

/// The output of a parsing operation, containing the result and diagnostics.
pub type ParseOutput<'a, L: Language> = OakDiagnostics<&'a GreenNode<'a, L>>;

/// Core parser trait that defines how to run the parser.
pub trait Parser<L: Language + Send + Sync + 'static> {
    /// The core parsing entry point.
    ///
    /// This method orchestrates the parsing process using the provided cache.
    /// It should handle incremental reuse automatically if the cache contains a previous tree.
    ///
    /// # Arguments
    /// * `text` - The source text
    /// * `edits` - Edits applied to the source since the last parse
    /// * `cache` - The cache for resources and incremental reuse
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<L>) -> ParseOutput<'a, L>;
}

/// Standalone parsing function that coordinates lexing and parsing.
///
/// This is a convenience function for performing a complete parse (lexing + parsing)
/// in one call.
pub fn parse<'a, L, P, Lex, S>(parser: &P, _lexer: &Lex, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<L>) -> ParseOutput<'a, L>
where
    L: Language + Send + Sync + 'static,
    P: Parser<L>,
    Lex: Lexer<L>,
    S: Source + ?Sized,
{
    parser.parse(text, edits, cache)
}

/// Standalone parsing function that performs a complete parse without incremental reuse.
///
/// This is a convenience function for parsing a source from scratch.
pub fn parse_one_pass<'a, L, P, S>(parser: &P, text: &'a S, cache: &'a mut impl ParseCache<L>) -> ParseOutput<'a, L>
where
    L: Language + Send + Sync + 'static,
    P: Parser<L>,
    S: Source + ?Sized,
{
    parser.parse(text, &[], cache)
}

/// Helper for implementing `Parser::parse` with automatic lexing.
///
/// This function handles the boilerplate of preparing the cache, ensuring lexing is performed,
/// setting up the parser state, and committing the result.
pub fn parse_with_lexer<'a, L, S, Lex>(lexer: &Lex, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<L>, run: impl FnOnce(&mut ParserState<'a, L, S>) -> Result<&'a GreenNode<'a, L>, OakError>) -> ParseOutput<'a, L>
where
    L: Language + Send + Sync + 'static,
    S: Source + ?Sized,
    Lex: Lexer<L>,
{
    // 1. Prepare for new generation
    cache.prepare_generation();

    // 2. Get Lexing Result (Auto-lex if missing)
    let lex_out = match cache.lex_output() {
        Some(out) => out.clone(),
        None => {
            let out = lexer.lex(text, edits, cache);
            cache.set_lex_output(out.clone());
            out
        }
    };

    let capacity_hint = cache.old_tree().map(|old| old.children.len().max(1024)).unwrap_or(1024);

    // 3. Initialize Parser State
    // Safety: We transmute the arena and old tree to 'a to satisfy the borrow checker.
    // The ParseCache guarantees that the arena and old tree live long enough.
    let arena: &'a crate::memory::arena::SyntaxArena = unsafe { std::mem::transmute(cache.arena()) };
    let mut st = ParserState::new(arena, lex_out, text, capacity_hint);

    if let Some(old) = cache.old_tree() {
        let old: &'a GreenNode<'a, L> = unsafe { std::mem::transmute(old) };
        st.set_incremental(old, edits);
    }

    // 4. Run Parser Logic
    let result = run(&mut st);
    let output = st.finish(result);

    // 5. Commit Generation
    if let Ok(root) = output.result {
        cache.commit_generation(root);
    }

    output
}

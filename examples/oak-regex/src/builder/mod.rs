use crate::{
    RegexLanguage, RegexParser, RegexSyntaxKind,
    ast::{Alternative, Assertion, AssertionKind, CharacterClass, CharacterRange, Group, GroupKind, Literal, Pattern, PatternElement, RegexRoot, Special, SpecialKind},
    lexer::RegexLexer,
};
use core::range::Range;
use oak_core::{
    Builder, GreenNode, Lexer, OakDiagnostics, OakError, Parser, RedNode, RedTree, SourceText,
    builder::{BuildOutput, BuilderCache},
    parser::ParseSession,
    source::{Source, TextEdit},
};

impl<'config> Builder<RegexLanguage> for RegexParser<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<RegexLanguage>) -> BuildOutput<RegexLanguage> {
        let parser = RegexParser::new(self.config);
        let lexer = RegexLexer::new(self.config);

        let mut session = ParseSession::<RegexLanguage>::default();
        lexer.lex(source, edits, &mut session);
        let parse_result = parser.parse(source, edits, &mut session);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()));
                match self.build_root(&green_tree, &source_text) {
                    Ok(ast_root) => OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics },
                    Err(build_error) => {
                        let mut diagnostics = parse_result.diagnostics;
                        diagnostics.push(build_error.clone());
                        OakDiagnostics { result: Err(build_error), diagnostics }
                    }
                }
            }
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}

impl<'config> RegexParser<'config> {
    pub(crate) fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, RegexLanguage>, source: &SourceText) -> Result<RegexRoot, OakError> {
        let red_root = RedNode::new(green_tree, 0);
        let mut patterns = Vec::new();

        for child in red_root.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    RegexSyntaxKind::RegexPattern => {
                        let pattern = self.build_pattern(n, source)?;
                        patterns.push(pattern);
                    }
                    _ => {
                        return Err(OakError::syntax_error("Unexpected item in root".to_string(), n.span().start, None));
                    }
                },
                RedTree::Leaf(t) => {
                    if t.kind == RegexSyntaxKind::Whitespace || t.kind == RegexSyntaxKind::Comment {
                        continue;
                    }
                    return Err(OakError::syntax_error("Unexpected token in root".to_string(), t.span.start, None));
                }
            }
        }

        Ok(RegexRoot { alternatives: patterns })
    }

    /// Build a pattern from a node
    pub(crate) fn build_pattern(&self, node: RedNode<RegexLanguage>, source: &SourceText) -> Result<Pattern, OakError> {
        let span = node.span();
        let mut elements = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Node(n) => {
                    let element = self.build_pattern_element(n, source)?;
                    elements.push(element);
                }
                RedTree::Leaf(t) => {
                    if t.kind == RegexSyntaxKind::Pipe {
                        continue;
                    }
                    if t.kind == RegexSyntaxKind::Whitespace || t.kind == RegexSyntaxKind::Comment {
                        continue;
                    }

                    if t.kind == RegexSyntaxKind::Character {
                        let value = text(source, t.span.clone());
                        let lit = Literal { value, span: t.span.clone() };
                        elements.push(PatternElement::Literal(lit));
                    }
                    else {
                        return Err(OakError::syntax_error("Unexpected token in pattern".to_string(), t.span.start, None));
                    }
                }
            }
        }

        Ok(Pattern { alternatives: vec![Alternative { elements, span: span.clone() }], span })
    }

    /// Build a pattern element from a node
    pub(crate) fn build_pattern_element(&self, node: RedNode<RegexLanguage>, source: &SourceText) -> Result<PatternElement, OakError> {
        let span = node.span();

        match node.green.kind {
            RegexSyntaxKind::Character => {
                let value = text(source, node.span());
                Ok(PatternElement::Literal(Literal { value, span }))
            }
            RegexSyntaxKind::Dot => Ok(PatternElement::Special(Special { kind: SpecialKind::Any, span })),
            RegexSyntaxKind::LBrack => self.build_character_class(node, source),
            RegexSyntaxKind::LParen => self.build_group(node, source),
            RegexSyntaxKind::Question | RegexSyntaxKind::Star | RegexSyntaxKind::Plus | RegexSyntaxKind::LBrace => Err(OakError::syntax_error("Quantifier without preceding element".to_string(), span.start, None)),
            RegexSyntaxKind::Backslash => self.build_escape_sequence(node, source),
            RegexSyntaxKind::Hat | RegexSyntaxKind::Dollar => self.build_assertion(node, source),
            _ => Err(OakError::syntax_error(format!("Unexpected pattern element: {:?}", node.green.kind), span.start, None)),
        }
    }

    /// Build a character class from a node
    fn build_character_class(&self, node: RedNode<RegexLanguage>, source: &SourceText) -> Result<PatternElement, OakError> {
        let span = node.span();
        let mut ranges: Vec<CharacterRange> = Vec::new();
        let mut negated = false;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    match t.kind {
                        RegexSyntaxKind::Hat => {
                            negated = true;
                        }
                        RegexSyntaxKind::Character => {
                            let value = text(source, t.span.clone());
                            let ch = value.chars().next().unwrap();
                            ranges.push(CharacterRange { start: ch, end: None, span: t.span.clone() });
                        }
                        RegexSyntaxKind::Dash => {
                            // Range separator, ignored in this minimal implementation
                        }
                        _ => {
                            // Skip other tokens
                        }
                    }
                }
                RedTree::Node(n) => {
                    return Err(OakError::syntax_error("Unexpected node in character class".to_string(), n.span().start, None));
                }
            }
        }

        Ok(PatternElement::CharacterClass(CharacterClass { negated, ranges, span }))
    }

    /// Build a group from a node
    fn build_group(&self, node: RedNode<RegexLanguage>, _source: &SourceText) -> Result<PatternElement, OakError> {
        let span = node.span();
        Ok(PatternElement::Group(Group { kind: GroupKind::NonCapturing, element: Box::new(PatternElement::Literal(Literal { value: String::new(), span: span.clone() })), span }))
    }

    /// Build an escape sequence from a node
    fn build_escape_sequence(&self, node: RedNode<RegexLanguage>, source: &SourceText) -> Result<PatternElement, OakError> {
        let span = node.span();
        let mut escaped_char = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == RegexSyntaxKind::Character {
                        let value = text(source, t.span.clone());
                        escaped_char = value.chars().next();
                    }
                }
                _ => {
                    return Err(OakError::syntax_error("Unexpected node in escape sequence".to_string(), child.span().start, None));
                }
            }
        }

        if let Some(c) = escaped_char { Ok(PatternElement::Special(Special { kind: SpecialKind::Control(c), span })) } else { Err(OakError::syntax_error("Invalid escape sequence".to_string(), span.start, None)) }
    }

    /// Build an assertion from a node
    fn build_assertion(&self, node: RedNode<RegexLanguage>, _source: &SourceText) -> Result<PatternElement, OakError> {
        let span = node.span();

        match node.green.kind {
            RegexSyntaxKind::Hat => Ok(PatternElement::Assertion(Assertion { kind: AssertionKind::Start, span })),
            RegexSyntaxKind::Dollar => Ok(PatternElement::Assertion(Assertion { kind: AssertionKind::End, span })),
            _ => Err(OakError::syntax_error(format!("Unexpected assertion: {:?}", node.green.kind), span.start, None)),
        }
    }
}

/// Helper function to extract text from source
fn text(source: &SourceText, span: Range<usize>) -> String {
    source.get_text_in(span.into()).to_string()
}

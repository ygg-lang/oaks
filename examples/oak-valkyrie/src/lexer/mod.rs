use crate::{
    kind::{ValkyrieSyntaxKind, ValkyrieToken},
    language::ValkyrieLanguage,
};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, ValkyrieLanguage>;

static VK_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static VK_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["//"] });
static VK_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });
static VK_CHAR: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: Some('\\') });

#[derive(Clone)]
pub struct ValkyrieLexer<'config> {
    config: &'config ValkyrieLanguage,
}

impl<'config> Lexer<ValkyrieLanguage> for ValkyrieLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<ValkyrieLanguage>,
    ) -> LexOutput<ValkyrieLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> ValkyrieLexer<'config> {
    pub fn new(config: &'config ValkyrieLanguage) -> Self {
        Self { config }
    }

    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.skip_comment(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_char_literal(state) {
                continue;
            }

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_operators(state) {
                continue;
            }

            if self.lex_single_char_tokens(state) {
                continue;
            }

            // If no rule matches, advance by one character and mark as error
            state.advance(1);
            state.add_token(ValkyrieSyntaxKind::Error, safe_point, state.get_position());
        }

        // Add EOF token
        let eof_pos = state.get_position();
        state.add_token(ValkyrieSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match VK_WHITESPACE.scan(state.rest(), state.get_position(), ValkyrieSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                return true;
            }
            None => {}
        }
        false
    }

    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        match VK_COMMENT.scan(state.rest(), state.get_position(), ValkyrieSyntaxKind::LineComment) {
            Some(token) => {
                state.advance_with(token);
                return true;
            }
            None => {}
        }
        false
    }

    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        match VK_STRING.scan(state.rest(), state.get_position(), ValkyrieSyntaxKind::StringLiteral) {
            Some(token) => {
                state.advance_with(token);
                return true;
            }
            None => {}
        }
        false
    }

    fn lex_char_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        match VK_CHAR.scan(state.rest(), state.get_position(), ValkyrieSyntaxKind::CharLiteral) {
            Some(token) => {
                state.advance_with(token);
                return true;
            }
            None => {}
        }
        false
    }

    fn lex_number_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.current() {
            if ch.is_ascii_digit() {
                state.advance(ch.len_utf8());

                // 继续读取数字
                while let Some(ch) = state.current() {
                    if ch.is_ascii_digit() || ch == '.' || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(ValkyrieSyntaxKind::IntegerLiteral, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.current() {
            if ch.is_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                // 继续读取标识符字符
                while let Some(ch) = state.current() {
                    if ch.is_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_in((start..state.get_position()).into());
                let token_kind = match text {
                    "mod" => ValkyrieSyntaxKind::ModKw,
                    "fn" => ValkyrieSyntaxKind::FnKw,
                    "struct" => ValkyrieSyntaxKind::StructKw,
                    "enum" => ValkyrieSyntaxKind::EnumKw,
                    "trait" => ValkyrieSyntaxKind::TraitKw,
                    "impl" => ValkyrieSyntaxKind::ImplKw,
                    "type" => ValkyrieSyntaxKind::TypeKw,
                    "let" => ValkyrieSyntaxKind::LetKw,
                    "mut" => ValkyrieSyntaxKind::MutKw,
                    "const" => ValkyrieSyntaxKind::ConstKw,
                    "static" => ValkyrieSyntaxKind::StaticKw,
                    "if" => ValkyrieSyntaxKind::IfKw,
                    "else" => ValkyrieSyntaxKind::ElseKw,
                    "match" => ValkyrieSyntaxKind::MatchKw,
                    "for" => ValkyrieSyntaxKind::ForKw,
                    "while" => ValkyrieSyntaxKind::WhileKw,
                    "loop" => ValkyrieSyntaxKind::LoopKw,
                    "break" => ValkyrieSyntaxKind::BreakKw,
                    "continue" => ValkyrieSyntaxKind::ContinueKw,
                    "return" => ValkyrieSyntaxKind::ReturnKw,
                    "pub" => ValkyrieSyntaxKind::PubKw,
                    "use" => ValkyrieSyntaxKind::UseKw,
                    "as" => ValkyrieSyntaxKind::AsKw,
                    "in" => ValkyrieSyntaxKind::InKw,
                    "where" => ValkyrieSyntaxKind::WhereKw,
                    "self" => ValkyrieSyntaxKind::SelfKw,
                    "super" => ValkyrieSyntaxKind::SuperKw,
                    "crate" => ValkyrieSyntaxKind::CrateKw,
                    "unsafe" => ValkyrieSyntaxKind::UnsafeKw,
                    "extern" => ValkyrieSyntaxKind::ExternKw,
                    "ref" => ValkyrieSyntaxKind::RefKw,
                    "move" => ValkyrieSyntaxKind::MoveKw,
                    "box" => ValkyrieSyntaxKind::BoxKw,
                    "async" => ValkyrieSyntaxKind::AsyncKw,
                    "await" => ValkyrieSyntaxKind::AwaitKw,
                    "try" => ValkyrieSyntaxKind::TryKw,
                    "catch" => ValkyrieSyntaxKind::CatchKw,
                    "finally" => ValkyrieSyntaxKind::FinallyKw,
                    "yield" => ValkyrieSyntaxKind::YieldKw,
                    "macro" => ValkyrieSyntaxKind::MacroKw,
                    "dyn" => ValkyrieSyntaxKind::DynKw,
                    "bool" => ValkyrieSyntaxKind::BoolKw,
                    "char" => ValkyrieSyntaxKind::CharKw,
                    "str" => ValkyrieSyntaxKind::StrKw,
                    "i8" => ValkyrieSyntaxKind::I8Kw,
                    "i16" => ValkyrieSyntaxKind::I16Kw,
                    "i32" => ValkyrieSyntaxKind::I32Kw,
                    "i64" => ValkyrieSyntaxKind::I64Kw,
                    "i128" => ValkyrieSyntaxKind::I128Kw,
                    "isize" => ValkyrieSyntaxKind::IsizeKw,
                    "u8" => ValkyrieSyntaxKind::U8Kw,
                    "u16" => ValkyrieSyntaxKind::U16Kw,
                    "u32" => ValkyrieSyntaxKind::U32Kw,
                    "u64" => ValkyrieSyntaxKind::U64Kw,
                    "u128" => ValkyrieSyntaxKind::U128Kw,
                    "usize" => ValkyrieSyntaxKind::UsizeKw,
                    "f32" => ValkyrieSyntaxKind::F32Kw,
                    "f64" => ValkyrieSyntaxKind::F64Kw,
                    "true" | "false" => ValkyrieSyntaxKind::BoolLiteral,
                    _ => ValkyrieSyntaxKind::Identifier,
                };

                state.add_token(token_kind, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_operators<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.current() {
            let kind = match ch {
                '+' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::PlusEq
                    }
                    else if let Some('+') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::PlusPlus
                    }
                    else {
                        ValkyrieSyntaxKind::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::MinusEq
                    }
                    else if let Some('-') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::MinusMinus
                    }
                    else if let Some('>') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::Arrow
                    }
                    else {
                        ValkyrieSyntaxKind::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::StarEq
                    }
                    else {
                        ValkyrieSyntaxKind::Star
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::SlashEq
                    }
                    else {
                        ValkyrieSyntaxKind::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::PercentEq
                    }
                    else {
                        ValkyrieSyntaxKind::Percent
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::EqEq
                    }
                    else {
                        ValkyrieSyntaxKind::Eq
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::NotEq
                    }
                    else {
                        ValkyrieSyntaxKind::Bang
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::LessEq
                    }
                    else if let Some('<') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::LeftShift
                    }
                    else {
                        ValkyrieSyntaxKind::LessThan
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::GreaterEq
                    }
                    else if let Some('>') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::RightShift
                    }
                    else {
                        ValkyrieSyntaxKind::GreaterThan
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::AndAnd
                    }
                    else {
                        ValkyrieSyntaxKind::Ampersand
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::OrOr
                    }
                    else {
                        ValkyrieSyntaxKind::Pipe
                    }
                }
                '^' => {
                    state.advance(1);
                    ValkyrieSyntaxKind::Caret
                }
                '~' => {
                    state.advance(1);
                    ValkyrieSyntaxKind::Tilde
                }
                '.' => {
                    state.advance(1);
                    ValkyrieSyntaxKind::Dot
                }
                ':' => {
                    state.advance(1);
                    ValkyrieSyntaxKind::Colon
                }
                _ => return false,
            };
            state.add_token(kind, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_single_char_tokens<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.current() {
            let kind = match ch {
                '(' => ValkyrieSyntaxKind::LeftParen,
                ')' => ValkyrieSyntaxKind::RightParen,
                '{' => ValkyrieSyntaxKind::LeftBrace,
                '}' => ValkyrieSyntaxKind::RightBrace,
                '[' => ValkyrieSyntaxKind::LeftBracket,
                ']' => ValkyrieSyntaxKind::RightBracket,
                ',' => ValkyrieSyntaxKind::Comma,
                ';' => ValkyrieSyntaxKind::Semicolon,
                '@' => ValkyrieSyntaxKind::At,
                '#' => ValkyrieSyntaxKind::Hash,
                '$' => ValkyrieSyntaxKind::Dollar,
                '?' => ValkyrieSyntaxKind::Question,
                _ => return false,
            };
            state.advance(ch.len_utf8());
            state.add_token(kind, start, state.get_position());
            return true;
        }
        false
    }
}

use crate::{kind::WitSyntaxKind, language::WitLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, WitLanguage>;

static WIT_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static WIT_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["//"] });
static WIT_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });

#[derive(Clone)]
pub struct WitLexer<'config> {
    config: &'config WitLanguage,
}

impl<'config> Lexer<WitLanguage> for WitLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<WitLanguage>,
    ) -> LexOutput<WitLanguage> {
        let mut state = LexerState::new(source);
        let _ = self.run(&mut state);
        state.finish(Ok(()))
    }
}

impl<'config> WitLexer<'config> {
    pub fn new(config: &'config WitLanguage) -> Self {
        Self { config }
    }

    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
        while state.not_at_end() {
            if self.skip_whitespace(state) {
                continue;
            }

            if self.skip_comment(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_punctuation(state) {
                continue;
            }

            if self.lex_text(state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(WitSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(WitSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match WIT_WHITESPACE.scan(state.rest(), state.get_position(), WitSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                return true;
            }
            None => {}
        }
        false
    }

    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        match WIT_COMMENT.scan(state.rest(), state.get_position(), WitSyntaxKind::Comment) {
            Some(token) => {
                state.advance_with(token);
                return true;
            }
            None => {}
        }
        false
    }

    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        match WIT_STRING.scan(state.rest(), start, WitSyntaxKind::StringLiteral) {
            Some(token) => {
                state.advance_with(token);
                return true;
            }
            None => {}
        }
        false
    }

    fn lex_number_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();
        let mut has_digits = false;

        // 处理数字
        while let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);
                has_digits = true;
            }
            else {
                break;
            }
        }

        if has_digits {
            state.add_token(WitSyntaxKind::IntegerLiteral, start_pos, state.get_position());
            return true;
        }

        false
    }

    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' || ch == '%' {
                state.advance(ch.len_utf8());

                // 继续读取标识符字符
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_in((start_pos..state.get_position()).into());
                let token_kind = match text {
                    // WIT 关键字
                    "world" => WitSyntaxKind::WorldKw,
                    "interface" => WitSyntaxKind::InterfaceKw,
                    "package" => WitSyntaxKind::PackageKw,
                    "component" => WitSyntaxKind::ComponentKw,
                    "instance" => WitSyntaxKind::InstanceKw,
                    "module" => WitSyntaxKind::ModuleKw,
                    "core" => WitSyntaxKind::CoreKw,
                    "func" => WitSyntaxKind::FuncKw,
                    "type" => WitSyntaxKind::TypeKw,
                    "record" => WitSyntaxKind::RecordKw,
                    "variant" => WitSyntaxKind::VariantKw,
                    "enum" => WitSyntaxKind::EnumKw,
                    "flags" => WitSyntaxKind::FlagsKw,
                    "union" => WitSyntaxKind::UnionKw,
                    "tuple" => WitSyntaxKind::TupleKw,
                    "list" => WitSyntaxKind::ListKw,
                    "option" => WitSyntaxKind::OptionKw,
                    "result" => WitSyntaxKind::ResultKw,
                    "static" => WitSyntaxKind::StaticKw,
                    "constructor" => WitSyntaxKind::ConstructorKw,
                    "method" => WitSyntaxKind::MethodKw,
                    "import" => WitSyntaxKind::ImportKw,
                    "export" => WitSyntaxKind::ExportKw,
                    "use" => WitSyntaxKind::UseKw,
                    "include" => WitSyntaxKind::IncludeKw,
                    "with" => WitSyntaxKind::WithKw,
                    "resource" => WitSyntaxKind::ResourceKw,
                    "bool" => WitSyntaxKind::BoolKw,
                    "u8" => WitSyntaxKind::U8Kw,
                    "u16" => WitSyntaxKind::U16Kw,
                    "u32" => WitSyntaxKind::U32Kw,
                    "u64" => WitSyntaxKind::U64Kw,
                    "s8" => WitSyntaxKind::S8Kw,
                    "s16" => WitSyntaxKind::S16Kw,
                    "s32" => WitSyntaxKind::S32Kw,
                    "s64" => WitSyntaxKind::S64Kw,
                    "f32" => WitSyntaxKind::F32Kw,
                    "f64" => WitSyntaxKind::F64Kw,
                    "char" => WitSyntaxKind::CharKw,
                    "string" => WitSyntaxKind::StringKw,
                    _ => WitSyntaxKind::Identifier,
                };

                state.add_token(token_kind, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_punctuation<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => {
                    state.advance(1);
                    WitSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    WitSyntaxKind::RightParen
                }
                '{' => {
                    state.advance(1);
                    WitSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    WitSyntaxKind::RightBrace
                }
                '[' => {
                    state.advance(1);
                    WitSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    WitSyntaxKind::RightBracket
                }
                '<' => {
                    state.advance(1);
                    WitSyntaxKind::Lt
                }
                '>' => {
                    state.advance(1);
                    WitSyntaxKind::Gt
                }
                ',' => {
                    state.advance(1);
                    WitSyntaxKind::Comma
                }
                ';' => {
                    state.advance(1);
                    WitSyntaxKind::Semicolon
                }
                ':' => {
                    state.advance(1);
                    WitSyntaxKind::Colon
                }
                '=' => {
                    state.advance(1);
                    WitSyntaxKind::Assign
                }
                '.' => {
                    state.advance(1);
                    WitSyntaxKind::Dot
                }
                '*' => {
                    state.advance(1);
                    WitSyntaxKind::Star
                }
                '/' => {
                    state.advance(1);
                    WitSyntaxKind::Slash
                }
                '@' => {
                    state.advance(1);
                    WitSyntaxKind::At
                }
                '-' => {
                    state.advance(1);
                    if state.peek() == Some('>') {
                        state.advance(1);
                        WitSyntaxKind::Arrow
                    }
                    else {
                        WitSyntaxKind::Minus
                    }
                }
                _ => return false,
            };

            state.add_token(token_kind, start_pos, state.get_position());
            return true;
        }

        false
    }

    fn lex_text<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            state.advance(ch.len_utf8());
            state.add_token(WitSyntaxKind::Error, start_pos, state.get_position());
            return true;
        }

        false
    }
}

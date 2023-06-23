#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::WitLanguage, lexer::token_type::WitTokenType};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError, TextEdit,
    lexer::{CommentConfig, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, WitLanguage>;

static WIT_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static WIT_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "//", block_start: "/*", block_end: "*/", nested_blocks: true });
static WIT_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });

#[derive(Clone)]
pub struct WitLexer<'config> {
    config: &'config WitLanguage,
}

impl<'config> Lexer<WitLanguage> for WitLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<WitLanguage>) -> LexOutput<WitLanguage> {
        let mut state: State<'_, S> = LexerState::new(source);
        let result = self.run(&mut state);
        state.finish_with_cache(result, cache)
    }
}

impl<'config> WitLexer<'config> {
    pub fn new(config: &'config WitLanguage) -> Self {
        Self { config }
    }

    fn run<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> Result<(), OakError> {
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
                state.add_token(WitTokenType::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(WitTokenType::Eof, eof_pos, eof_pos);
        Ok(())
    }

    fn skip_whitespace<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        WIT_WHITESPACE.scan(state, WitTokenType::Whitespace)
    }

    fn skip_comment<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        WIT_COMMENT.scan(state, WitTokenType::Comment, WitTokenType::Comment)
    }

    fn lex_string_literal<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        WIT_STRING.scan(state, WitTokenType::StringLiteral)
    }

    fn lex_number_literal<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
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
            state.add_token(WitTokenType::IntegerLiteral, start_pos, state.get_position());
            return true;
        }

        false
    }

    fn lex_identifier_or_keyword<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
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

                let text = state.get_text_from(start_pos);
                let token_kind = match text.as_ref() {
                    // WIT 关键字
                    "world" => WitTokenType::WorldKw,
                    "interface" => WitTokenType::InterfaceKw,
                    "package" => WitTokenType::PackageKw,
                    "component" => WitTokenType::ComponentKw,
                    "instance" => WitTokenType::InstanceKw,
                    "module" => WitTokenType::ModuleKw,
                    "core" => WitTokenType::CoreKw,
                    "func" => WitTokenType::FuncKw,
                    "type" => WitTokenType::TypeKw,
                    "record" => WitTokenType::RecordKw,
                    "variant" => WitTokenType::VariantKw,
                    "enum" => WitTokenType::EnumKw,
                    "flags" => WitTokenType::FlagsKw,
                    "union" => WitTokenType::UnionKw,
                    "tuple" => WitTokenType::TupleKw,
                    "list" => WitTokenType::ListKw,
                    "option" => WitTokenType::OptionKw,
                    "result" => WitTokenType::ResultKw,
                    "static" => WitTokenType::StaticKw,
                    "constructor" => WitTokenType::ConstructorKw,
                    "method" => WitTokenType::MethodKw,
                    "import" => WitTokenType::ImportKw,
                    "export" => WitTokenType::ExportKw,
                    "use" => WitTokenType::UseKw,
                    "include" => WitTokenType::IncludeKw,
                    "with" => WitTokenType::WithKw,
                    "resource" => WitTokenType::ResourceKw,
                    "bool" => WitTokenType::BoolKw,
                    "u8" => WitTokenType::U8Kw,
                    "u16" => WitTokenType::U16Kw,
                    "u32" => WitTokenType::U32Kw,
                    "u64" => WitTokenType::U64Kw,
                    "s8" => WitTokenType::S8Kw,
                    "s16" => WitTokenType::S16Kw,
                    "s32" => WitTokenType::S32Kw,
                    "s64" => WitTokenType::S64Kw,
                    "f32" => WitTokenType::F32Kw,
                    "f64" => WitTokenType::F64Kw,
                    "char" => WitTokenType::CharKw,
                    "string" => WitTokenType::StringKw,
                    _ => WitTokenType::Identifier,
                };

                state.add_token(token_kind, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_punctuation<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => {
                    state.advance(1);
                    WitTokenType::LeftParen
                }
                ')' => {
                    state.advance(1);
                    WitTokenType::RightParen
                }
                '{' => {
                    state.advance(1);
                    WitTokenType::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    WitTokenType::RightBrace
                }
                '[' => {
                    state.advance(1);
                    WitTokenType::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    WitTokenType::RightBracket
                }
                '<' => {
                    state.advance(1);
                    WitTokenType::Lt
                }
                '>' => {
                    state.advance(1);
                    WitTokenType::Gt
                }
                ',' => {
                    state.advance(1);
                    WitTokenType::Comma
                }
                ';' => {
                    state.advance(1);
                    WitTokenType::Semicolon
                }
                ':' => {
                    state.advance(1);
                    WitTokenType::Colon
                }
                '=' => {
                    state.advance(1);
                    WitTokenType::Assign
                }
                '.' => {
                    state.advance(1);
                    WitTokenType::Dot
                }
                '*' => {
                    state.advance(1);
                    WitTokenType::Star
                }
                '/' => {
                    state.advance(1);
                    WitTokenType::Slash
                }
                '@' => {
                    state.advance(1);
                    WitTokenType::At
                }
                '-' => {
                    state.advance(1);
                    if state.peek() == Some('>') {
                        state.advance(1);
                        WitTokenType::Arrow
                    }
                    else {
                        WitTokenType::Minus
                    }
                }
                _ => return false,
            };

            state.add_token(token_kind, start_pos, state.get_position());
            return true;
        }

        false
    }

    fn lex_text<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            state.advance(ch.len_utf8());
            state.add_token(WitTokenType::Error, start_pos, state.get_position());
            return true;
        }

        false
    }
}

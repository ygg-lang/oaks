#![doc = include_str!("readme.md")]
use oak_core::{
    Lexer, LexerCache, LexerState, OakError, Source,
    lexer::{CommentConfig, LexOutput, StringConfig},
};
/// Token types for the JASM language.
pub mod token_type;

use crate::{language::JasmLanguage, lexer::token_type::JasmTokenType};
use std::sync::LazyLock;

pub(crate) type State<'a, S> = LexerState<'a, S, JasmLanguage>;

static JASM_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "//", block_start: "", block_end: "", nested_blocks: false });
static JASM_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });

/// Lexer for the JASM language.
#[derive(Clone, Debug)]
pub struct JasmLexer<'config> {
    config: &'config JasmLanguage,
}

impl<'config> Lexer<JasmLanguage> for JasmLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::TextEdit], _cache: &'a mut impl LexerCache<JasmLanguage>) -> LexOutput<JasmLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> JasmLexer<'config> {
    /// Creates a new `JasmLexer`.
    pub fn new(config: &'config JasmLanguage) -> Self {
        Self { config }
    }

    /// Main lexing loop.
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
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

            state.advance_if_dead_lock(safe_point);
        }

        // Add EOF token
        state.add_eof();
        Ok(())
    }

    /// Skips whitespace characters (excluding newlines).
    fn skip_whitespace<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start {
            state.add_token(JasmTokenType::Whitespace, start, state.get_position());
            return true;
        }

        false
    }

    /// Handles newlines.
    fn lex_newline<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();

        if state.current() == Some('\n') {
            state.advance(1);
            state.add_token(JasmTokenType::Newline, start, state.get_position());
            return true;
        }
        false
    }

    /// Skips comments.
    fn skip_comment<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        if !self.config.comments {
            return false;
        }
        JASM_COMMENT.scan(state, JasmTokenType::Comment, JasmTokenType::Comment)
    }

    /// Handles string literals.
    fn lex_string_literal<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        JASM_STRING.scan(state, JasmTokenType::String)
    }

    /// Handles number literals.
    fn lex_number_literal<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        let first = match state.peek() {
            Some(c) => c,
            None => return false,
        };

        // Check if starts with a digit or sign
        if !first.is_ascii_digit() && first != '-' && first != '+' {
            return false;
        }

        // If sign, check if followed by a digit
        if first == '-' || first == '+' {
            if let Some(next) = state.peek_next_n(1) {
                if !next.is_ascii_digit() {
                    return false;
                }
            }
            else {
                return false;
            }
        }

        state.advance(first.len_utf8());
        let mut has_dot = false;
        let mut has_exp = false;

        while let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(ch.len_utf8());
            }
            else if ch == '.' && !has_dot && !has_exp {
                has_dot = true;
                state.advance(1);
            }
            else if (ch == 'e' || ch == 'E') && !has_exp {
                has_exp = true;
                state.advance(1);
                // Handle exponent sign
                if let Some(sign) = state.peek() {
                    if sign == '+' || sign == '-' {
                        state.advance(1);
                    }
                }
            }
            else {
                break;
            }
        }

        state.add_token(JasmTokenType::Number, start, state.get_position());
        true
    }

    /// Handles identifiers or keywords.
    fn lex_identifier_or_keyword<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        let ch = match state.peek() {
            Some(c) => c,
            None => return false,
        };

        // Identifier must start with a letter or underscore
        if !(ch.is_ascii_alphabetic() || ch == '_') {
            return false;
        }

        state.advance(ch.len_utf8());
        while let Some(c) = state.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                state.advance(c.len_utf8());
            }
            else {
                break;
            }
        }

        let end = state.get_position();
        let text = state.get_text_in((start..end).into());

        // Check if keyword or instruction
        let kind = self.classify_identifier(&text);
        state.add_token(kind, start, state.get_position());
        true
    }

    /// Classifies an identifier as a keyword, instruction, or identifier.
    fn classify_identifier(&self, text: &str) -> JasmTokenType {
        match text {
            // Keywords
            "class" => JasmTokenType::ClassKw,
            "version" => JasmTokenType::VersionKw,
            "method" => JasmTokenType::MethodKw,
            "field" => JasmTokenType::FieldKw,
            "string" => JasmTokenType::StringKw,
            "source" => JasmTokenType::SourceKw,
            "sourcefile" => JasmTokenType::SourceFileKw,
            "stack" => JasmTokenType::StackKw,
            "locals" => JasmTokenType::LocalsKw,
            "end" => JasmTokenType::EndKw,
            "compiled" => JasmTokenType::CompiledKw,
            "from" => JasmTokenType::FromKw,
            "innerclass" => JasmTokenType::InnerClassKw,
            "nestmembers" => JasmTokenType::NestMembersKw,
            "bootstrapmethod" => JasmTokenType::BootstrapMethodKw,
            "interface" => JasmTokenType::InterfaceKw,
            "extends" => JasmTokenType::ExtendsKw,
            "implements" => JasmTokenType::ImplementsKw,
            "catch" => JasmTokenType::CatchKw,
            "attribute" => JasmTokenType::AttributeKw,
            "stackmap" => JasmTokenType::StackMapKw,

            // Access modifiers
            "public" => JasmTokenType::Public,
            "private" => JasmTokenType::Private,
            "protected" => JasmTokenType::Protected,
            "static" => JasmTokenType::Static,
            "super" => JasmTokenType::Super,
            "final" => JasmTokenType::Final,
            "abstract" => JasmTokenType::Abstract,
            "synchronized" => JasmTokenType::Synchronized,
            "native" => JasmTokenType::Native,
            "synthetic" => JasmTokenType::Synthetic,
            "deprecated" => JasmTokenType::Deprecated,
            "varargs" => JasmTokenType::Varargs,

            // Base bytecode instructions
            "aload_0" => JasmTokenType::ALoad0,
            "aload_1" => JasmTokenType::ALoad1,
            "aload_2" => JasmTokenType::ALoad2,
            "aload_3" => JasmTokenType::ALoad3,
            "iload_0" => JasmTokenType::ILoad0,
            "iload_1" => JasmTokenType::ILoad1,
            "iload_2" => JasmTokenType::ILoad2,
            "iload_3" => JasmTokenType::ILoad3,
            "ldc" => JasmTokenType::Ldc,
            "ldc_w" => JasmTokenType::LdcW,
            "ldc2_w" => JasmTokenType::Ldc2W,
            "invokespecial" => JasmTokenType::InvokeSpecial,
            "invokevirtual" => JasmTokenType::InvokeVirtual,
            "invokestatic" => JasmTokenType::InvokeStatic,
            "getstatic" => JasmTokenType::GetStatic,
            "putstatic" => JasmTokenType::PutStatic,
            "getfield" => JasmTokenType::GetField,
            "putfield" => JasmTokenType::PutField,
            "return" => JasmTokenType::Return,
            "ireturn" => JasmTokenType::IReturn,
            "areturn" => JasmTokenType::AReturn,
            "lreturn" => JasmTokenType::LReturn,
            "freturn" => JasmTokenType::FReturn,
            "dreturn" => JasmTokenType::DReturn,
            "nop" => JasmTokenType::Nop,
            "dup" => JasmTokenType::Dup,
            "pop" => JasmTokenType::Pop,
            "new" => JasmTokenType::New,

            // Extended bytecode instructions (only if extended mode is enabled)
            _ if self.config.extended => match text {
                "invokeinterface" => JasmTokenType::InvokeInterface,
                "invokedynamic" => JasmTokenType::InvokeDynamic,
                "checkcast" => JasmTokenType::CheckCast,
                "instanceof" => JasmTokenType::InstanceOf,
                "newarray" => JasmTokenType::NewArray,
                "anewarray" => JasmTokenType::ANewArray,
                "arraylength" => JasmTokenType::ArrayLength,
                "athrow" => JasmTokenType::AThrow,
                "monitorenter" => JasmTokenType::MonitorEnter,
                "monitorexit" => JasmTokenType::MonitorExit,
                "multianewarray" => JasmTokenType::MultiANewArray,
                "ifnull" => JasmTokenType::IfNull,
                "ifnonnull" => JasmTokenType::IfNonNull,
                "goto" => JasmTokenType::Goto,
                "goto_w" => JasmTokenType::GotoW,
                "jsr" => JasmTokenType::Jsr,
                "jsr_w" => JasmTokenType::JsrW,
                "ret" => JasmTokenType::Ret,
                "tableswitch" => JasmTokenType::TableSwitch,
                "lookupswitch" => JasmTokenType::LookupSwitch,
                "bipush" => JasmTokenType::BiPush,
                "sipush" => JasmTokenType::SiPush,
                "iinc" => JasmTokenType::IInc,
                "wide" => JasmTokenType::Wide,
                "breakpoint" => JasmTokenType::BreakPoint,
                "impdep1" => JasmTokenType::ImpDep1,
                "impdep2" => JasmTokenType::ImpDep2,
                _ => JasmTokenType::Identifier,
            },

            // Default to identifier
            _ => JasmTokenType::Identifier,
        }
    }

    /// Handles punctuation marks.
    fn lex_punctuation<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.current() {
            let kind = match ch {
                '{' => JasmTokenType::LeftBrace,
                '}' => JasmTokenType::RightBrace,
                '(' => JasmTokenType::LeftParen,
                ')' => JasmTokenType::RightParen,
                '[' => JasmTokenType::LeftBracket,
                ']' => JasmTokenType::RightBracket,
                ':' => JasmTokenType::Colon,
                ';' => JasmTokenType::Semicolon,
                '.' => JasmTokenType::Dot,
                ',' => JasmTokenType::Comma,
                '/' => JasmTokenType::Slash,
                '@' => JasmTokenType::At,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(kind, start, state.get_position());
            return true;
        }

        false
    }
}

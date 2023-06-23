#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::DLanguage, lexer::token_type::DTokenType};
use oak_core::{Lexer, LexerCache, LexerState, TextEdit, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, DLanguage>;

/// Lexer implementation for D programming language
#[derive(Clone)]
pub struct DLexer<'config> {
    _config: &'config DLanguage,
}

impl<'config> Lexer<DLanguage> for DLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<DLanguage>) -> LexOutput<DLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> DLexer<'config> {
    pub fn new(config: &'config DLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        while state.not_at_end() {
            let start_pos = state.get_position();

            // 尝试各种词法规则
            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.lex_line_comment(state) {
                continue;
            }

            if self.lex_block_comment(state) {
                continue;
            }

            if self.lex_nested_comment(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_string(state) {
                continue;
            }

            if self.lex_character(state) {
                continue;
            }

            if self.lex_operator(state) {
                continue;
            }

            if self.lex_delimiter(state) {
                continue;
            }

            // 如果没有匹配任何规则，添加错误token并强行推进，防止死循环
            state.advance_if_dead_lock(start_pos);
            if state.get_position() > start_pos {
                state.add_token(DTokenType::Error, start_pos, state.get_position())
            }
        }
        Ok(())
    }

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_whitespace() && ch != '\n' && ch != '\r' {
                let start_pos = state.get_position();
                while let Some(ch) = state.peek() {
                    if !ch.is_whitespace() || ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8())
                }
                state.add_token(DTokenType::Whitespace, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch == '\n' || ch == '\r' {
                let start_pos = state.get_position();
                if ch == '\r' {
                    state.advance(1);
                    if state.peek() == Some('\n') {
                        state.advance(1)
                    }
                }
                else {
                    state.advance(1)
                }
                state.add_token(DTokenType::Newline, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                let start_pos = state.get_position();
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' { state.advance(ch.len_utf8()) } else { break }
                }
                let end_pos = state.get_position();
                let text = state.get_text_in((start_pos..end_pos).into());

                let kind = match text.as_ref() {
                    "module" => DTokenType::ModuleKeyword,
                    "import" => DTokenType::ImportKeyword,
                    "public" => DTokenType::PublicKeyword,
                    "private" => DTokenType::PrivateKeyword,
                    "protected" => DTokenType::ProtectedKeyword,
                    "package" => DTokenType::PackageKeyword,
                    "export" => DTokenType::ExportKeyword,
                    "static" => DTokenType::StaticKeyword,
                    "final" => DTokenType::FinalKeyword,
                    "abstract" => DTokenType::AbstractKeyword,
                    "override" => DTokenType::OverrideKeyword,
                    "synchronized" => DTokenType::SynchronizedKeyword,
                    "const" => DTokenType::ConstKeyword,
                    "immutable" => DTokenType::ImmutableKeyword,
                    "inout" => DTokenType::InoutKeyword,
                    "shared" => DTokenType::SharedKeyword,
                    "class" => DTokenType::ClassKeyword,
                    "struct" => DTokenType::StructKeyword,
                    "interface" => DTokenType::InterfaceKeyword,
                    "union" => DTokenType::UnionKeyword,
                    "enum" => DTokenType::EnumKeyword,
                    "function" => DTokenType::FunctionKeyword,
                    "delegate" => DTokenType::DelegateKeyword,
                    "if" => DTokenType::IfKeyword,
                    "else" => DTokenType::ElseKeyword,
                    "while" => DTokenType::WhileKeyword,
                    "for" => DTokenType::ForKeyword,
                    "foreach" => DTokenType::ForeachKeyword,
                    "do" => DTokenType::DoKeyword,
                    "switch" => DTokenType::SwitchKeyword,
                    "case" => DTokenType::CaseKeyword,
                    "default" => DTokenType::DefaultKeyword,
                    "break" => DTokenType::BreakKeyword,
                    "continue" => DTokenType::ContinueKeyword,
                    "return" => DTokenType::ReturnKeyword,
                    "goto" => DTokenType::GotoKeyword,
                    "try" => DTokenType::TryKeyword,
                    "catch" => DTokenType::CatchKeyword,
                    "finally" => DTokenType::FinallyKeyword,
                    "throw" => DTokenType::ThrowKeyword,
                    "scope" => DTokenType::ScopeKeyword,
                    "with" => DTokenType::WithKeyword,
                    "asm" => DTokenType::AsmKeyword,
                    "mixin" => DTokenType::MixinKeyword,
                    "template" => DTokenType::TemplateKeyword,
                    "alias" => DTokenType::AliasKeyword,
                    "typeof" => DTokenType::TypeofKeyword,
                    "typeid" => DTokenType::TypeidKeyword,
                    "is" => DTokenType::IsKeyword,
                    "in" => DTokenType::InKeyword,
                    "out" => DTokenType::OutKeyword,
                    "ref" => DTokenType::RefKeyword,
                    "lazy" => DTokenType::LazyKeyword,
                    "auto" => DTokenType::AutoKeyword,
                    "extern" => DTokenType::ExternKeyword,
                    "align" => DTokenType::AlignKeyword,
                    "pragma" => DTokenType::PragmaKeyword,
                    "debug" => DTokenType::DebugKeyword,
                    "version" => DTokenType::VersionKeyword,
                    "unittest" => DTokenType::UnitTestKeyword,
                    "invariant" => DTokenType::InvariantKeyword,
                    "body" => DTokenType::BodyKeyword,
                    "new" => DTokenType::NewKeyword,
                    "delete" => DTokenType::DeleteKeyword,
                    "this" => DTokenType::ThisKeyword,
                    "super" => DTokenType::SuperKeyword,
                    "null" => DTokenType::NullKeyword,
                    "true" => DTokenType::TrueKeyword,
                    "false" => DTokenType::FalseKeyword,
                    "cast" => DTokenType::CastKeyword,
                    "void" => DTokenType::VoidType,
                    "bool" => DTokenType::BoolType,
                    "byte" => DTokenType::ByteType,
                    "ubyte" => DTokenType::UbyteType,
                    "short" => DTokenType::ShortType,
                    "ushort" => DTokenType::UshortType,
                    "int" => DTokenType::IntType,
                    "uint" => DTokenType::UintType,
                    "long" => DTokenType::LongType,
                    "ulong" => DTokenType::UlongType,
                    "cent" => DTokenType::CentType,
                    "ucent" => DTokenType::UcentType,
                    "float" => DTokenType::FloatType,
                    "double" => DTokenType::DoubleType,
                    "real" => DTokenType::RealType,
                    "ifloat" => DTokenType::IfloatType,
                    "idouble" => DTokenType::IdoubleType,
                    "ireal" => DTokenType::IrealType,
                    "cfloat" => DTokenType::CfloatType,
                    "cdouble" => DTokenType::CdoubleType,
                    "creal" => DTokenType::CrealType,
                    "char" => DTokenType::CharType,
                    "wchar" => DTokenType::WcharType,
                    "dchar" => DTokenType::DcharType,
                    "string" => DTokenType::StringType,
                    "wstring" => DTokenType::WstringType,
                    "dstring" => DTokenType::DstringType,
                    "typedef" => DTokenType::TypedefKeyword,
                    "pure" => DTokenType::PureKeyword,
                    "nothrow" => DTokenType::NothrowKeyword,
                    "safe" => DTokenType::SafeKeyword,
                    "trusted" => DTokenType::TrustedKeyword,
                    "system" => DTokenType::SystemKeyword,
                    "nogc" => DTokenType::NogcKeyword,
                    "property" => DTokenType::PropertyKeyword,
                    "disable" => DTokenType::DisableKeyword,
                    "deprecated" => DTokenType::DeprecatedKeyword,
                    _ => DTokenType::Identifier,
                };

                state.add_token(kind, start_pos, end_pos);
                return true;
            }
        }
        false
    }

    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                let start_pos = state.get_position();

                // 处理数字
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '_' { state.advance(ch.len_utf8()) } else { break }
                }

                // 检查小数点
                if let Some('.') = state.peek() {
                    state.advance(1);
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() || ch == '_' { state.advance(ch.len_utf8()) } else { break }
                    }
                }

                // 检查指数
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' {
                        state.advance(1);
                        if let Some(ch) = state.peek() {
                            if ch == '+' || ch == '-' {
                                state.advance(1)
                            }
                        }
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() || ch == '_' { state.advance(ch.len_utf8()) } else { break }
                        }
                    }
                }

                // 检查后缀
                if let Some(ch) = state.peek() {
                    if ch == 'f' || ch == 'F' || ch == 'L' || ch == 'u' || ch == 'U' {
                        state.advance(1)
                    }
                }

                state.add_token(DTokenType::IntegerLiteral, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch == '"' || ch == '\'' {
                let start_pos = state.get_position();
                let quote = ch;
                state.advance(1); // consume opening quote

                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1); // consume closing quote
                        break;
                    }
                    else if ch == '\\' {
                        state.advance(1); // consume backslash
                        if state.peek().is_some() {
                            state.advance(1); // consume escaped character
                        }
                    }
                    else {
                        state.advance(ch.len_utf8())
                    }
                }

                state.add_token(DTokenType::StringLiteral, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_character<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some('\'') = state.peek() {
            let start_pos = state.get_position();
            state.advance(1); // consume opening quote

            if let Some(ch) = state.peek() {
                if ch == '\\' {
                    state.advance(1); // consume backslash
                    if state.peek().is_some() {
                        state.advance(1); // consume escaped character
                    }
                }
                else {
                    state.advance(ch.len_utf8())
                }
            }

            if let Some('\'') = state.peek() {
                state.advance(1); // consume closing quote
            }

            state.add_token(DTokenType::CharLiteral, start_pos, state.get_position());
            return true;
        }
        false
    }

    fn lex_line_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some('/') = state.peek() {
            if let Some('/') = state.peek_next_n(1) {
                let start_pos = state.get_position();
                state.advance(2);
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8())
                }
                state.add_token(DTokenType::LineComment, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_block_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some('/') = state.peek() {
            if let Some('*') = state.peek_next_n(1) {
                let start_pos = state.get_position();
                state.advance(2);
                while let Some(ch) = state.peek() {
                    if ch == '*' {
                        state.advance(1);
                        if state.peek() == Some('/') {
                            state.advance(1);
                            break;
                        }
                    }
                    else {
                        state.advance(ch.len_utf8())
                    }
                }
                state.add_token(DTokenType::BlockComment, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_nested_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some('/') = state.peek() {
            if let Some('+') = state.peek_next_n(1) {
                let start_pos = state.get_position();
                state.advance(2);
                let mut depth = 1;
                while let Some(ch) = state.peek() {
                    if ch == '/' {
                        state.advance(1);
                        if state.peek() == Some('+') {
                            state.advance(1);
                            depth += 1
                        }
                    }
                    else if ch == '+' {
                        state.advance(1);
                        if state.peek() == Some('/') {
                            state.advance(1);
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }
                    }
                    else {
                        state.advance(ch.len_utf8())
                    }
                }
                state.add_token(DTokenType::NestedComment, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_operator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(ch) = state.peek() {
            let start_pos = state.get_position();

            match ch {
                '+' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(DTokenType::PlusAssign, start_pos, state.get_position())
                    }
                    else if let Some('+') = state.peek() {
                        state.advance(1);
                        state.add_token(DTokenType::Increment, start_pos, state.get_position())
                    }
                    else {
                        state.add_token(DTokenType::Plus, start_pos, state.get_position())
                    }
                    return true;
                }
                '-' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(DTokenType::MinusAssign, start_pos, state.get_position())
                    }
                    else if let Some('-') = state.peek() {
                        state.advance(1);
                        state.add_token(DTokenType::Decrement, start_pos, state.get_position())
                    }
                    else {
                        state.add_token(DTokenType::Minus, start_pos, state.get_position())
                    }
                    return true;
                }
                '*' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(DTokenType::MultiplyAssign, start_pos, state.get_position())
                    }
                    else {
                        state.add_token(DTokenType::Multiply, start_pos, state.get_position())
                    }
                    return true;
                }
                '/' => {
                    // 已在注释处理中处理
                    return false;
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(DTokenType::ModuloAssign, start_pos, state.get_position())
                    }
                    else {
                        state.add_token(DTokenType::Modulo, start_pos, state.get_position())
                    }
                    return true;
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        state.add_token(DTokenType::LogicalAnd, start_pos, state.get_position())
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(DTokenType::BitwiseAndAssign, start_pos, state.get_position())
                    }
                    else {
                        state.add_token(DTokenType::BitwiseAnd, start_pos, state.get_position())
                    }
                    return true;
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        state.add_token(DTokenType::LogicalOr, start_pos, state.get_position())
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(DTokenType::BitwiseOrAssign, start_pos, state.get_position())
                    }
                    else {
                        state.add_token(DTokenType::BitwiseOr, start_pos, state.get_position())
                    }
                    return true;
                }
                '^' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(DTokenType::BitwiseXorAssign, start_pos, state.get_position())
                    }
                    else {
                        state.add_token(DTokenType::BitwiseXor, start_pos, state.get_position())
                    }
                    return true;
                }
                '~' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(DTokenType::ConcatenateAssign, start_pos, state.get_position())
                    }
                    else {
                        state.add_token(DTokenType::BitwiseNot, start_pos, state.get_position())
                    }
                    return true;
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(DTokenType::NotEqual, start_pos, state.get_position())
                    }
                    else {
                        state.add_token(DTokenType::Not, start_pos, state.get_position())
                    }
                    return true;
                }
                '<' => {
                    state.advance(1);
                    if let Some('<') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            state.add_token(DTokenType::LeftShiftAssign, start_pos, state.get_position())
                        }
                        else {
                            state.add_token(DTokenType::LeftShift, start_pos, state.get_position())
                        }
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(DTokenType::LessEqual, start_pos, state.get_position())
                    }
                    else {
                        state.add_token(DTokenType::Less, start_pos, state.get_position())
                    }
                    return true;
                }
                '>' => {
                    state.advance(1);
                    if let Some('>') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            state.add_token(DTokenType::RightShiftAssign, start_pos, state.get_position())
                        }
                        else {
                            state.add_token(DTokenType::RightShift, start_pos, state.get_position())
                        }
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(DTokenType::GreaterEqual, start_pos, state.get_position())
                    }
                    else {
                        state.add_token(DTokenType::Greater, start_pos, state.get_position())
                    }
                    return true;
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(DTokenType::Equal, start_pos, state.get_position())
                    }
                    else {
                        state.add_token(DTokenType::Assign, start_pos, state.get_position())
                    }
                    return true;
                }
                _ => false,
            }
        }
        else {
            false
        }
    }

    fn lex_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(ch) = state.peek() {
            let start_pos = state.get_position();

            match ch {
                '(' => {
                    state.advance(1);
                    state.add_token(DTokenType::LeftParen, start_pos, state.get_position());
                    return true;
                }
                ')' => {
                    state.advance(1);
                    state.add_token(DTokenType::RightParen, start_pos, state.get_position());
                    return true;
                }
                '[' => {
                    state.advance(1);
                    state.add_token(DTokenType::LeftBracket, start_pos, state.get_position());
                    return true;
                }
                ']' => {
                    state.advance(1);
                    state.add_token(DTokenType::RightBracket, start_pos, state.get_position());
                    return true;
                }
                '{' => {
                    state.advance(1);
                    state.add_token(DTokenType::LeftBrace, start_pos, state.get_position());
                    return true;
                }
                '}' => {
                    state.advance(1);
                    state.add_token(DTokenType::RightBrace, start_pos, state.get_position());
                    return true;
                }
                ';' => {
                    state.advance(1);
                    state.add_token(DTokenType::Semicolon, start_pos, state.get_position());
                    return true;
                }
                ',' => {
                    state.advance(1);
                    state.add_token(DTokenType::Comma, start_pos, state.get_position());
                    return true;
                }
                '.' => {
                    state.advance(1);
                    state.add_token(DTokenType::Dot, start_pos, state.get_position());
                    return true;
                }
                ':' => {
                    state.advance(1);
                    state.add_token(DTokenType::Colon, start_pos, state.get_position());
                    return true;
                }
                '?' => {
                    state.advance(1);
                    state.add_token(DTokenType::Question, start_pos, state.get_position());
                    return true;
                }
                _ => false,
            }
        }
        else {
            false
        }
    }
}

use crate::{kind::DSyntaxKind, language::DLanguage};
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
            state.add_eof();
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
                state.add_token(DSyntaxKind::Error, start_pos, state.get_position());
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
                    state.advance(ch.len_utf8());
                }
                state.add_token(DSyntaxKind::Whitespace, start_pos, state.get_position());
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
                        state.advance(1);
                    }
                }
                else {
                    state.advance(1);
                }
                state.add_token(DSyntaxKind::Newline, start_pos, state.get_position());
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
                    if ch.is_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                let end_pos = state.get_position();
                let text = state.get_text_in((start_pos..end_pos).into());

                let kind = match text.as_ref() {
                    "module" => DSyntaxKind::ModuleKeyword,
                    "import" => DSyntaxKind::ImportKeyword,
                    "public" => DSyntaxKind::PublicKeyword,
                    "private" => DSyntaxKind::PrivateKeyword,
                    "protected" => DSyntaxKind::ProtectedKeyword,
                    "package" => DSyntaxKind::PackageKeyword,
                    "export" => DSyntaxKind::ExportKeyword,
                    "static" => DSyntaxKind::StaticKeyword,
                    "final" => DSyntaxKind::FinalKeyword,
                    "abstract" => DSyntaxKind::AbstractKeyword,
                    "override" => DSyntaxKind::OverrideKeyword,
                    "synchronized" => DSyntaxKind::SynchronizedKeyword,
                    "const" => DSyntaxKind::ConstKeyword,
                    "immutable" => DSyntaxKind::ImmutableKeyword,
                    "inout" => DSyntaxKind::InoutKeyword,
                    "shared" => DSyntaxKind::SharedKeyword,
                    "class" => DSyntaxKind::ClassKeyword,
                    "struct" => DSyntaxKind::StructKeyword,
                    "interface" => DSyntaxKind::InterfaceKeyword,
                    "union" => DSyntaxKind::UnionKeyword,
                    "enum" => DSyntaxKind::EnumKeyword,
                    "function" => DSyntaxKind::FunctionKeyword,
                    "delegate" => DSyntaxKind::DelegateKeyword,
                    "if" => DSyntaxKind::IfKeyword,
                    "else" => DSyntaxKind::ElseKeyword,
                    "while" => DSyntaxKind::WhileKeyword,
                    "for" => DSyntaxKind::ForKeyword,
                    "foreach" => DSyntaxKind::ForeachKeyword,
                    "do" => DSyntaxKind::DoKeyword,
                    "switch" => DSyntaxKind::SwitchKeyword,
                    "case" => DSyntaxKind::CaseKeyword,
                    "default" => DSyntaxKind::DefaultKeyword,
                    "break" => DSyntaxKind::BreakKeyword,
                    "continue" => DSyntaxKind::ContinueKeyword,
                    "return" => DSyntaxKind::ReturnKeyword,
                    "goto" => DSyntaxKind::GotoKeyword,
                    "try" => DSyntaxKind::TryKeyword,
                    "catch" => DSyntaxKind::CatchKeyword,
                    "finally" => DSyntaxKind::FinallyKeyword,
                    "throw" => DSyntaxKind::ThrowKeyword,
                    "scope" => DSyntaxKind::ScopeKeyword,
                    "with" => DSyntaxKind::WithKeyword,
                    "asm" => DSyntaxKind::AsmKeyword,
                    "mixin" => DSyntaxKind::MixinKeyword,
                    "template" => DSyntaxKind::TemplateKeyword,
                    "alias" => DSyntaxKind::AliasKeyword,
                    "typeof" => DSyntaxKind::TypeofKeyword,
                    "typeid" => DSyntaxKind::TypeidKeyword,
                    "is" => DSyntaxKind::IsKeyword,
                    "in" => DSyntaxKind::InKeyword,
                    "out" => DSyntaxKind::OutKeyword,
                    "ref" => DSyntaxKind::RefKeyword,
                    "lazy" => DSyntaxKind::LazyKeyword,
                    "auto" => DSyntaxKind::AutoKeyword,
                    "extern" => DSyntaxKind::ExternKeyword,
                    "align" => DSyntaxKind::AlignKeyword,
                    "pragma" => DSyntaxKind::PragmaKeyword,
                    "debug" => DSyntaxKind::DebugKeyword,
                    "version" => DSyntaxKind::VersionKeyword,
                    "unittest" => DSyntaxKind::UnitTestKeyword,
                    "invariant" => DSyntaxKind::InvariantKeyword,
                    "body" => DSyntaxKind::BodyKeyword,
                    "new" => DSyntaxKind::NewKeyword,
                    "delete" => DSyntaxKind::DeleteKeyword,
                    "this" => DSyntaxKind::ThisKeyword,
                    "super" => DSyntaxKind::SuperKeyword,
                    "null" => DSyntaxKind::NullKeyword,
                    "true" => DSyntaxKind::TrueKeyword,
                    "false" => DSyntaxKind::FalseKeyword,
                    "cast" => DSyntaxKind::CastKeyword,
                    "void" => DSyntaxKind::VoidType,
                    "bool" => DSyntaxKind::BoolType,
                    "byte" => DSyntaxKind::ByteType,
                    "ubyte" => DSyntaxKind::UbyteType,
                    "short" => DSyntaxKind::ShortType,
                    "ushort" => DSyntaxKind::UshortType,
                    "int" => DSyntaxKind::IntType,
                    "uint" => DSyntaxKind::UintType,
                    "long" => DSyntaxKind::LongType,
                    "ulong" => DSyntaxKind::UlongType,
                    "cent" => DSyntaxKind::CentType,
                    "ucent" => DSyntaxKind::UcentType,
                    "float" => DSyntaxKind::FloatType,
                    "double" => DSyntaxKind::DoubleType,
                    "real" => DSyntaxKind::RealType,
                    "ifloat" => DSyntaxKind::IfloatType,
                    "idouble" => DSyntaxKind::IdoubleType,
                    "ireal" => DSyntaxKind::IrealType,
                    "cfloat" => DSyntaxKind::CfloatType,
                    "cdouble" => DSyntaxKind::CdoubleType,
                    "creal" => DSyntaxKind::CrealType,
                    "char" => DSyntaxKind::CharType,
                    "wchar" => DSyntaxKind::WcharType,
                    "dchar" => DSyntaxKind::DcharType,
                    "string" => DSyntaxKind::StringType,
                    "wstring" => DSyntaxKind::WstringType,
                    "dstring" => DSyntaxKind::DstringType,
                    "typedef" => DSyntaxKind::TypedefKeyword,
                    "pure" => DSyntaxKind::PureKeyword,
                    "nothrow" => DSyntaxKind::NothrowKeyword,
                    "safe" => DSyntaxKind::SafeKeyword,
                    "trusted" => DSyntaxKind::TrustedKeyword,
                    "system" => DSyntaxKind::SystemKeyword,
                    "nogc" => DSyntaxKind::NogcKeyword,
                    "property" => DSyntaxKind::PropertyKeyword,
                    "disable" => DSyntaxKind::DisableKeyword,
                    "deprecated" => DSyntaxKind::DeprecatedKeyword,
                    _ => DSyntaxKind::Identifier,
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
                    if ch.is_ascii_digit() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 检查小数点
                if let Some('.') = state.peek() {
                    state.advance(1);
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() || ch == '_' {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                }

                // 检查指数
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' {
                        state.advance(1);
                        if let Some(ch) = state.peek() {
                            if ch == '+' || ch == '-' {
                                state.advance(1);
                            }
                        }
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() || ch == '_' {
                                state.advance(ch.len_utf8());
                            }
                            else {
                                break;
                            }
                        }
                    }
                }

                // 检查后缀
                if let Some(ch) = state.peek() {
                    if ch == 'f' || ch == 'F' || ch == 'L' || ch == 'u' || ch == 'U' {
                        state.advance(1);
                    }
                }

                state.add_token(DSyntaxKind::IntegerLiteral, start_pos, state.get_position());
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
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(DSyntaxKind::StringLiteral, start_pos, state.get_position());
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
                    state.advance(ch.len_utf8());
                }
            }

            if let Some('\'') = state.peek() {
                state.advance(1); // consume closing quote
            }

            state.add_token(DSyntaxKind::CharLiteral, start_pos, state.get_position());
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
                    state.advance(ch.len_utf8());
                }
                state.add_token(DSyntaxKind::LineComment, start_pos, state.get_position());
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
                        state.advance(ch.len_utf8());
                    }
                }
                state.add_token(DSyntaxKind::BlockComment, start_pos, state.get_position());
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
                            depth += 1;
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
                        state.advance(ch.len_utf8());
                    }
                }
                state.add_token(DSyntaxKind::NestedComment, start_pos, state.get_position());
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
                        state.add_token(DSyntaxKind::PlusAssign, start_pos, state.get_position());
                    }
                    else if let Some('+') = state.peek() {
                        state.advance(1);
                        state.add_token(DSyntaxKind::Increment, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(DSyntaxKind::Plus, start_pos, state.get_position());
                    }
                    return true;
                }
                '-' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(DSyntaxKind::MinusAssign, start_pos, state.get_position());
                    }
                    else if let Some('-') = state.peek() {
                        state.advance(1);
                        state.add_token(DSyntaxKind::Decrement, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(DSyntaxKind::Minus, start_pos, state.get_position());
                    }
                    return true;
                }
                '*' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(DSyntaxKind::MultiplyAssign, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(DSyntaxKind::Multiply, start_pos, state.get_position());
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
                        state.add_token(DSyntaxKind::ModuloAssign, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(DSyntaxKind::Modulo, start_pos, state.get_position());
                    }
                    return true;
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        state.add_token(DSyntaxKind::LogicalAnd, start_pos, state.get_position());
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(DSyntaxKind::BitwiseAndAssign, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(DSyntaxKind::BitwiseAnd, start_pos, state.get_position());
                    }
                    return true;
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        state.add_token(DSyntaxKind::LogicalOr, start_pos, state.get_position());
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(DSyntaxKind::BitwiseOrAssign, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(DSyntaxKind::BitwiseOr, start_pos, state.get_position());
                    }
                    return true;
                }
                '^' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(DSyntaxKind::BitwiseXorAssign, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(DSyntaxKind::BitwiseXor, start_pos, state.get_position());
                    }
                    return true;
                }
                '~' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(DSyntaxKind::ConcatenateAssign, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(DSyntaxKind::BitwiseNot, start_pos, state.get_position());
                    }
                    return true;
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(DSyntaxKind::NotEqual, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(DSyntaxKind::Not, start_pos, state.get_position());
                    }
                    return true;
                }
                '<' => {
                    state.advance(1);
                    if let Some('<') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            state.add_token(DSyntaxKind::LeftShiftAssign, start_pos, state.get_position());
                        }
                        else {
                            state.add_token(DSyntaxKind::LeftShift, start_pos, state.get_position());
                        }
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(DSyntaxKind::LessEqual, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(DSyntaxKind::Less, start_pos, state.get_position());
                    }
                    return true;
                }
                '>' => {
                    state.advance(1);
                    if let Some('>') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            state.add_token(DSyntaxKind::RightShiftAssign, start_pos, state.get_position());
                        }
                        else {
                            state.add_token(DSyntaxKind::RightShift, start_pos, state.get_position());
                        }
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(DSyntaxKind::GreaterEqual, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(DSyntaxKind::Greater, start_pos, state.get_position());
                    }
                    return true;
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(DSyntaxKind::Equal, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(DSyntaxKind::Assign, start_pos, state.get_position());
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
                    state.add_token(DSyntaxKind::LeftParen, start_pos, state.get_position());
                    return true;
                }
                ')' => {
                    state.advance(1);
                    state.add_token(DSyntaxKind::RightParen, start_pos, state.get_position());
                    return true;
                }
                '[' => {
                    state.advance(1);
                    state.add_token(DSyntaxKind::LeftBracket, start_pos, state.get_position());
                    return true;
                }
                ']' => {
                    state.advance(1);
                    state.add_token(DSyntaxKind::RightBracket, start_pos, state.get_position());
                    return true;
                }
                '{' => {
                    state.advance(1);
                    state.add_token(DSyntaxKind::LeftBrace, start_pos, state.get_position());
                    return true;
                }
                '}' => {
                    state.advance(1);
                    state.add_token(DSyntaxKind::RightBrace, start_pos, state.get_position());
                    return true;
                }
                ';' => {
                    state.advance(1);
                    state.add_token(DSyntaxKind::Semicolon, start_pos, state.get_position());
                    return true;
                }
                ',' => {
                    state.advance(1);
                    state.add_token(DSyntaxKind::Comma, start_pos, state.get_position());
                    return true;
                }
                '.' => {
                    state.advance(1);
                    state.add_token(DSyntaxKind::Dot, start_pos, state.get_position());
                    return true;
                }
                ':' => {
                    state.advance(1);
                    state.add_token(DSyntaxKind::Colon, start_pos, state.get_position());
                    return true;
                }
                '?' => {
                    state.advance(1);
                    state.add_token(DSyntaxKind::Question, start_pos, state.get_position());
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

use crate::{kind::DSyntaxKind, language::DLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, DLanguage>;

pub struct DLexer<'config> {
    config: &'config DLanguage,
}

impl<'config> DLexer<'config> {
    pub fn new(config: &'config DLanguage) -> Self {
        Self { config }
    }

    fn skip_whitespace(&self, state: &mut State) -> bool {
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

    fn lex_newline(&self, state: &mut State) -> bool {
        if let Some(ch) = state.peek() {
            if ch == '\n' || ch == '\r' {
                let start_pos = state.get_position();
                if ch == '\r' {
                    state.advance(1);
                    if let Some('\n') = state.peek() {
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

    fn lex_identifier_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                let start_pos = state.get_position();

                // 第一个字
                state.advance(ch.len_utf8());

                // 后续字符
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let end_pos = state.get_position();
                let text = source.get_text_in((start_pos..end_pos).into()).unwrap_or("");

                // 检查是否为关键
                let kind = match text {
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

    fn lex_number(&self, state: &mut State) -> bool {
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

                // 检查指
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

    fn lex_string(&self, state: &mut State) -> bool {
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

    fn lex_character(&self, state: &mut State) -> bool {
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

    fn lex_line_comment(&self, state: &mut State) -> bool {
        if let Some('/') = state.peek() {
            // 保存当前位置
            let current_pos = state.get_position();
            state.advance(1);
            if let Some('/') = state.peek() {
                // 恢复位置并开始处
                state.set_position(current_pos);
                let start_pos = state.get_position();
                state.advance(2); // skip //

                while let Some(ch) = state.peek() {
                    if ch == '\n' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(DSyntaxKind::LineComment, start_pos, state.get_position());
                return true;
            }
            else {
                // 恢复位置
                state.set_position(current_pos);
            }
        }
        false
    }

    fn lex_block_comment(&self, state: &mut State) -> bool {
        if let Some('/') = state.peek() {
            // 保存当前位置
            let current_pos = state.get_position();
            state.advance(1);
            if let Some('*') = state.peek() {
                // 恢复位置并开始处
                state.set_position(current_pos);
                let start_pos = state.get_position();
                state.advance(2); // skip /*

                while let Some(ch) = state.peek() {
                    if ch == '*' {
                        state.advance(1);
                        if let Some('/') = state.peek() {
                            state.advance(1); // consume closing */
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
            else {
                // 恢复位置
                state.set_position(current_pos);
            }
        }
        false
    }

    fn lex_nested_comment(&self, state: &mut State) -> bool {
        if let Some('/') = state.peek() {
            // 保存当前位置
            let current_pos = state.get_position();
            state.advance(1);
            if let Some('+') = state.peek() {
                // 恢复位置并开始处                state.set_position(current_pos);
                let start_pos = state.get_position();
                state.advance(2); // skip /+

                let mut depth = 1;
                while depth > 0 {
                    let Some(ch) = state.peek()
                    else {
                        break;
                    };
                    if ch == '/' {
                        state.advance(1);
                        if let Some('+') = state.peek() {
                            state.advance(1);
                            depth += 1;
                        }
                    }
                    else if ch == '+' {
                        state.advance(1);
                        if let Some('/') = state.peek() {
                            state.advance(1);
                            depth -= 1;
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(DSyntaxKind::NestedComment, start_pos, state.get_position());
                return true;
            }
            else {
                // 恢复位置
                state.set_position(current_pos);
            }
        }
        false
    }

    fn lex_operator(&self, state: &mut State) -> bool {
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
                    // 已在注释处理中处
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

    fn lex_delimiter(&self, state: &mut State) -> bool {
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

impl<'config> Lexer<DLanguage> for DLexer<'config> {
    fn lex(&self, source: &SourceText) -> oak_core::lexer::LexOutput<DSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_line_comment(&mut state) {
                continue;
            }

            if self.lex_block_comment(&mut state) {
                continue;
            }

            if self.lex_nested_comment(&mut state) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_character(&mut state) {
                continue;
            }

            if self.lex_operator(&mut state) {
                continue;
            }

            if self.lex_delimiter(&mut state) {
                continue;
            }

            // 如果没有匹配任何规则，添加错token
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(DSyntaxKind::Error, start_pos, state.get_position());
            }
            else {
                break;
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(DSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}

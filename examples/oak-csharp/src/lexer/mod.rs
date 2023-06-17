use crate::{CSharpSyntaxKind, language::CSharpLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, CSharpLanguage>;

pub struct CSharpLexer<'config> {
    config: &'config CSharpLanguage,
}

impl<'config> CSharpLexer<'config> {
    pub fn new(config: &'config CSharpLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn skip_whitespace(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(CSharpSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(CSharpSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(CSharpSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State<'_>, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            if let Some('/') = source.get_char_at(start_pos + 1) {
                // 单行注释
                state.advance(2);
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                state.add_token(CSharpSyntaxKind::Comment, start_pos, state.get_position());
                true
            }
            else if let Some('*') = source.get_char_at(start_pos + 1) {
                // 多行注释
                state.advance(2);
                while let Some(ch) = state.peek() {
                    if ch == '*' {
                        if let Some('/') = source.get_char_at(state.get_position() + 1) {
                            state.advance(2);
                            break;
                        }
                    }
                    state.advance(ch.len_utf8());
                }
                state.add_token(CSharpSyntaxKind::Comment, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// 处理标识符或关键
    fn lex_identifier_or_keyword(&self, state: &mut State<'_>, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        // 处理 @ 前缀的逐字标识
        let is_verbatim = if let Some('@') = state.peek() {
            state.advance(1);
            true
        }
        else {
            false
        };

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 如果不是逐字标识符，检查是否为关键
                if !is_verbatim {
                    let text = source.get_text_at(start_pos).unwrap_or("");
                    let text_slice = &text[..(state.get_position() - start_pos)];

                    let token_kind = match text_slice {
                        "abstract" => CSharpSyntaxKind::Abstract,
                        "as" => CSharpSyntaxKind::As,
                        "base" => CSharpSyntaxKind::Base,
                        "bool" => CSharpSyntaxKind::Bool,
                        "break" => CSharpSyntaxKind::Break,
                        "byte" => CSharpSyntaxKind::Byte,
                        "case" => CSharpSyntaxKind::Case,
                        "catch" => CSharpSyntaxKind::Catch,
                        "char" => CSharpSyntaxKind::Char,
                        "checked" => CSharpSyntaxKind::Checked,
                        "class" => CSharpSyntaxKind::Class,
                        "const" => CSharpSyntaxKind::Const,
                        "continue" => CSharpSyntaxKind::Continue,
                        "decimal" => CSharpSyntaxKind::Decimal,
                        "default" => CSharpSyntaxKind::Default,
                        "delegate" => CSharpSyntaxKind::Delegate,
                        "do" => CSharpSyntaxKind::Do,
                        "double" => CSharpSyntaxKind::Double,
                        "else" => CSharpSyntaxKind::Else,
                        "enum" => CSharpSyntaxKind::Enum,
                        "event" => CSharpSyntaxKind::Event,
                        "explicit" => CSharpSyntaxKind::Explicit,
                        "extern" => CSharpSyntaxKind::Extern,
                        "false" => CSharpSyntaxKind::False,
                        "finally" => CSharpSyntaxKind::Finally,
                        "fixed" => CSharpSyntaxKind::Fixed,
                        "float" => CSharpSyntaxKind::Float,
                        "for" => CSharpSyntaxKind::For,
                        "foreach" => CSharpSyntaxKind::Foreach,
                        "goto" => CSharpSyntaxKind::Goto,
                        "if" => CSharpSyntaxKind::If,
                        "implicit" => CSharpSyntaxKind::Implicit,
                        "in" => CSharpSyntaxKind::In,
                        "int" => CSharpSyntaxKind::Int,
                        "interface" => CSharpSyntaxKind::Interface,
                        "internal" => CSharpSyntaxKind::Internal,
                        "is" => CSharpSyntaxKind::Is,
                        "lock" => CSharpSyntaxKind::Lock,
                        "long" => CSharpSyntaxKind::Long,
                        "namespace" => CSharpSyntaxKind::Namespace,
                        "new" => CSharpSyntaxKind::New,
                        "null" => CSharpSyntaxKind::Null,
                        "object" => CSharpSyntaxKind::Object,
                        "operator" => CSharpSyntaxKind::Operator,
                        "out" => CSharpSyntaxKind::Out,
                        "override" => CSharpSyntaxKind::Override,
                        "params" => CSharpSyntaxKind::Params,
                        "private" => CSharpSyntaxKind::Private,
                        "protected" => CSharpSyntaxKind::Protected,
                        "public" => CSharpSyntaxKind::Public,
                        "readonly" => CSharpSyntaxKind::Readonly,
                        "ref" => CSharpSyntaxKind::Ref,
                        "return" => CSharpSyntaxKind::Return,
                        "sbyte" => CSharpSyntaxKind::Sbyte,
                        "sealed" => CSharpSyntaxKind::Sealed,
                        "short" => CSharpSyntaxKind::Short,
                        "sizeof" => CSharpSyntaxKind::Sizeof,
                        "stackalloc" => CSharpSyntaxKind::Stackalloc,
                        "static" => CSharpSyntaxKind::Static,
                        "string" => CSharpSyntaxKind::String,
                        "struct" => CSharpSyntaxKind::Struct,
                        "switch" => CSharpSyntaxKind::Switch,
                        "this" => CSharpSyntaxKind::This,
                        "throw" => CSharpSyntaxKind::Throw,
                        "true" => CSharpSyntaxKind::True,
                        "try" => CSharpSyntaxKind::Try,
                        "typeof" => CSharpSyntaxKind::Typeof,
                        "uint" => CSharpSyntaxKind::Uint,
                        "ulong" => CSharpSyntaxKind::Ulong,
                        "unchecked" => CSharpSyntaxKind::Unchecked,
                        "unsafe" => CSharpSyntaxKind::Unsafe,
                        "ushort" => CSharpSyntaxKind::Ushort,
                        "using" => CSharpSyntaxKind::Using,
                        "virtual" => CSharpSyntaxKind::Virtual,
                        "void" => CSharpSyntaxKind::Void,
                        "volatile" => CSharpSyntaxKind::Volatile,
                        "while" => CSharpSyntaxKind::While,
                        _ => CSharpSyntaxKind::Identifier,
                    };
                    state.add_token(token_kind, start_pos, state.get_position());
                }
                else {
                    state.add_token(CSharpSyntaxKind::Identifier, start_pos, state.get_position());
                }
                true
            }
            else {
                if is_verbatim {
                    // 回退 @ 字符
                    state.set_position(start_pos);
                }
                false
            }
        }
        else {
            if is_verbatim {
                // 回退 @ 字符
                state.set_position(start_pos);
            }
            false
        }
    }

    /// 处理数字字面
    fn lex_number_literal(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);

                // 处理十六进制数字
                if ch == '0' {
                    if let Some('x') | Some('X') = state.peek() {
                        state.advance(1);
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_hexdigit() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                    else {
                        // 处理八进制或普通数
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                }
                else {
                    // 处理十进制数
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                // 处理小数
                if let Some('.') = state.peek() {
                    state.advance(1);
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                // 处理指数
                if let Some('e') | Some('E') = state.peek() {
                    state.advance(1);
                    if let Some('+') | Some('-') = state.peek() {
                        state.advance(1);
                    }
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                // 处理后缀
                if let Some(ch) = state.peek() {
                    match ch {
                        'f' | 'F' | 'd' | 'D' | 'm' | 'M' | 'l' | 'L' | 'u' | 'U' => {
                            state.advance(1);
                        }
                        _ => {}
                    }
                }

                state.add_token(CSharpSyntaxKind::NumberLiteral, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// 处理字符串字面量
    fn lex_string_literal(&self, state: &mut State<'_>, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        // 处理逐字字符@"..."
        let is_verbatim = if let Some('@') = state.peek() {
            if let Some('"') = source.get_char_at(start_pos + 1) {
                state.advance(2);
                true
            }
            else {
                false
            }
        }
        else {
            false
        };

        if !is_verbatim && let Some('"') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    break;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if let Some(_) = state.peek() {
                        state.advance(1);
                    }
                }
                else if ch == '\n' || ch == '\r' {
                    break; // 普通字符串不能跨行
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            state.add_token(CSharpSyntaxKind::StringLiteral, start_pos, state.get_position());
            true
        }
        else if is_verbatim {
            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    if let Some('"') = state.peek() {
                        state.advance(1); // 转义的双引号
                    }
                    else {
                        break;
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            state.add_token(CSharpSyntaxKind::StringLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符字面
    fn lex_char_literal(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            state.advance(1);

            if let Some(ch) = state.peek() {
                if ch == '\\' {
                    state.advance(1);
                    if let Some(_) = state.peek() {
                        state.advance(1);
                    }
                }
                else if ch != '\'' {
                    state.advance(ch.len_utf8());
                }
            }

            if let Some('\'') = state.peek() {
                state.advance(1);
                state.add_token(CSharpSyntaxKind::CharLiteral, start_pos, state.get_position());
                true
            }
            else {
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理操作
    fn lex_operator(&self, state: &mut State<'_>, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    match state.peek() {
                        Some('+') => {
                            state.advance(1);
                            CSharpSyntaxKind::Increment
                        }
                        Some('=') => {
                            state.advance(1);
                            CSharpSyntaxKind::PlusAssign
                        }
                        _ => CSharpSyntaxKind::Plus,
                    }
                }
                '-' => {
                    state.advance(1);
                    match state.peek() {
                        Some('-') => {
                            state.advance(1);
                            CSharpSyntaxKind::Decrement
                        }
                        Some('=') => {
                            state.advance(1);
                            CSharpSyntaxKind::MinusAssign
                        }
                        _ => CSharpSyntaxKind::Minus,
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpSyntaxKind::StarAssign
                    }
                    else {
                        CSharpSyntaxKind::Star
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpSyntaxKind::SlashAssign
                    }
                    else {
                        CSharpSyntaxKind::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpSyntaxKind::PercentAssign
                    }
                    else {
                        CSharpSyntaxKind::Percent
                    }
                }
                '=' => {
                    state.advance(1);
                    match state.peek() {
                        Some('=') => {
                            state.advance(1);
                            CSharpSyntaxKind::Equal
                        }
                        Some('>') => {
                            state.advance(1);
                            CSharpSyntaxKind::Arrow
                        }
                        _ => CSharpSyntaxKind::Assign,
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpSyntaxKind::NotEqual
                    }
                    else {
                        CSharpSyntaxKind::LogicalNot
                    }
                }
                '<' => {
                    state.advance(1);
                    match state.peek() {
                        Some('=') => {
                            state.advance(1);
                            CSharpSyntaxKind::LessEqual
                        }
                        Some('<') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                CSharpSyntaxKind::LeftShiftAssign
                            }
                            else {
                                CSharpSyntaxKind::LeftShift
                            }
                        }
                        _ => CSharpSyntaxKind::Less,
                    }
                }
                '>' => {
                    state.advance(1);
                    match state.peek() {
                        Some('=') => {
                            state.advance(1);
                            CSharpSyntaxKind::GreaterEqual
                        }
                        Some('>') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                CSharpSyntaxKind::RightShiftAssign
                            }
                            else {
                                CSharpSyntaxKind::RightShift
                            }
                        }
                        _ => CSharpSyntaxKind::Greater,
                    }
                }
                '&' => {
                    state.advance(1);
                    match state.peek() {
                        Some('&') => {
                            state.advance(1);
                            CSharpSyntaxKind::LogicalAnd
                        }
                        Some('=') => {
                            state.advance(1);
                            CSharpSyntaxKind::AndAssign
                        }
                        _ => CSharpSyntaxKind::BitAnd,
                    }
                }
                '|' => {
                    state.advance(1);
                    match state.peek() {
                        Some('|') => {
                            state.advance(1);
                            CSharpSyntaxKind::LogicalOr
                        }
                        Some('=') => {
                            state.advance(1);
                            CSharpSyntaxKind::OrAssign
                        }
                        _ => CSharpSyntaxKind::BitOr,
                    }
                }
                '^' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpSyntaxKind::XorAssign
                    }
                    else {
                        CSharpSyntaxKind::BitXor
                    }
                }
                '~' => {
                    state.advance(1);
                    CSharpSyntaxKind::BitNot
                }
                '?' => {
                    state.advance(1);
                    match state.peek() {
                        Some('?') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                CSharpSyntaxKind::QuestionQuestionAssign
                            }
                            else {
                                CSharpSyntaxKind::QuestionQuestion
                            }
                        }
                        _ => CSharpSyntaxKind::Question,
                    }
                }
                '.' => {
                    state.advance(1);
                    CSharpSyntaxKind::Dot
                }
                ':' => {
                    state.advance(1);
                    if let Some(':') = state.peek() {
                        state.advance(1);
                        CSharpSyntaxKind::ColonColon
                    }
                    else {
                        CSharpSyntaxKind::Colon
                    }
                }
                _ => return false,
            };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理分隔
    fn lex_delimiter(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => CSharpSyntaxKind::LeftParen,
                ')' => CSharpSyntaxKind::RightParen,
                '[' => CSharpSyntaxKind::LeftBracket,
                ']' => CSharpSyntaxKind::RightBracket,
                '{' => CSharpSyntaxKind::LeftBrace,
                '}' => CSharpSyntaxKind::RightBrace,
                ',' => CSharpSyntaxKind::Comma,
                ';' => CSharpSyntaxKind::Semicolon,
                '@' => CSharpSyntaxKind::At,
                '#' => CSharpSyntaxKind::Hash,
                '$' => CSharpSyntaxKind::Dollar,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<CSharpLanguage> for CSharpLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<CSharpSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state, source) {
                continue;
            }

            if self.lex_string_literal(&mut state, source) {
                continue;
            }

            if self.lex_char_literal(&mut state) {
                continue;
            }

            if self.lex_number_literal(&mut state) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_operator(&mut state, source) {
                continue;
            }

            if self.lex_delimiter(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(CSharpSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(CSharpSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}

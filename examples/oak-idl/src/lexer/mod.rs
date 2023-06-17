use crate::{kind::IdlSyntaxKind, language::IdlLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, IdlLanguage>;

pub struct IdlLexer<'config> {
    config: &'config IdlLanguage,
}

impl<'config> IdlLexer<'config> {
    pub fn new(config: &'config IdlLanguage) -> Self {
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
            state.add_token(IdlSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(IdlSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(IdlSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        // C++ 风格单行注释
        if state.peek() == Some('/') && state.peek_at(1) == Some('/') {
            state.advance(2);

            // 读取到行尾
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(IdlSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        // C 风格多行注释
        else if state.peek() == Some('/') && state.peek_at(1) == Some('*') {
            state.advance(2);

            while let Some(ch) = state.peek() {
                if ch == '*' && state.peek_at(1) == Some('/') {
                    state.advance(2);
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(IdlSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理预处理器指令
    fn lex_preprocessor(&self, state: &mut State<'_>, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1);

            // 跳过空白
            while let Some(ch) = state.peek() {
                if ch == ' ' || ch == '\t' {
                    state.advance(1);
                }
                else {
                    break;
                }
            }

            // 读取指令名称
            let directive_start = state.get_position();
            while let Some(ch) = state.peek() {
                if ch.is_ascii_alphabetic() || ch == '_' {
                    state.advance(1);
                }
                else {
                    break;
                }
            }

            if state.get_position() > directive_start {
                let directive = source.slice(directive_start, state.get_position());

                let kind = match directive {
                    "include" => IdlSyntaxKind::Include,
                    "pragma" => IdlSyntaxKind::Pragma,
                    "define" => IdlSyntaxKind::Define,
                    "ifdef" => IdlSyntaxKind::Ifdef,
                    "ifndef" => IdlSyntaxKind::Ifndef,
                    "endif" => IdlSyntaxKind::Endif,
                    "else" => IdlSyntaxKind::Else,
                    "elif" => IdlSyntaxKind::Elif,
                    "undef" => IdlSyntaxKind::Undef,
                    _ => IdlSyntaxKind::Hash,
                };

                // 读取到行尾
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(kind, start_pos, state.get_position());
                true
            }
            else {
                state.add_token(IdlSyntaxKind::Hash, start_pos, state.get_position());
                true
            }
        }
        else {
            false
        }
    }

    /// 处理字符串字面量
    fn lex_string(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' {
                state.advance(1);

                let mut escaped = false;
                while let Some(ch) = state.peek() {
                    if escaped {
                        escaped = false;
                        state.advance(ch.len_utf8());
                        continue;
                    }

                    if ch == '\\' {
                        escaped = true;
                        state.advance(1);
                    }
                    else if ch == quote {
                        state.advance(1);
                        break;
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(IdlSyntaxKind::StringLiteral, start_pos, state.get_position());
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

    /// 处理字符字面量
    fn lex_char(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            state.advance(1);

            let mut escaped = false;
            while let Some(ch) = state.peek() {
                if escaped {
                    escaped = false;
                    state.advance(ch.len_utf8());
                    continue;
                }

                if ch == '\\' {
                    escaped = true;
                    state.advance(1);
                }
                else if ch == '\'' {
                    state.advance(1);
                    break;
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            state.add_token(IdlSyntaxKind::CharLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理数字字面量
    fn lex_number(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                // 十六进制数
                if ch == '0' && (state.peek_at(1) == Some('x') || state.peek_at(1) == Some('X')) {
                    state.advance(2);
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_hexdigit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
                // 八进制数
                else if ch == '0' && state.peek_at(1).map_or(false, |c| c.is_ascii_digit()) {
                    state.advance(1);
                    while let Some(ch) = state.peek() {
                        if ch >= '0' && ch <= '7' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
                // 十进制数
                else {
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }

                    // 浮点数
                    if state.peek() == Some('.') && state.peek_at(1).map_or(false, |c| c.is_ascii_digit()) {
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

                    // 指数
                    if let Some(ch) = state.peek() {
                        if ch == 'e' || ch == 'E' {
                            state.advance(1);
                            if let Some(sign) = state.peek() {
                                if sign == '+' || sign == '-' {
                                    state.advance(1);
                                }
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
                    }
                }

                // 数字后缀
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphabetic() || ch == '_' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                state.add_token(IdlSyntaxKind::NumberLiteral, start_pos, state.get_position());
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

    /// 处理标识符和关键字
    fn lex_identifier_or_keyword(&self, state: &mut State<'_>, source: &SourceText) -> bool {
        let start_pos = state.get_position();

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

                let text = source.slice(start_pos, state.get_position());
                let kind = match text {
                    // 基本数据类型
                    "void" => IdlSyntaxKind::Void,
                    "boolean" => IdlSyntaxKind::Boolean,
                    "byte" => IdlSyntaxKind::Byte,
                    "octet" => IdlSyntaxKind::Octet,
                    "short" => IdlSyntaxKind::Short,
                    "unsigned" => {
                        // 检查下一个词是否是 short, long 等
                        if let Some(next_word) = self.peek_next_word(state, source) {
                            match next_word {
                                "short" => IdlSyntaxKind::UnsignedShort,
                                "long" => {
                                    // 检查是否是 unsigned long long
                                    if let Some(third_word) = self.peek_word_after_next(state, source) {
                                        if third_word == "long" {
                                            IdlSyntaxKind::UnsignedLongLong
                                        }
                                        else {
                                            IdlSyntaxKind::UnsignedLong
                                        }
                                    }
                                    else {
                                        IdlSyntaxKind::UnsignedLong
                                    }
                                }
                                _ => IdlSyntaxKind::Identifier,
                            }
                        }
                        else {
                            IdlSyntaxKind::Identifier
                        }
                    }
                    "long" => {
                        // 检查是否是 long long
                        if let Some(next_word) = self.peek_next_word(state, source) {
                            if next_word == "long" { IdlSyntaxKind::LongLong } else { IdlSyntaxKind::Long }
                        }
                        else {
                            IdlSyntaxKind::Long
                        }
                    }
                    "float" => IdlSyntaxKind::Float,
                    "double" => IdlSyntaxKind::Double,
                    "char" => IdlSyntaxKind::Char,
                    "wchar" => IdlSyntaxKind::WChar,
                    "string" => IdlSyntaxKind::String,
                    "wstring" => IdlSyntaxKind::WString,
                    "any" => IdlSyntaxKind::Any,
                    "Object" => IdlSyntaxKind::Object,
                    "ValueBase" => IdlSyntaxKind::ValueBase,

                    // 复合类型关键字
                    "struct" => IdlSyntaxKind::Struct,
                    "union" => IdlSyntaxKind::Union,
                    "enum" => IdlSyntaxKind::Enum,
                    "interface" => IdlSyntaxKind::Interface,
                    "module" => IdlSyntaxKind::Module,
                    "exception" => IdlSyntaxKind::Exception,
                    "typedef" => IdlSyntaxKind::Typedef,
                    "sequence" => IdlSyntaxKind::Sequence,
                    "fixed" => IdlSyntaxKind::Fixed,

                    // 修饰符
                    "const" => IdlSyntaxKind::Const,
                    "readonly" => IdlSyntaxKind::Readonly,
                    "attribute" => IdlSyntaxKind::Attribute,
                    "oneway" => IdlSyntaxKind::Oneway,
                    "in" => IdlSyntaxKind::In,
                    "out" => IdlSyntaxKind::Out,
                    "inout" => IdlSyntaxKind::Inout,
                    "raises" => IdlSyntaxKind::Raises,
                    "context" => IdlSyntaxKind::Context,
                    "local" => IdlSyntaxKind::Local,
                    "abstract" => IdlSyntaxKind::Abstract,
                    "custom" => IdlSyntaxKind::Custom,
                    "private" => IdlSyntaxKind::Private,
                    "public" => IdlSyntaxKind::Public,
                    "truncatable" => IdlSyntaxKind::Truncatable,
                    "supports" => IdlSyntaxKind::Supports,
                    "valuetype" => IdlSyntaxKind::ValueType,
                    "native" => IdlSyntaxKind::Native,
                    "factory" => IdlSyntaxKind::Factory,

                    // 布尔字面量
                    "TRUE" | "FALSE" => IdlSyntaxKind::BooleanLiteral,

                    _ => IdlSyntaxKind::Identifier,
                };

                state.add_token(kind, start_pos, state.get_position());
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

    /// 辅助方法：查看下一个单词
    fn peek_next_word(&self, state: &State<'_>, source: &SourceText) -> Option<&str> {
        let mut pos = state.get_position();

        // 跳过空白
        while let Some(ch) = source.char_at(pos) {
            if ch == ' ' || ch == '\t' {
                pos += ch.len_utf8();
            }
            else {
                break;
            }
        }

        let word_start = pos;
        while let Some(ch) = source.char_at(pos) {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                pos += ch.len_utf8();
            }
            else {
                break;
            }
        }

        if pos > word_start { Some(source.slice(word_start, pos)) } else { None }
    }

    /// 辅助方法：查看下下个单词
    fn peek_word_after_next(&self, state: &State<'_>, source: &SourceText) -> Option<&str> {
        let mut pos = state.get_position();

        // 跳过空白
        while let Some(ch) = source.char_at(pos) {
            if ch == ' ' || ch == '\t' {
                pos += ch.len_utf8();
            }
            else {
                break;
            }
        }

        // 跳过第一个单词
        while let Some(ch) = source.char_at(pos) {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                pos += ch.len_utf8();
            }
            else {
                break;
            }
        }

        // 跳过空白
        while let Some(ch) = source.char_at(pos) {
            if ch == ' ' || ch == '\t' {
                pos += ch.len_utf8();
            }
            else {
                break;
            }
        }

        let word_start = pos;
        while let Some(ch) = source.char_at(pos) {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                pos += ch.len_utf8();
            }
            else {
                break;
            }
        }

        if pos > word_start { Some(source.slice(word_start, pos)) } else { None }
    }

    /// 处理运算符
    fn lex_operators(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '+' => {
                    state.advance(1);
                    IdlSyntaxKind::Plus
                }
                '-' => {
                    state.advance(1);
                    if state.peek() == Some('>') {
                        state.advance(1);
                        IdlSyntaxKind::Arrow
                    }
                    else {
                        IdlSyntaxKind::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    IdlSyntaxKind::Multiply
                }
                '/' => {
                    state.advance(1);
                    IdlSyntaxKind::Divide
                }
                '%' => {
                    state.advance(1);
                    IdlSyntaxKind::Modulo
                }
                '&' => {
                    state.advance(1);
                    if state.peek() == Some('&') {
                        state.advance(1);
                        IdlSyntaxKind::LogicalAnd
                    }
                    else {
                        IdlSyntaxKind::BitwiseAnd
                    }
                }
                '|' => {
                    state.advance(1);
                    if state.peek() == Some('|') {
                        state.advance(1);
                        IdlSyntaxKind::LogicalOr
                    }
                    else {
                        IdlSyntaxKind::BitwiseOr
                    }
                }
                '^' => {
                    state.advance(1);
                    IdlSyntaxKind::BitwiseXor
                }
                '~' => {
                    state.advance(1);
                    IdlSyntaxKind::BitwiseNot
                }
                '!' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        IdlSyntaxKind::NotEqual
                    }
                    else {
                        IdlSyntaxKind::LogicalNot
                    }
                }
                '=' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        IdlSyntaxKind::Equal
                    }
                    else {
                        IdlSyntaxKind::Assign
                    }
                }
                '<' => {
                    state.advance(1);
                    if state.peek() == Some('<') {
                        state.advance(1);
                        IdlSyntaxKind::LeftShift
                    }
                    else if state.peek() == Some('=') {
                        state.advance(1);
                        IdlSyntaxKind::LessEqual
                    }
                    else {
                        IdlSyntaxKind::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if state.peek() == Some('>') {
                        state.advance(1);
                        IdlSyntaxKind::RightShift
                    }
                    else if state.peek() == Some('=') {
                        state.advance(1);
                        IdlSyntaxKind::GreaterEqual
                    }
                    else {
                        IdlSyntaxKind::Greater
                    }
                }
                _ => return false,
            };

            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理分隔符
    fn lex_delimiters(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '(' => {
                    state.advance(1);
                    IdlSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    IdlSyntaxKind::RightParen
                }
                '[' => {
                    state.advance(1);
                    IdlSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    IdlSyntaxKind::RightBracket
                }
                '{' => {
                    state.advance(1);
                    IdlSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    IdlSyntaxKind::RightBrace
                }
                ';' => {
                    state.advance(1);
                    IdlSyntaxKind::Semicolon
                }
                ',' => {
                    state.advance(1);
                    IdlSyntaxKind::Comma
                }
                ':' => {
                    state.advance(1);
                    if state.peek() == Some(':') {
                        state.advance(1);
                        IdlSyntaxKind::DoubleColon
                    }
                    else {
                        IdlSyntaxKind::Colon
                    }
                }
                '.' => {
                    state.advance(1);
                    IdlSyntaxKind::Dot
                }
                _ => return false,
            };

            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<IdlLanguage> for IdlLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<IdlSyntaxKind> {
        let mut state = State::new(source);

        loop {
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state) {
                continue;
            }

            if self.lex_preprocessor(&mut state, source) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_char(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_operators(&mut state) {
                continue;
            }

            if self.lex_delimiters(&mut state) {
                continue;
            }

            // 如果没有匹配任何规则，标记为错误并前进一个字符
            if let Some(ch) = state.peek() {
                let start_pos = state.get_position();
                state.advance(ch.len_utf8());
                state.add_token(IdlSyntaxKind::Error, start_pos, state.get_position());
            }
            else {
                break;
            }
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(IdlSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}

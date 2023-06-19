use crate::{kind::ActionScriptSyntaxKind, language::ActionScriptLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, ActionScriptLanguage>;

static AS_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static AS_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["//"] });
static AS_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });
static AS_CHAR: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: Some('\\') });

#[derive(Clone)]
pub struct ActionScriptLexer<'config> {
    config: &'config ActionScriptLanguage,
}

impl<'config> Lexer<ActionScriptLanguage> for ActionScriptLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<ActionScriptLanguage>,
    ) -> LexOutput<ActionScriptLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> ActionScriptLexer<'config> {
    pub fn new(config: &'config ActionScriptLanguage) -> Self {
        Self { config }
    }

    /// 主要词法分析逻辑
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

            if self.lex_operator_or_delimiter(state) {
                continue;
            }

            state.safe_check(safe_point);
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(ActionScriptSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match AS_WHITESPACE.scan(state.rest(), state.get_position(), ActionScriptSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                return true;
            }
            None => {}
        }
        false
    }

    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();
        // line comment: // ... until newline
        if rest.starts_with("//") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(ActionScriptSyntaxKind::Comment, start, state.get_position());
            return true;
        }
        // block comment: /* ... */ with nesting support
        if rest.starts_with("/*") {
            state.advance(2);
            let mut depth = 1usize;
            while let Some(ch) = state.peek() {
                if ch == '/' && state.peek_next_n(1) == Some('*') {
                    state.advance(2);
                    depth += 1;
                    continue;
                }
                if ch == '*' && state.peek_next_n(1) == Some('/') {
                    state.advance(2);
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                    continue;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(ActionScriptSyntaxKind::Comment, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_number_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let first = match state.peek() {
            Some(c) => c,
            None => return false,
        };
        if !first.is_ascii_digit() {
            return false;
        }

        state.advance(first.len_utf8());
        while let Some(c) = state.peek() {
            if c.is_ascii_digit() || c == '_' {
                state.advance(c.len_utf8());
            }
            else {
                break;
            }
        }

        // fractional part
        if state.peek() == Some('.') {
            let n1 = state.peek_next_n(1);
            if n1.map(|c| c.is_ascii_digit()).unwrap_or(false) {
                state.advance(1); // consume '.'
                while let Some(c) = state.peek() {
                    if c.is_ascii_digit() || c == '_' {
                        state.advance(c.len_utf8());
                    }
                    else {
                        break;
                    }
                }
            }
        }
        // exponent
        if let Some(c) = state.peek() {
            if c == 'e' || c == 'E' {
                let n1 = state.peek_next_n(1);
                if n1 == Some('+') || n1 == Some('-') || n1.map(|d| d.is_ascii_digit()).unwrap_or(false) {
                    state.advance(1);
                    if let Some(sign) = state.peek() {
                        if sign == '+' || sign == '-' {
                            state.advance(1);
                        }
                    }
                    while let Some(d) = state.peek() {
                        if d.is_ascii_digit() || d == '_' {
                            state.advance(d.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                }
            }
        }
        let end = state.get_position();
        state.add_token(ActionScriptSyntaxKind::NumberLiteral, start, end);
        true
    }

    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let first = match state.peek() {
            Some(c) => c,
            None => return false,
        };
        if !first.is_ascii_alphabetic() && first != '_' && first != '$' {
            return false;
        }

        let mut buf = String::new();
        buf.push(first);
        state.advance(first.len_utf8());
        while let Some(c) = state.peek() {
            if c.is_ascii_alphanumeric() || c == '_' || c == '$' {
                buf.push(c);
                state.advance(c.len_utf8());
            }
            else {
                break;
            }
        }

        let end = state.get_position();
        let kind = match buf.as_str() {
            "as" => ActionScriptSyntaxKind::As,
            "break" => ActionScriptSyntaxKind::Break,
            "case" => ActionScriptSyntaxKind::Case,
            "catch" => ActionScriptSyntaxKind::Catch,
            "class" => ActionScriptSyntaxKind::Class,
            "const" => ActionScriptSyntaxKind::Const,
            "continue" => ActionScriptSyntaxKind::Continue,
            "default" => ActionScriptSyntaxKind::Default,
            "delete" => ActionScriptSyntaxKind::Delete,
            "do" => ActionScriptSyntaxKind::Do,
            "else" => ActionScriptSyntaxKind::Else,
            "extends" => ActionScriptSyntaxKind::Extends,
            "false" => ActionScriptSyntaxKind::False,
            "finally" => ActionScriptSyntaxKind::Finally,
            "for" => ActionScriptSyntaxKind::For,
            "function" => ActionScriptSyntaxKind::Function,
            "if" => ActionScriptSyntaxKind::If,
            "implements" => ActionScriptSyntaxKind::Implements,
            "import" => ActionScriptSyntaxKind::Import,
            "in" => ActionScriptSyntaxKind::In,
            "instanceof" => ActionScriptSyntaxKind::Instanceof,
            "interface" => ActionScriptSyntaxKind::Interface,
            "internal" => ActionScriptSyntaxKind::Internal,
            "is" => ActionScriptSyntaxKind::Is,
            "native" => ActionScriptSyntaxKind::Native,
            "new" => ActionScriptSyntaxKind::New,
            "null" => ActionScriptSyntaxKind::Null,
            "package" => ActionScriptSyntaxKind::Package,
            "private" => ActionScriptSyntaxKind::Private,
            "protected" => ActionScriptSyntaxKind::Protected,
            "public" => ActionScriptSyntaxKind::Public,
            "return" => ActionScriptSyntaxKind::Return,
            "static" => ActionScriptSyntaxKind::Static,
            "super" => ActionScriptSyntaxKind::Super,
            "switch" => ActionScriptSyntaxKind::Switch,
            "this" => ActionScriptSyntaxKind::This,
            "throw" => ActionScriptSyntaxKind::Throw,
            "true" => ActionScriptSyntaxKind::True,
            "try" => ActionScriptSyntaxKind::Try,
            "typeof" => ActionScriptSyntaxKind::Typeof,
            "use" => ActionScriptSyntaxKind::Use,
            "var" => ActionScriptSyntaxKind::Var,
            "void" => ActionScriptSyntaxKind::Void,
            "while" => ActionScriptSyntaxKind::While,
            "with" => ActionScriptSyntaxKind::With,
            "each" => ActionScriptSyntaxKind::Each,
            "get" => ActionScriptSyntaxKind::Get,
            "set" => ActionScriptSyntaxKind::Set,
            "namespace" => ActionScriptSyntaxKind::Namespace,
            "include" => ActionScriptSyntaxKind::Include,
            "dynamic" => ActionScriptSyntaxKind::Dynamic,
            "final" => ActionScriptSyntaxKind::Final,
            "override" => ActionScriptSyntaxKind::Override,
            "Array" => ActionScriptSyntaxKind::Array,
            "Boolean" => ActionScriptSyntaxKind::Boolean,
            "Date" => ActionScriptSyntaxKind::Date,
            "Error" => ActionScriptSyntaxKind::Error,
            "Function" => ActionScriptSyntaxKind::Function_,
            "Number" => ActionScriptSyntaxKind::Number,
            "Object" => ActionScriptSyntaxKind::Object,
            "RegExp" => ActionScriptSyntaxKind::RegExp,
            "String" => ActionScriptSyntaxKind::String_,
            "uint" => ActionScriptSyntaxKind::Uint,
            "Vector" => ActionScriptSyntaxKind::Vector,
            "XML" => ActionScriptSyntaxKind::Xml,
            "XMLList" => ActionScriptSyntaxKind::XmlList,
            _ => ActionScriptSyntaxKind::Identifier,
        };

        state.add_token(kind, start, end);
        true
    }

    fn lex_operator_or_delimiter<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let first = match state.peek() {
            Some(c) => c,
            None => return false,
        };

        let kind = match first {
            '+' => {
                state.advance(1);
                match state.peek() {
                    Some('=') => {
                        state.advance(1);
                        ActionScriptSyntaxKind::PlusAssign
                    }
                    Some('+') => {
                        state.advance(1);
                        ActionScriptSyntaxKind::Increment
                    }
                    _ => ActionScriptSyntaxKind::Plus,
                }
            }
            '-' => {
                state.advance(1);
                match state.peek() {
                    Some('=') => {
                        state.advance(1);
                        ActionScriptSyntaxKind::MinusAssign
                    }
                    Some('-') => {
                        state.advance(1);
                        ActionScriptSyntaxKind::Decrement
                    }
                    Some('>') => {
                        state.advance(1);
                        ActionScriptSyntaxKind::Arrow
                    }
                    _ => ActionScriptSyntaxKind::Minus,
                }
            }
            '*' => {
                state.advance(1);
                if state.peek() == Some('=') {
                    state.advance(1);
                    ActionScriptSyntaxKind::StarAssign
                }
                else {
                    ActionScriptSyntaxKind::Star
                }
            }
            '/' => {
                state.advance(1);
                if state.peek() == Some('=') {
                    state.advance(1);
                    ActionScriptSyntaxKind::SlashAssign
                }
                else {
                    ActionScriptSyntaxKind::Slash
                }
            }
            '%' => {
                state.advance(1);
                if state.peek() == Some('=') {
                    state.advance(1);
                    ActionScriptSyntaxKind::PercentAssign
                }
                else {
                    ActionScriptSyntaxKind::Percent
                }
            }
            '=' => {
                state.advance(1);
                match state.peek() {
                    Some('=') => {
                        state.advance(1);
                        if state.peek() == Some('=') {
                            state.advance(1);
                            ActionScriptSyntaxKind::EqualEqualEqual
                        }
                        else {
                            ActionScriptSyntaxKind::EqualEqual
                        }
                    }
                    _ => ActionScriptSyntaxKind::Equal,
                }
            }
            '!' => {
                state.advance(1);
                match state.peek() {
                    Some('=') => {
                        state.advance(1);
                        if state.peek() == Some('=') {
                            state.advance(1);
                            ActionScriptSyntaxKind::NotEqualEqual
                        }
                        else {
                            ActionScriptSyntaxKind::NotEqual
                        }
                    }
                    _ => ActionScriptSyntaxKind::LogicalNot,
                }
            }
            '<' => {
                state.advance(1);
                match state.peek() {
                    Some('=') => {
                        state.advance(1);
                        ActionScriptSyntaxKind::LessEqual
                    }
                    Some('<') => {
                        state.advance(1);
                        if state.peek() == Some('=') {
                            state.advance(1);
                            ActionScriptSyntaxKind::LeftShiftAssign
                        }
                        else {
                            ActionScriptSyntaxKind::LeftShift
                        }
                    }
                    _ => ActionScriptSyntaxKind::LessThan,
                }
            }
            '>' => {
                state.advance(1);
                match state.peek() {
                    Some('=') => {
                        state.advance(1);
                        ActionScriptSyntaxKind::GreaterEqual
                    }
                    Some('>') => {
                        state.advance(1);
                        match state.peek() {
                            Some('>') => {
                                state.advance(1);
                                if state.peek() == Some('=') {
                                    state.advance(1);
                                    ActionScriptSyntaxKind::UnsignedRightShiftAssign
                                }
                                else {
                                    ActionScriptSyntaxKind::UnsignedRightShift
                                }
                            }
                            Some('=') => {
                                state.advance(1);
                                ActionScriptSyntaxKind::RightShiftAssign
                            }
                            _ => ActionScriptSyntaxKind::RightShift,
                        }
                    }
                    _ => ActionScriptSyntaxKind::GreaterThan,
                }
            }
            '&' => {
                state.advance(1);
                match state.peek() {
                    Some('&') => {
                        state.advance(1);
                        ActionScriptSyntaxKind::LogicalAnd
                    }
                    Some('=') => {
                        state.advance(1);
                        ActionScriptSyntaxKind::BitwiseAndAssign
                    }
                    _ => ActionScriptSyntaxKind::BitwiseAnd,
                }
            }
            '|' => {
                state.advance(1);
                match state.peek() {
                    Some('|') => {
                        state.advance(1);
                        ActionScriptSyntaxKind::LogicalOr
                    }
                    Some('=') => {
                        state.advance(1);
                        ActionScriptSyntaxKind::BitwiseOrAssign
                    }
                    _ => ActionScriptSyntaxKind::BitwiseOr,
                }
            }
            '^' => {
                state.advance(1);
                if state.peek() == Some('=') {
                    state.advance(1);
                    ActionScriptSyntaxKind::BitwiseXorAssign
                }
                else {
                    ActionScriptSyntaxKind::BitwiseXor
                }
            }
            '~' => {
                state.advance(1);
                ActionScriptSyntaxKind::BitwiseNot
            }
            '?' => {
                state.advance(1);
                ActionScriptSyntaxKind::Question
            }
            ':' => {
                state.advance(1);
                ActionScriptSyntaxKind::Colon
            }
            '(' => {
                state.advance(1);
                ActionScriptSyntaxKind::LeftParen
            }
            ')' => {
                state.advance(1);
                ActionScriptSyntaxKind::RightParen
            }
            '{' => {
                state.advance(1);
                ActionScriptSyntaxKind::LeftBrace
            }
            '}' => {
                state.advance(1);
                ActionScriptSyntaxKind::RightBrace
            }
            '[' => {
                state.advance(1);
                ActionScriptSyntaxKind::LeftBracket
            }
            ']' => {
                state.advance(1);
                ActionScriptSyntaxKind::RightBracket
            }
            ';' => {
                state.advance(1);
                ActionScriptSyntaxKind::Semicolon
            }
            ',' => {
                state.advance(1);
                ActionScriptSyntaxKind::Comma
            }
            '.' => {
                state.advance(1);
                ActionScriptSyntaxKind::Dot
            }
            '@' => {
                state.advance(1);
                ActionScriptSyntaxKind::At
            }
            '#' => {
                state.advance(1);
                ActionScriptSyntaxKind::Hash
            }
            '$' => {
                state.advance(1);
                ActionScriptSyntaxKind::Dollar
            }
            '\\' => {
                state.advance(1);
                ActionScriptSyntaxKind::Backslash
            }
            '\'' => {
                state.advance(1);
                ActionScriptSyntaxKind::Quote
            }
            '"' => {
                state.advance(1);
                ActionScriptSyntaxKind::DoubleQuote
            }
            '`' => {
                state.advance(1);
                ActionScriptSyntaxKind::Backtick
            }
            _ => return false,
        };

        let end = state.get_position();
        state.add_token(kind, start, end);
        true
    }

    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        // normal string: "..." or '...'
        if state.current() == Some('"') || state.current() == Some('\'') {
            let quote_char = state.current().unwrap();
            state.advance(1);
            let mut escaped = false;
            while let Some(ch) = state.peek() {
                if ch == quote_char && !escaped {
                    state.advance(1); // consume closing quote
                    break;
                }
                state.advance(ch.len_utf8());
                if escaped {
                    escaped = false;
                    continue;
                }
                if ch == '\\' {
                    escaped = true;
                    continue;
                }
                if ch == '\n' || ch == '\r' {
                    break;
                }
            }
            state.add_token(ActionScriptSyntaxKind::StringLiteral, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_char_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        if state.peek() != Some('\'') {
            return false;
        }

        state.advance(1); // consume opening quote
        if let Some('\\') = state.peek() {
            state.advance(1); // consume backslash
            if let Some(escaped) = state.peek() {
                state.advance(escaped.len_utf8()); // consume escaped character
            }
        }
        else if let Some(ch) = state.peek() {
            state.advance(ch.len_utf8()); // consume character
        }

        if state.peek() == Some('\'') {
            state.advance(1); // consume closing quote
            state.add_token(ActionScriptSyntaxKind::CharLiteral, start, state.get_position());
            return true;
        }

        // Reset position if not a valid char literal
        state.set_position(start);
        false
    }
}

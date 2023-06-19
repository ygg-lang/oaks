use crate::{kind::GroovySyntaxKind, language::GroovyLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, GroovyLanguage>;

static GROOVY_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static GROOVY_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["//"] });
static GROOVY_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });
static GROOVY_CHAR: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: Some('\\') });

#[derive(Clone)]
pub struct GroovyLexer<'config> {
    config: &'config GroovyLanguage,
}

impl<'config> Lexer<GroovyLanguage> for GroovyLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<GroovyLanguage>,
    ) -> LexOutput<GroovyLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> GroovyLexer<'config> {
    pub fn new(config: &'config GroovyLanguage) -> Self {
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

            state.safe_check(safe_point);
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(GroovySyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match GROOVY_WHITESPACE.scan(state.rest(), state.get_position(), GroovySyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    /// 跳过注释
    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        // 行注释 //
        if let Some(token) = GROOVY_COMMENT.scan(state.rest(), state.get_position(), GroovySyntaxKind::Comment) {
            state.advance_with(token);
            return true;
        }

        // 块注释 /* ... */
        if state.rest().starts_with("/*") {
            let start = state.get_position();
            state.advance(2); // 跳过 /*

            while state.not_at_end() {
                if state.rest().starts_with("*/") {
                    state.advance(2); // 跳过 */
                    break;
                }
                if let Some(ch) = state.peek() {
                    state.advance(ch.len_utf8());
                }
            }

            let end = state.get_position();
            state.add_token(GroovySyntaxKind::Comment, start, end);
            return true;
        }

        false
    }

    /// 词法分析字符串字面量
    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        // 普通字符串 "..."
        if let Some(token) = GROOVY_STRING.scan(state.rest(), state.get_position(), GroovySyntaxKind::StringLiteral) {
            state.advance_with(token);
            return true;
        }

        // 三重引号字符串 """..."""
        if state.rest().starts_with("\"\"\"") {
            let start = state.get_position();
            state.advance(3); // 跳过开始的 """

            while state.not_at_end() {
                if state.rest().starts_with("\"\"\"") {
                    state.advance(3); // 跳过结束的 """
                    break;
                }
                if let Some(ch) = state.peek() {
                    state.advance(ch.len_utf8());
                }
            }

            let end = state.get_position();
            state.add_token(GroovySyntaxKind::StringLiteral, start, end);
            return true;
        }

        // GString $"..." 或 $/.../$
        if state.rest().starts_with("$/") {
            let start = state.get_position();
            state.advance(2); // 跳过 $/

            while state.not_at_end() {
                if state.rest().starts_with("/$") {
                    state.advance(2); // 跳过 /$
                    break;
                }
                if let Some(ch) = state.peek() {
                    state.advance(ch.len_utf8());
                }
            }

            let end = state.get_position();
            state.add_token(GroovySyntaxKind::StringLiteral, start, end);
            return true;
        }

        false
    }

    /// 词法分析字符字面量
    fn lex_char_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        match GROOVY_CHAR.scan(state.rest(), state.get_position(), GroovySyntaxKind::CharLiteral) {
            Some(token) => {
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    /// 词法分析数字字面量
    fn lex_number_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let mut has_digits = false;
        let mut is_float = false;

        // 处理负号
        if state.rest().starts_with('-') {
            state.advance(1);
        }

        // 处理十六进制 0x...
        if state.rest().starts_with("0x") || state.rest().starts_with("0X") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch.is_ascii_hexdigit() {
                    state.advance(ch.len_utf8());
                    has_digits = true;
                }
                else {
                    break;
                }
            }
        }
        // 处理八进制 0...
        else if state.rest().starts_with('0') && state.rest().len() > 1 {
            if let Some(next_ch) = state.rest().chars().nth(1) {
                if next_ch.is_ascii_digit() {
                    state.advance(1); // 跳过 0
                    while let Some(ch) = state.peek() {
                        if ch >= '0' && ch <= '7' {
                            state.advance(ch.len_utf8());
                            has_digits = true;
                        }
                        else {
                            break;
                        }
                    }
                }
            }
        }
        // 处理十进制
        else {
            // 处理整数部分
            while let Some(ch) = state.peek() {
                if ch.is_ascii_digit() {
                    state.advance(ch.len_utf8());
                    has_digits = true;
                }
                else {
                    break;
                }
            }

            // 处理小数部分
            if state.rest().starts_with('.') && has_digits {
                if let Some(next_ch) = state.rest().chars().nth(1) {
                    if next_ch.is_ascii_digit() {
                        state.advance(1); // 跳过 .
                        is_float = true;

                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() {
                                state.advance(ch.len_utf8());
                            }
                            else {
                                break;
                            }
                        }
                    }
                }
            }

            // 处理指数部分
            if (state.rest().starts_with('e') || state.rest().starts_with('E')) && has_digits {
                state.advance(1);
                is_float = true;

                // 处理指数符号
                if state.rest().starts_with('+') || state.rest().starts_with('-') {
                    state.advance(1);
                }

                // 处理指数数字
                let mut exp_digits = false;
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(ch.len_utf8());
                        exp_digits = true;
                    }
                    else {
                        break;
                    }
                }

                if !exp_digits {
                    // 指数部分必须有数字
                    return false;
                }
            }
        }

        // 处理数字后缀 (G, L, F, D)
        if has_digits {
            if let Some(ch) = state.peek() {
                if matches!(ch, 'G' | 'g' | 'L' | 'l' | 'F' | 'f' | 'D' | 'd') {
                    state.advance(ch.len_utf8());
                    is_float = matches!(ch, 'F' | 'f' | 'D' | 'd' | 'G' | 'g');
                }
            }
        }

        if has_digits {
            let end = state.get_position();
            let kind = if is_float { GroovySyntaxKind::FloatLiteral } else { GroovySyntaxKind::IntLiteral };
            state.add_token(kind, start, end);
            true
        }
        else {
            false
        }
    }

    /// 词法分析标识符或关键字
    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        // 标识符必须以字母或下划线开始
        if let Some(first_ch) = state.peek() {
            if !first_ch.is_alphabetic() && first_ch != '_' {
                return false;
            }

            state.advance(first_ch.len_utf8());

            // 后续字符可以是字母、数字或下划线
            while let Some(ch) = state.peek() {
                if ch.is_alphanumeric() || ch == '_' {
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }

            let end = state.get_position();
            let text = state.get_text_in((start..end).into());
            let kind = self.keyword_or_identifier(&text);
            state.add_token(kind, start, end);
            true
        }
        else {
            false
        }
    }

    /// 判断是关键字还是标识符
    fn keyword_or_identifier(&self, text: &str) -> GroovySyntaxKind {
        match text {
            // 关键字
            "abstract" => GroovySyntaxKind::AbstractKeyword,
            "as" => GroovySyntaxKind::AsKeyword,
            "assert" => GroovySyntaxKind::AssertKeyword,
            "break" => GroovySyntaxKind::BreakKeyword,
            "case" => GroovySyntaxKind::CaseKeyword,
            "catch" => GroovySyntaxKind::CatchKeyword,
            "class" => GroovySyntaxKind::ClassKeyword,
            "const" => GroovySyntaxKind::ConstKeyword,
            "continue" => GroovySyntaxKind::ContinueKeyword,
            "def" => GroovySyntaxKind::DefKeyword,
            "default" => GroovySyntaxKind::DefaultKeyword,
            "do" => GroovySyntaxKind::DoKeyword,
            "else" => GroovySyntaxKind::ElseKeyword,
            "enum" => GroovySyntaxKind::EnumKeyword,
            "extends" => GroovySyntaxKind::ExtendsKeyword,
            "final" => GroovySyntaxKind::FinalKeyword,
            "finally" => GroovySyntaxKind::FinallyKeyword,
            "for" => GroovySyntaxKind::ForKeyword,
            "goto" => GroovySyntaxKind::GotoKeyword,
            "if" => GroovySyntaxKind::IfKeyword,
            "implements" => GroovySyntaxKind::ImplementsKeyword,
            "import" => GroovySyntaxKind::ImportKeyword,
            "in" => GroovySyntaxKind::InKeyword,
            "instanceof" => GroovySyntaxKind::InstanceofKeyword,
            "interface" => GroovySyntaxKind::InterfaceKeyword,
            "native" => GroovySyntaxKind::NativeKeyword,
            "new" => GroovySyntaxKind::NewKeyword,
            "package" => GroovySyntaxKind::PackageKeyword,
            "private" => GroovySyntaxKind::PrivateKeyword,
            "protected" => GroovySyntaxKind::ProtectedKeyword,
            "public" => GroovySyntaxKind::PublicKeyword,
            "return" => GroovySyntaxKind::ReturnKeyword,
            "static" => GroovySyntaxKind::StaticKeyword,
            "strictfp" => GroovySyntaxKind::StrictfpKeyword,
            "super" => GroovySyntaxKind::SuperKeyword,
            "switch" => GroovySyntaxKind::SwitchKeyword,
            "synchronized" => GroovySyntaxKind::SynchronizedKeyword,
            "this" => GroovySyntaxKind::ThisKeyword,
            "throw" => GroovySyntaxKind::ThrowKeyword,
            "throws" => GroovySyntaxKind::ThrowsKeyword,
            "trait" => GroovySyntaxKind::TraitKeyword,
            "transient" => GroovySyntaxKind::TransientKeyword,
            "try" => GroovySyntaxKind::TryKeyword,
            "void" => GroovySyntaxKind::VoidKeyword,
            "volatile" => GroovySyntaxKind::VolatileKeyword,
            "while" => GroovySyntaxKind::WhileKeyword,

            // 特殊字面量
            "true" | "false" => GroovySyntaxKind::BooleanLiteral,
            "null" => GroovySyntaxKind::NullLiteral,

            // 默认为标识符
            _ => GroovySyntaxKind::Identifier,
        }
    }

    /// 词法分析操作符
    fn lex_operators<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // 三字符操作符
        if rest.starts_with(">>>") {
            state.advance(3);
            state.add_token(GroovySyntaxKind::UnsignedRightShift, start, state.get_position());
            return true;
        }
        if rest.starts_with("<=>") {
            state.advance(3);
            state.add_token(GroovySyntaxKind::Spaceship, start, state.get_position());
            return true;
        }

        // 两字符操作符
        if rest.starts_with("**") {
            state.advance(2);
            state.add_token(GroovySyntaxKind::Power, start, state.get_position());
            return true;
        }
        if rest.starts_with("+=") {
            state.advance(2);
            state.add_token(GroovySyntaxKind::PlusAssign, start, state.get_position());
            return true;
        }
        if rest.starts_with("-=") {
            state.advance(2);
            state.add_token(GroovySyntaxKind::MinusAssign, start, state.get_position());
            return true;
        }
        if rest.starts_with("*=") {
            state.advance(2);
            state.add_token(GroovySyntaxKind::StarAssign, start, state.get_position());
            return true;
        }
        if rest.starts_with("/=") {
            state.advance(2);
            state.add_token(GroovySyntaxKind::SlashAssign, start, state.get_position());
            return true;
        }
        if rest.starts_with("%=") {
            state.advance(2);
            state.add_token(GroovySyntaxKind::PercentAssign, start, state.get_position());
            return true;
        }
        if rest.starts_with("**=") {
            state.advance(3);
            state.add_token(GroovySyntaxKind::PowerAssign, start, state.get_position());
            return true;
        }
        if rest.starts_with("==") {
            state.advance(2);
            state.add_token(GroovySyntaxKind::Equal, start, state.get_position());
            return true;
        }
        if rest.starts_with("!=") {
            state.advance(2);
            state.add_token(GroovySyntaxKind::NotEqual, start, state.get_position());
            return true;
        }
        if rest.starts_with("<=") {
            state.advance(2);
            state.add_token(GroovySyntaxKind::LessEqual, start, state.get_position());
            return true;
        }
        if rest.starts_with(">=") {
            state.advance(2);
            state.add_token(GroovySyntaxKind::GreaterEqual, start, state.get_position());
            return true;
        }
        if rest.starts_with("&&") {
            state.advance(2);
            state.add_token(GroovySyntaxKind::LogicalAnd, start, state.get_position());
            return true;
        }
        if rest.starts_with("||") {
            state.advance(2);
            state.add_token(GroovySyntaxKind::LogicalOr, start, state.get_position());
            return true;
        }
        if rest.starts_with("<<") {
            state.advance(2);
            state.add_token(GroovySyntaxKind::LeftShift, start, state.get_position());
            return true;
        }
        if rest.starts_with(">>") {
            state.advance(2);
            state.add_token(GroovySyntaxKind::RightShift, start, state.get_position());
            return true;
        }
        if rest.starts_with("++") {
            state.advance(2);
            state.add_token(GroovySyntaxKind::Increment, start, state.get_position());
            return true;
        }
        if rest.starts_with("--") {
            state.advance(2);
            state.add_token(GroovySyntaxKind::Decrement, start, state.get_position());
            return true;
        }
        if rest.starts_with("?:") {
            state.advance(2);
            state.add_token(GroovySyntaxKind::Elvis, start, state.get_position());
            return true;
        }
        if rest.starts_with("?.") {
            state.advance(2);
            state.add_token(GroovySyntaxKind::SafeNavigation, start, state.get_position());
            return true;
        }

        false
    }

    /// 词法分析单字符 token
    fn lex_single_char_tokens<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.peek() {
            let start = state.get_position();
            let kind = match ch {
                '+' => Some(GroovySyntaxKind::Plus),
                '-' => Some(GroovySyntaxKind::Minus),
                '*' => Some(GroovySyntaxKind::Star),
                '/' => Some(GroovySyntaxKind::Slash),
                '%' => Some(GroovySyntaxKind::Percent),
                '=' => Some(GroovySyntaxKind::Assign),
                '<' => Some(GroovySyntaxKind::Less),
                '>' => Some(GroovySyntaxKind::Greater),
                '!' => Some(GroovySyntaxKind::LogicalNot),
                '&' => Some(GroovySyntaxKind::BitAnd),
                '|' => Some(GroovySyntaxKind::BitOr),
                '^' => Some(GroovySyntaxKind::BitXor),
                '~' => Some(GroovySyntaxKind::BitNot),
                '?' => Some(GroovySyntaxKind::Question),
                ':' => Some(GroovySyntaxKind::Colon),
                '(' => Some(GroovySyntaxKind::LeftParen),
                ')' => Some(GroovySyntaxKind::RightParen),
                '[' => Some(GroovySyntaxKind::LeftBracket),
                ']' => Some(GroovySyntaxKind::RightBracket),
                '{' => Some(GroovySyntaxKind::LeftBrace),
                '}' => Some(GroovySyntaxKind::RightBrace),
                ',' => Some(GroovySyntaxKind::Comma),
                '.' => Some(GroovySyntaxKind::Period),
                ';' => Some(GroovySyntaxKind::Semicolon),
                '@' => Some(GroovySyntaxKind::At),
                _ => None,
            };

            if let Some(token_kind) = kind {
                state.advance(ch.len_utf8());
                let end = state.get_position();
                state.add_token(token_kind, start, end);
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
}

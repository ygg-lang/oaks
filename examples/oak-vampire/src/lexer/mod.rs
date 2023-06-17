use crate::{kind::VampireSyntaxKind, language::VampireLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, VampireLanguage>;

pub struct VampireLexer<'config> {
    config: &'config VampireLanguage,
}

impl<'config> VampireLexer<'config> {
    pub fn new(config: &'config VampireLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn skip_whitespace(&self, state: &mut State) -> bool {
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
            state.add_token(VampireSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(VampireSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(VampireSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        // 行注%
        if let Some('%') = state.peek() {
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(VampireSyntaxKind::LineComment, start_pos, state.get_position());
            return true;
        }

        // 块注
        // ...
        if let Some('/') = state.peek() {
            if let Some('*') = source.get_char_at(start_pos + 1) {
                state.advance(2);
                let mut depth = 1;

                while depth > 0 && state.not_at_end() {
                    if let Some('/') = state.peek() {
                        if let Some('*') = source.get_char_at(state.get_position() + 1) {
                            state.advance(2);
                            depth += 1;
                            continue;
                        }
                    }
                    if let Some('*') = state.peek() {
                        if let Some('/') = source.get_char_at(state.get_position() + 1) {
                            state.advance(2);
                            depth -= 1;
                            continue;
                        }
                    }
                    if let Some(ch) = state.peek() {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(VampireSyntaxKind::BlockComment, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理字符串字面量
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);
                let mut escaped = false;

                while let Some(ch) = state.peek() {
                    if escaped {
                        escaped = false;
                        state.advance(ch.len_utf8());
                    }
                    else if ch == '\\' {
                        escaped = true;
                        state.advance(1);
                    }
                    else if ch == quote {
                        state.advance(1);
                        state.add_token(VampireSyntaxKind::StringLiteral, start_pos, state.get_position());
                        return true;
                    }
                    else if ch == '\n' || ch == '\r' {
                        break; // 字符串不能跨
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                // 未闭合的字符

                state.add_token(VampireSyntaxKind::Error, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理数字字面

    fn lex_number(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() || (ch == '-' && source.get_char_at(start_pos + 1).map_or(false, |c| c.is_ascii_digit())) {
                // 处理负号
                if ch == '-' {
                    state.advance(1);
                }

                // 整数部分
                while let Some(digit) = state.peek() {
                    if digit.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // 检查是否是浮点

                if let Some('.') = state.peek() {
                    if let Some(next_ch) = source.get_char_at(state.get_position() + 1) {
                        if next_ch.is_ascii_digit() {
                            state.advance(1); // 跳过 '.'
                            while let Some(digit) = state.peek() {
                                if digit.is_ascii_digit() {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }

                            // 检查科学计数法
                            if let Some(e) = state.peek() {
                                if e == 'e' || e == 'E' {
                                    state.advance(1);
                                    if let Some(sign) = state.peek() {
                                        if sign == '+' || sign == '-' {
                                            state.advance(1);
                                        }
                                    }
                                    while let Some(digit) = state.peek() {
                                        if digit.is_ascii_digit() {
                                            state.advance(1);
                                        }
                                        else {
                                            break;
                                        }
                                    }
                                }
                            }

                            state.add_token(VampireSyntaxKind::RealLiteral, start_pos, state.get_position());
                            return true;
                        }
                    }
                }

                state.add_token(VampireSyntaxKind::IntegerLiteral, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理标识符和关键

    fn lex_ident_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' || ch == '$' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 检查是否是关键

                if let Some(text) = source.get_text_in((start_pos..state.get_position()).into()) {
                    let token_kind = match text {
                        // Vampire 语法关键
                        "fof" => VampireSyntaxKind::FofKw,
                        "cnf" => VampireSyntaxKind::CnfKw,
                        "tff" => VampireSyntaxKind::TffKw,
                        "thf" => VampireSyntaxKind::ThfKw,
                        "tpi" => VampireSyntaxKind::TpiKw,
                        "include" => VampireSyntaxKind::IncludeKw,
                        "axiom" => VampireSyntaxKind::AxiomKw,
                        "hypothesis" => VampireSyntaxKind::HypothesisKw,
                        "definition" => VampireSyntaxKind::DefinitionKw,
                        "assumption" => VampireSyntaxKind::AssumptionKw,
                        "lemma" => VampireSyntaxKind::LemmaKw,
                        "theorem" => VampireSyntaxKind::TheoremKw,
                        "conjecture" => VampireSyntaxKind::ConjectureKw,
                        "negated_conjecture" => VampireSyntaxKind::NegatedConjectureKw,
                        "plain" => VampireSyntaxKind::PlainKw,
                        "type" => VampireSyntaxKind::TypeKw,
                        "fi_domain" => VampireSyntaxKind::FiDomainKw,
                        "fi_functors" => VampireSyntaxKind::FiFunctorsKw,
                        "fi_predicates" => VampireSyntaxKind::FiPredicatesKw,
                        "unknown" => VampireSyntaxKind::UnknownKw,

                        // 基本类型
                        "$o" => VampireSyntaxKind::BoolKw,
                        "$int" => VampireSyntaxKind::IntKw,
                        "$real" => VampireSyntaxKind::RealKw,
                        "$rat" => VampireSyntaxKind::RatKw,
                        "$i" => VampireSyntaxKind::IndividualKw,
                        "$tType" => VampireSyntaxKind::TTypeKw,
                        "$oType" => VampireSyntaxKind::OTypeKw,
                        "$iType" => VampireSyntaxKind::ITypeKw,

                        // 布尔字面
                        "$true" | "$false" => VampireSyntaxKind::BoolLiteral,

                        _ => VampireSyntaxKind::Identifier,
                    };

                    state.add_token(token_kind, start_pos, state.get_position());
                    return true;
                }
                else {
                    state.add_token(VampireSyntaxKind::Identifier, start_pos, state.get_position());
                    return true;
                }
            }
        }

        false
    }

    /// 处理标点符号和操作符
    fn lex_punctuation(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => {
                    state.advance(1);
                    VampireSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    VampireSyntaxKind::RightParen
                }
                '[' => {
                    state.advance(1);
                    VampireSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    VampireSyntaxKind::RightBracket
                }
                '{' => {
                    state.advance(1);
                    VampireSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    VampireSyntaxKind::RightBrace
                }
                ':' => {
                    state.advance(1);
                    VampireSyntaxKind::Colon
                }
                ';' => {
                    state.advance(1);
                    VampireSyntaxKind::Semicolon
                }
                '.' => {
                    state.advance(1);
                    VampireSyntaxKind::Dot
                }
                ',' => {
                    state.advance(1);
                    VampireSyntaxKind::Comma
                }
                '?' => {
                    state.advance(1);
                    VampireSyntaxKind::Question
                }
                '!' => {
                    if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        VampireSyntaxKind::NotEq
                    }
                    else {
                        state.advance(1);
                        VampireSyntaxKind::Bang
                    }
                }
                '@' => {
                    state.advance(1);
                    VampireSyntaxKind::At
                }
                '#' => {
                    state.advance(1);
                    VampireSyntaxKind::Hash
                }
                '$' => {
                    state.advance(1);
                    VampireSyntaxKind::Dollar
                }
                '%' => {
                    if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        VampireSyntaxKind::PercentEq
                    }
                    else {
                        state.advance(1);
                        VampireSyntaxKind::Percent
                    }
                }
                '^' => {
                    state.advance(1);
                    VampireSyntaxKind::Caret
                }
                '&' => {
                    if let Some('&') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        VampireSyntaxKind::AndAnd
                    }
                    else {
                        state.advance(1);
                        VampireSyntaxKind::Ampersand
                    }
                }
                '*' => {
                    if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        VampireSyntaxKind::StarEq
                    }
                    else {
                        state.advance(1);
                        VampireSyntaxKind::Star
                    }
                }
                '+' => {
                    if let Some('+') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        VampireSyntaxKind::PlusPlus
                    }
                    else if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        VampireSyntaxKind::PlusEq
                    }
                    else {
                        state.advance(1);
                        VampireSyntaxKind::Plus
                    }
                }
                '-' => {
                    if let Some('-') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        VampireSyntaxKind::MinusMinus
                    }
                    else if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        VampireSyntaxKind::MinusEq
                    }
                    else if let Some('>') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        VampireSyntaxKind::Arrow
                    }
                    else {
                        state.advance(1);
                        VampireSyntaxKind::Minus
                    }
                }
                '=' => {
                    if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        VampireSyntaxKind::EqEq
                    }
                    else if let Some('>') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        VampireSyntaxKind::ImpliesKw
                    }
                    else {
                        state.advance(1);
                        VampireSyntaxKind::Eq
                    }
                }
                '<' => {
                    if let Some('<') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        VampireSyntaxKind::LeftShift
                    }
                    else if let Some('=') = source.get_char_at(start_pos + 1) {
                        if let Some('>') = source.get_char_at(start_pos + 2) {
                            state.advance(3);
                            VampireSyntaxKind::IffKw
                        }
                        else {
                            state.advance(2);
                            VampireSyntaxKind::LessEq
                        }
                    }
                    else if let Some('~') = source.get_char_at(start_pos + 1) {
                        if let Some('>') = source.get_char_at(start_pos + 2) {
                            state.advance(3);
                            VampireSyntaxKind::XorKw
                        }
                        else {
                            state.advance(1);
                            VampireSyntaxKind::LessThan
                        }
                    }
                    else {
                        state.advance(1);
                        VampireSyntaxKind::LessThan
                    }
                }
                '>' => {
                    if let Some('>') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        VampireSyntaxKind::RightShift
                    }
                    else if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        VampireSyntaxKind::GreaterEq
                    }
                    else {
                        state.advance(1);
                        VampireSyntaxKind::GreaterThan
                    }
                }
                '/' => {
                    if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        VampireSyntaxKind::SlashEq
                    }
                    else {
                        state.advance(1);
                        VampireSyntaxKind::Slash
                    }
                }
                '\\' => {
                    state.advance(1);
                    VampireSyntaxKind::Backslash
                }
                '|' => {
                    if let Some('|') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        VampireSyntaxKind::OrOr
                    }
                    else {
                        state.advance(1);
                        VampireSyntaxKind::Pipe
                    }
                }
                '~' => {
                    if let Some('|') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        VampireSyntaxKind::NorKw
                    }
                    else if let Some('&') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        VampireSyntaxKind::NandKw
                    }
                    else {
                        state.advance(1);
                        VampireSyntaxKind::Tilde
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

    /// 处理普通文

    fn lex_text(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            // 遇到特殊字符时停

            match ch {
                ' ' | '\t' | '\n' | '\r' | '(' | ')' | '[' | ']' | '{' | '}' | ':' | ';' | '.' | ',' | '?' | '!' | '@'
                | '#' | '$' | '%' | '^' | '&' | '*' | '+' | '-' | '=' | '<' | '>' | '/' | '\\' | '|' | '~' | '"' | '\'' => {
                    break;
                }
                _ => {
                    if ch.is_ascii_alphabetic() || ch.is_ascii_digit() || ch == '_' {
                        break; // 这些应该由其他规则处
                    }
                    state.advance(ch.len_utf8());
                }
            }
        }

        if state.get_position() > start_pos {
            state.add_token(VampireSyntaxKind::Text, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<VampireLanguage> for VampireLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<VampireSyntaxKind> {
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

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_number(&mut state, source) {
                continue;
            }

            if self.lex_ident_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_punctuation(&mut state, source) {
                continue;
            }

            if self.lex_text(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(VampireSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(VampireSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}

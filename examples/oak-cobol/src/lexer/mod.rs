use crate::{kind::CobolSyntaxKind, language::CobolLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, CobolLanguage>;

pub struct CobolLexer<'config> {
    config: &'config CobolLanguage,
}

impl<'config> CobolLexer<'config> {
    pub fn new(config: &'config CobolLanguage) -> Self {
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
            state.add_token(CobolSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(CobolSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(CobolSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释 (COBOL 注释* 开头在
    fn lex_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('*') = state.peek() {
            // COBOL 注释通常在第7列开            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(CobolSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符串字面量
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);
                let mut found_end = false;

                while let Some(ch) = state.peek() {
                    state.advance(ch.len_utf8());

                    if ch == quote {
                        // 检查是否是双引号转义
                        if let Some(next_ch) = state.peek() {
                            if next_ch == quote {
                                state.advance(1); // 跳过转义的引号
                                continue;
                            }
                        }
                        found_end = true;
                        break;
                    }
                    else if ch == '\n' || ch == '\r' {
                        // 未闭合的字符串
                        break;
                    }
                }

                state.add_token(CobolSyntaxKind::StringLiteral, start_pos, state.get_position());
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

    /// 处理数字字面量
    fn lex_number(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() || ch == '.' || ch == '+' || ch == '-' {
                let mut has_dot = ch == '.';
                let mut has_sign = ch == '+' || ch == '-';

                if has_sign {
                    state.advance(1);
                    // 符号后必须跟数字
                    if let Some(next_ch) = state.peek() {
                        if !next_ch.is_ascii_digit() && next_ch != '.' {
                            state.set_position(start_pos);
                            return false;
                        }
                    }
                    else {
                        state.set_position(start_pos);
                        return false;
                    }
                }

                if has_dot && !has_sign {
                    state.advance(1);
                    // 确保点后面有数字
                    if let Some(next_ch) = state.peek() {
                        if !next_ch.is_ascii_digit() {
                            state.set_position(start_pos);
                            return false;
                        }
                    }
                    else {
                        state.set_position(start_pos);
                        return false;
                    }
                }

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else if ch == '.' && !has_dot {
                        has_dot = true;
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                state.add_token(CobolSyntaxKind::NumberLiteral, start_pos, state.get_position());
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

    /// 处理标识符或关键字
    fn lex_identifier_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' || ch == '-' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = source
                    .get_text_in(core::range::Range { start: start_pos, end: state.get_position() })
                    .unwrap_or("")
                    .to_uppercase();
                let kind = match text.as_str() {
                    "ACCEPT" => CobolSyntaxKind::Accept,
                    "ADD" => CobolSyntaxKind::Add,
                    "CALL" => CobolSyntaxKind::Call,
                    "CANCEL" => CobolSyntaxKind::Cancel,
                    "CLOSE" => CobolSyntaxKind::Close,
                    "COMPUTE" => CobolSyntaxKind::Compute,
                    "CONTINUE" => CobolSyntaxKind::Continue,
                    "DELETE" => CobolSyntaxKind::Delete,
                    "DISPLAY" => CobolSyntaxKind::Display_,
                    "DIVIDE" => CobolSyntaxKind::Divide,
                    "EVALUATE" => CobolSyntaxKind::Evaluate,
                    "EXIT" => CobolSyntaxKind::Exit,
                    "GO" | "GOTO" => CobolSyntaxKind::GoTo,
                    "IF" => CobolSyntaxKind::If,
                    "INITIALIZE" => CobolSyntaxKind::Initialize,
                    "INSPECT" => CobolSyntaxKind::Inspect,
                    "MOVE" => CobolSyntaxKind::Move,
                    "MULTIPLY" => CobolSyntaxKind::Multiply,
                    "OPEN" => CobolSyntaxKind::Open,
                    "PERFORM" => CobolSyntaxKind::Perform,
                    "READ" => CobolSyntaxKind::Read,
                    "RETURN" => CobolSyntaxKind::Return,
                    "REWRITE" => CobolSyntaxKind::Rewrite,
                    "SEARCH" => CobolSyntaxKind::Search,
                    "SET" => CobolSyntaxKind::Set,
                    "SORT" => CobolSyntaxKind::Sort,
                    "START" => CobolSyntaxKind::Start,
                    "STOP" => CobolSyntaxKind::Stop,
                    "STRING" => CobolSyntaxKind::String,
                    "SUBTRACT" => CobolSyntaxKind::Subtract,
                    "UNSTRING" => CobolSyntaxKind::Unstring,
                    "WRITE" => CobolSyntaxKind::Write,

                    // 数据部门关键                    "DATA" => CobolSyntaxKind::Data,
                    "DIVISION" => CobolSyntaxKind::Division,
                    "SECTION" => CobolSyntaxKind::Section,
                    "WORKING-STORAGE" => CobolSyntaxKind::WorkingStorage,
                    "FILE-SECTION" => CobolSyntaxKind::FileSection,
                    "LINKAGE-SECTION" => CobolSyntaxKind::LinkageSection,
                    "LOCAL-STORAGE-SECTION" => CobolSyntaxKind::LocalStorageSection,

                    // 程序结构关键                    "IDENTIFICATION" => CobolSyntaxKind::Identification,
                    "PROGRAM" => CobolSyntaxKind::Program,
                    "ENVIRONMENT" => CobolSyntaxKind::Environment,
                    "CONFIGURATION" => CobolSyntaxKind::Configuration,
                    "INPUT-OUTPUT" => CobolSyntaxKind::InputOutput,
                    "FILE" => CobolSyntaxKind::File,
                    "PROCEDURE" => CobolSyntaxKind::Procedure,

                    // 数据类型和级                    "PIC" | "PICTURE" => CobolSyntaxKind::Picture,
                    "VALUE" => CobolSyntaxKind::Value,
                    "OCCURS" => CobolSyntaxKind::Occurs,
                    "REDEFINES" => CobolSyntaxKind::Redefines,
                    "USAGE" => CobolSyntaxKind::Usage,
                    "COMP" => CobolSyntaxKind::Comp,
                    "COMP-1" => CobolSyntaxKind::Comp1,
                    "COMP-2" => CobolSyntaxKind::Comp2,
                    "COMP-3" => CobolSyntaxKind::Comp3,
                    "COMP-4" => CobolSyntaxKind::Comp4,
                    "COMP-5" => CobolSyntaxKind::Comp5,
                    "BINARY" => CobolSyntaxKind::Binary,
                    "PACKED-DECIMAL" => CobolSyntaxKind::Packed,

                    // 文件操作关键                    "SELECT" => CobolSyntaxKind::Select,
                    "ASSIGN" => CobolSyntaxKind::Assign,
                    "ORGANIZATION" => CobolSyntaxKind::Organization,
                    "ACCESS" => CobolSyntaxKind::Access,
                    "RECORD" => CobolSyntaxKind::Record,
                    "KEY" => CobolSyntaxKind::Key,
                    "STATUS" => CobolSyntaxKind::Status,
                    "SEQUENTIAL" => CobolSyntaxKind::Sequential,
                    "RANDOM" => CobolSyntaxKind::Random,
                    "DYNAMIC" => CobolSyntaxKind::Dynamic,
                    "INDEXED" => CobolSyntaxKind::Indexed,
                    "RELATIVE" => CobolSyntaxKind::Relative,

                    // 条件和控制流
                    "WHEN" => CobolSyntaxKind::When,
                    "OTHER" => CobolSyntaxKind::Other,
                    "ALSO" => CobolSyntaxKind::Also,
                    "THROUGH" | "THRU" => CobolSyntaxKind::Thru,
                    "UNTIL" => CobolSyntaxKind::Until,
                    "VARYING" => CobolSyntaxKind::Varying,
                    "FROM" => CobolSyntaxKind::From,
                    "BY" => CobolSyntaxKind::By,
                    "AFTER" => CobolSyntaxKind::After,
                    "BEFORE" => CobolSyntaxKind::Before,

                    // 逻辑操作                    "AND" => CobolSyntaxKind::And,
                    "OR" => CobolSyntaxKind::Or,
                    "NOT" => CobolSyntaxKind::Not,
                    "EQUAL" => CobolSyntaxKind::Equal,
                    "GREATER" => CobolSyntaxKind::Greater,
                    "LESS" => CobolSyntaxKind::Less,

                    _ => CobolSyntaxKind::Identifier,
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

    /// 处理操作符和分隔
    fn lex_operator_or_delimiter(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '+' => {
                    state.advance(1);
                    CobolSyntaxKind::Plus
                }
                '-' => {
                    state.advance(1);
                    CobolSyntaxKind::Minus
                }
                '*' => {
                    state.advance(1);
                    if let Some('*') = state.peek() {
                        state.advance(1);
                        CobolSyntaxKind::Power
                    }
                    else {
                        CobolSyntaxKind::Star
                    }
                }
                '/' => {
                    state.advance(1);
                    CobolSyntaxKind::Slash
                }
                '=' => {
                    state.advance(1);
                    CobolSyntaxKind::EqualSign
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CobolSyntaxKind::GreaterEqual
                    }
                    else {
                        CobolSyntaxKind::GreaterThan
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CobolSyntaxKind::LessEqual
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        CobolSyntaxKind::NotEqual
                    }
                    else {
                        CobolSyntaxKind::LessThan
                    }
                }
                '(' => {
                    state.advance(1);
                    CobolSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    CobolSyntaxKind::RightParen
                }
                ',' => {
                    state.advance(1);
                    CobolSyntaxKind::Comma
                }
                '.' => {
                    state.advance(1);
                    CobolSyntaxKind::Period
                }
                ';' => {
                    state.advance(1);
                    CobolSyntaxKind::Semicolon
                }
                ':' => {
                    state.advance(1);
                    CobolSyntaxKind::Colon
                }
                '"' => {
                    state.advance(1);
                    CobolSyntaxKind::Quote
                }
                '\'' => {
                    state.advance(1);
                    CobolSyntaxKind::Apostrophe
                }
                '@' => {
                    state.advance(1);
                    CobolSyntaxKind::At
                }
                '#' => {
                    state.advance(1);
                    CobolSyntaxKind::Hash
                }
                '$' => {
                    state.advance(1);
                    CobolSyntaxKind::Dollar
                }
                '&' => {
                    state.advance(1);
                    CobolSyntaxKind::Ampersand
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

impl<'config> Lexer<CobolLanguage> for CobolLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<CobolSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_operator_or_delimiter(&mut state) {
                continue;
            }

            // 如果没有匹配到任何模式，跳过当前字符
            if let Some(ch) = state.peek() {
                let start_pos = state.get_position();
                state.advance(ch.len_utf8());
                state.add_token(CobolSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(CobolSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}

use crate::{kind::CobolSyntaxKind, language::CobolLanguage};
use oak_core::{
    IncrementalCache,
    lexer::{LexOutput, Lexer, LexerState},
    source::Source,
};

pub struct CobolLexer;

impl CobolLexer {
    pub fn new() -> Self {
        Self
    }

    fn skip_whitespace(&self, state: &mut LexerState<impl Source, CobolLanguage>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_whitespace() && ch != '\n' && ch != '\r' {
                let start_pos = state.get_position();
                while let Some(ch) = state.peek() {
                    if ch.is_whitespace() && ch != '\n' && ch != '\r' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                state.add_token(CobolSyntaxKind::Whitespace, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_newline(&self, state: &mut LexerState<impl Source, CobolLanguage>) -> bool {
        if let Some(ch) = state.peek() {
            if ch == '\n' || ch == '\r' {
                let start_pos = state.get_position();
                state.advance(ch.len_utf8());
                if ch == '\r' && state.peek() == Some('\n') {
                    state.advance(1);
                }
                state.add_token(CobolSyntaxKind::Newline, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_comment(&self, state: &mut LexerState<impl Source, CobolLanguage>) -> bool {
        if let Some('*') = state.peek() {
            let start_pos = state.get_position();
            state.advance(1);

            // COBOL 注释以 * 开头，读取到行尾
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(CobolSyntaxKind::Comment, start_pos, state.get_position());
            return true;
        }
        false
    }

    fn lex_string(&self, state: &mut LexerState<impl Source, CobolLanguage>) -> bool {
        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                let start_pos = state.get_position();
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch == quote {
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
                        break; // 字符串不能跨行
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(CobolSyntaxKind::StringLiteral, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_number(&self, state: &mut LexerState<impl Source, CobolLanguage>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                let start_pos = state.get_position();

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '.' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                state.add_token(CobolSyntaxKind::NumberLiteral, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_identifier_or_keyword(&self, state: &mut LexerState<impl Source, CobolLanguage>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                let start_pos = state.get_position();

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_in(std::range::Range { start: start_pos, end: state.get_position() }).to_uppercase();
                let kind = match text.as_str() {
                    "ACCEPT" => CobolSyntaxKind::Accept,
                    "ADD" => CobolSyntaxKind::Add,
                    "CALL" => CobolSyntaxKind::Call,
                    "CANCEL" => CobolSyntaxKind::Cancel,
                    "CLOSE" => CobolSyntaxKind::Close,
                    "COMPUTE" => CobolSyntaxKind::Compute,
                    "CONTINUE" => CobolSyntaxKind::Continue,
                    "DELETE" => CobolSyntaxKind::Delete,
                    "DISPLAY" => CobolSyntaxKind::Display,
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
                    "IDENTIFICATION" => CobolSyntaxKind::Identification,
                    "DIVISION" => CobolSyntaxKind::Division,
                    "SECTION" => CobolSyntaxKind::Section,
                    "WORKING-STORAGE" => CobolSyntaxKind::WorkingStorage,
                    "DATA" => CobolSyntaxKind::Data,
                    "PROCEDURE" => CobolSyntaxKind::Procedure,
                    "PIC" | "PICTURE" => CobolSyntaxKind::Picture,
                    "VALUE" => CobolSyntaxKind::Value,
                    "OCCURS" => CobolSyntaxKind::Occurs,
                    "REDEFINES" => CobolSyntaxKind::Redefines,
                    "COMP" | "COMPUTATIONAL" => CobolSyntaxKind::Comp,
                    "BINARY" => CobolSyntaxKind::Binary,
                    "PACKED-DECIMAL" => CobolSyntaxKind::Packed,
                    "USAGE" => CobolSyntaxKind::Usage,

                    "OR" => CobolSyntaxKind::Or,
                    "NOT" => CobolSyntaxKind::Not,
                    "EQUAL" | "=" => CobolSyntaxKind::Equal,
                    "GREATER" | ">" => CobolSyntaxKind::Greater,
                    "LESS" | "<" => CobolSyntaxKind::Less,
                    "THROUGH" | "THRU" => CobolSyntaxKind::Through,
                    "VARYING" => CobolSyntaxKind::Varying,
                    "FROM" => CobolSyntaxKind::From,
                    "BY" => CobolSyntaxKind::By,
                    "UNTIL" => CobolSyntaxKind::Until,
                    "WHEN" => CobolSyntaxKind::When,
                    "OTHER" => CobolSyntaxKind::Other,
                    "ALSO" => CobolSyntaxKind::Also,
                    _ => CobolSyntaxKind::Identifier,
                };

                state.add_token(kind, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_operator_or_delimiter(&self, state: &mut LexerState<impl Source, CobolLanguage>) -> bool {
        if let Some(ch) = state.peek() {
            let start_pos = state.get_position();

            let kind = match ch {
                '+' => CobolSyntaxKind::Plus,
                '-' => CobolSyntaxKind::Minus,
                '*' => CobolSyntaxKind::Star,
                '/' => CobolSyntaxKind::Slash,
                '=' => CobolSyntaxKind::EqualSign,
                '>' => CobolSyntaxKind::GreaterThan,
                '<' => CobolSyntaxKind::LessThan,
                '(' => CobolSyntaxKind::LeftParen,
                ')' => CobolSyntaxKind::RightParen,
                ',' => CobolSyntaxKind::Comma,
                '.' => CobolSyntaxKind::Period,
                '"' => CobolSyntaxKind::Quote,
                '\'' => CobolSyntaxKind::Apostrophe,
                '@' => CobolSyntaxKind::At,
                '#' => CobolSyntaxKind::Hash,
                '$' => CobolSyntaxKind::Dollar,
                '&' => CobolSyntaxKind::Ampersand,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(kind, start_pos, state.get_position());
            return true;
        }
        false
    }
}

impl Lexer<CobolLanguage> for CobolLexer {
    fn lex_incremental(
        &self,
        source: impl Source,
        _changed: usize,
        _cache: IncrementalCache<CobolLanguage>,
    ) -> LexOutput<CobolLanguage> {
        let mut state = LexerState::new_with_cache(source, _changed, _cache);

        while state.not_at_end() {
            // 尝试各种词法规则
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

            if self.lex_identifier_or_keyword(&mut state) {
                continue;
            }

            if self.lex_operator_or_delimiter(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(CobolSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(CobolSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish(Ok(()))
    }
}

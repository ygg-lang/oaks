use crate::{kind::TexSyntaxKind, language::TexLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, TexLanguage>;

pub struct TexLexer<'config> {
    config: &'config TexLanguage,
}

impl<'config> TexLexer<'config> {
    pub fn new(config: &'config TexLanguage) -> Self {
        Self { config }
    }

    fn skip_whitespace(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();
        let mut found_whitespace = false;

        while let Some(ch) = state.peek() {
            if ch.is_whitespace() && ch != '\n' && ch != '\r' {
                state.advance(ch.len_utf8());
                found_whitespace = true;
            }
            else {
                break;
            }
        }

        if found_whitespace {
            state.add_token(TexSyntaxKind::Whitespace, start_pos, state.get_position());
        }

        found_whitespace
    }

    fn lex_newline(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch == '\n' {
                state.advance(1);
                state.add_token(TexSyntaxKind::Newline, start_pos, state.get_position());
                return true;
            }
            else if ch == '\r' {
                state.advance(1);
                if state.peek() == Some('\n') {
                    state.advance(1);
                }
                state.add_token(TexSyntaxKind::Newline, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_line_comment(&self, state: &mut State, _source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some('%') {
            state.advance(1);

            // 读取到行尾
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(TexSyntaxKind::Comment, start_pos, state.get_position());
            return true;
        }

        false
    }

    fn lex_block_comment(&self, _state: &mut State, _source: &SourceText) -> bool {
        // TeX 没有块注释，返回 false
        false
    }

    fn lex_string_literal(&self, _state: &mut State, _source: &SourceText) -> bool {
        // TeX 的字符串处理比较复杂，暂时返回 false
        false
    }

    fn lex_template_literal(&self, _state: &mut State, _source: &SourceText) -> bool {
        // TeX 没有模板字面量，返回 false
        false
    }

    fn lex_numeric_literal(&self, state: &mut State, _source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                // 读取数字
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '.' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(TexSyntaxKind::Number, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_identifier_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' || ch == '\\' {
                // 读取标识符或命令
                if ch == '\\' {
                    state.advance(1); // 跳过反斜杠
                }

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 检查是否是关键字
                let text = source.get_text_in((start_pos..state.get_position()).into()).unwrap_or("");
                let kind = match text {
                    "\\begin" => TexSyntaxKind::BeginKeyword,
                    "\\end" => TexSyntaxKind::EndKeyword,
                    "\\documentclass" => TexSyntaxKind::DocumentclassKeyword,
                    "\\usepackage" => TexSyntaxKind::UsepackageKeyword,
                    "\\section" => TexSyntaxKind::SectionKeyword,
                    "\\subsection" => TexSyntaxKind::SubsectionKeyword,
                    "\\subsubsection" => TexSyntaxKind::SubsubsectionKeyword,
                    "\\chapter" => TexSyntaxKind::ChapterKeyword,
                    "\\part" => TexSyntaxKind::PartKeyword,
                    "\\title" => TexSyntaxKind::TitleKeyword,
                    "\\author" => TexSyntaxKind::AuthorKeyword,
                    "\\date" => TexSyntaxKind::DateKeyword,
                    "\\maketitle" => TexSyntaxKind::MaketitleKeyword,
                    "\\tableofcontents" => TexSyntaxKind::TableofcontentsKeyword,
                    "\\item" => TexSyntaxKind::ItemKeyword,
                    "\\label" => TexSyntaxKind::LabelKeyword,
                    "\\ref" => TexSyntaxKind::RefKeyword,
                    "\\cite" => TexSyntaxKind::CiteKeyword,
                    "\\includegraphics" => TexSyntaxKind::IncludegraphicsKeyword,
                    "\\textbf" => TexSyntaxKind::TextbfKeyword,
                    "\\textit" => TexSyntaxKind::TextitKeyword,
                    "\\emph" => TexSyntaxKind::EmphKeyword,
                    _ if text.starts_with('\\') => TexSyntaxKind::Command,
                    _ => TexSyntaxKind::Identifier,
                };

                state.add_token(kind, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_operator_or_punctuation(&self, state: &mut State, _source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '{' => TexSyntaxKind::LeftBrace,
                '}' => TexSyntaxKind::RightBrace,
                '[' => TexSyntaxKind::LeftBracket,
                ']' => TexSyntaxKind::RightBracket,
                '(' => TexSyntaxKind::LeftParen,
                ')' => TexSyntaxKind::RightParen,
                '$' => TexSyntaxKind::Dollar,
                '&' => TexSyntaxKind::Ampersand,
                '#' => TexSyntaxKind::Hash,
                '^' => TexSyntaxKind::Caret,
                '_' => TexSyntaxKind::Underscore,
                '~' => TexSyntaxKind::Tilde,
                '=' => TexSyntaxKind::Equals,
                '+' => TexSyntaxKind::Plus,
                '-' => TexSyntaxKind::Minus,
                '*' => TexSyntaxKind::Star,
                '/' => TexSyntaxKind::Slash,
                '\\' => TexSyntaxKind::Backslash,
                '|' => TexSyntaxKind::Pipe,
                '<' => TexSyntaxKind::LessThan,
                '>' => TexSyntaxKind::GreaterThan,
                '!' => TexSyntaxKind::Exclamation,
                '?' => TexSyntaxKind::Question,
                '@' => TexSyntaxKind::At,
                '%' => TexSyntaxKind::Percent,
                ':' => TexSyntaxKind::Colon,
                ';' => TexSyntaxKind::Semicolon,
                ',' => TexSyntaxKind::Comma,
                '.' => TexSyntaxKind::Dot,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(kind, start_pos, state.get_position());
            return true;
        }

        false
    }
}

impl<'config> Lexer<TexLanguage> for TexLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<TexSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_line_comment(&mut state, source) {
                continue;
            }

            if self.lex_block_comment(&mut state, source) {
                continue;
            }

            if self.lex_string_literal(&mut state, source) {
                continue;
            }

            if self.lex_template_literal(&mut state, source) {
                continue;
            }

            if self.lex_numeric_literal(&mut state, source) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_operator_or_punctuation(&mut state, source) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(TexSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(TexSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}

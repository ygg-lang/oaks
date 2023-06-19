use crate::{kind::TexSyntaxKind, language::TexLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, TexLanguage>;

static TEX_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static TEX_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["%"] });

#[derive(Clone)]
pub struct TexLexer<'config> {
    config: &'config TexLanguage,
}

impl<'config> Lexer<TexLanguage> for TexLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<TexLanguage>,
    ) -> LexOutput<TexLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> TexLexer<'config> {
    pub fn new(config: &'config TexLanguage) -> Self {
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

            if self.lex_command(state) {
                continue;
            }

            if self.lex_math_delimiters(state) {
                continue;
            }

            if self.lex_braces_and_brackets(state) {
                continue;
            }

            if self.lex_special_chars(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_text(state) {
                continue;
            }

            state.safe_check(safe_point);
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(TexSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match TEX_WHITESPACE.scan(state.rest(), state.get_position(), TexSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // TeX 行注释: % ... 直到换行
        if rest.starts_with("%") {
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(TexSyntaxKind::Comment, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_command<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if state.current() != Some('\\') {
            return false;
        }

        state.advance(1); // consume '\'

        // 读取命令名
        let mut has_name = false;
        while let Some(ch) = state.current() {
            if ch.is_ascii_alphabetic() {
                state.advance(1);
                has_name = true;
            }
            else {
                break;
            }
        }

        if has_name {
            let end = state.get_position();
            let text = state.get_text_in((start + 1..end).into()); // 跳过反斜杠

            let kind = match text {
                "begin" => TexSyntaxKind::BeginKeyword,
                "end" => TexSyntaxKind::EndKeyword,
                "documentclass" => TexSyntaxKind::DocumentclassKeyword,
                "usepackage" => TexSyntaxKind::UsepackageKeyword,
                "section" => TexSyntaxKind::SectionKeyword,
                "subsection" => TexSyntaxKind::SubsectionKeyword,
                "subsubsection" => TexSyntaxKind::SubsubsectionKeyword,
                "chapter" => TexSyntaxKind::ChapterKeyword,
                "part" => TexSyntaxKind::PartKeyword,
                "title" => TexSyntaxKind::TitleKeyword,
                "author" => TexSyntaxKind::AuthorKeyword,
                "date" => TexSyntaxKind::DateKeyword,
                "maketitle" => TexSyntaxKind::MaketitleKeyword,
                "tableofcontents" => TexSyntaxKind::TableofcontentsKeyword,
                "item" => TexSyntaxKind::ItemKeyword,
                "label" => TexSyntaxKind::LabelKeyword,
                "ref" => TexSyntaxKind::RefKeyword,
                "cite" => TexSyntaxKind::CiteKeyword,
                "includegraphics" => TexSyntaxKind::IncludegraphicsKeyword,
                "textbf" => TexSyntaxKind::TextbfKeyword,
                "textit" => TexSyntaxKind::TextitKeyword,
                "emph" => TexSyntaxKind::EmphKeyword,
                "frac" => TexSyntaxKind::Frac,
                "sqrt" => TexSyntaxKind::Sqrt,
                "sum" => TexSyntaxKind::Sum,
                "int" => TexSyntaxKind::Int,
                "lim" => TexSyntaxKind::Lim,
                "alpha" => TexSyntaxKind::Alpha,
                "beta" => TexSyntaxKind::Beta,
                "gamma" => TexSyntaxKind::Gamma,
                "delta" => TexSyntaxKind::Delta,
                "epsilon" => TexSyntaxKind::Epsilon,
                _ => TexSyntaxKind::Command,
            };

            state.add_token(kind, start, state.get_position());
            return true;
        }

        // 如果没有命令名，只是一个反斜杠
        state.add_token(TexSyntaxKind::Backslash, start, state.get_position());
        true
    }

    fn lex_math_delimiters<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        if rest.starts_with("$$") {
            state.advance(2);
            state.add_token(TexSyntaxKind::DoubleDollar, start, state.get_position());
            return true;
        }

        if rest.starts_with("$") {
            state.advance(1);
            state.add_token(TexSyntaxKind::Dollar, start, state.get_position());
            return true;
        }

        false
    }

    fn lex_braces_and_brackets<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.current() {
            let kind = match ch {
                '{' => TexSyntaxKind::LeftBrace,
                '}' => TexSyntaxKind::RightBrace,
                '[' => TexSyntaxKind::LeftBracket,
                ']' => TexSyntaxKind::RightBracket,
                '(' => TexSyntaxKind::LeftParen,
                ')' => TexSyntaxKind::RightParen,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(kind, start, state.get_position());
            return true;
        }

        false
    }

    fn lex_special_chars<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.current() {
            let kind = match ch {
                '&' => TexSyntaxKind::Ampersand,
                '#' => TexSyntaxKind::Hash,
                '^' => TexSyntaxKind::Caret,
                '_' => TexSyntaxKind::Underscore,
                '~' => TexSyntaxKind::Tilde,
                '=' => TexSyntaxKind::Equal,
                '+' => TexSyntaxKind::Plus,
                '-' => TexSyntaxKind::Minus,
                '*' => TexSyntaxKind::Star,
                '/' => TexSyntaxKind::Slash,
                '|' => TexSyntaxKind::Pipe,
                '<' => TexSyntaxKind::Less,
                '>' => TexSyntaxKind::Greater,
                '!' => TexSyntaxKind::Exclamation,
                '?' => TexSyntaxKind::Question,
                '@' => TexSyntaxKind::At,
                ':' => TexSyntaxKind::Colon,
                ';' => TexSyntaxKind::Semicolon,
                ',' => TexSyntaxKind::Comma,
                '.' => TexSyntaxKind::Dot,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(kind, start, state.get_position());
            return true;
        }

        false
    }

    fn lex_number<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let first = match state.current() {
            Some(c) => c,
            None => return false,
        };

        if !first.is_ascii_digit() {
            return false;
        }

        state.advance(1);
        while let Some(c) = state.current() {
            if c.is_ascii_digit() || c == '.' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        state.add_token(TexSyntaxKind::Number, start, state.get_position());
        true
    }

    fn lex_text<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.current() {
            if ch.is_ascii_alphabetic() {
                state.advance(1);
                while let Some(c) = state.current() {
                    if c.is_ascii_alphanumeric() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }
                state.add_token(TexSyntaxKind::Identifier, start, state.get_position());
                return true;
            }
        }

        false
    }
}

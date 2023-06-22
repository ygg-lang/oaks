use crate::{kind::TexSyntaxKind, language::TexLanguage};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::{CommentConfig, LexOutput, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, TexLanguage>;

static TEX_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static TEX_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "%", block_start: "", block_end: "", nested_blocks: false });

#[derive(Clone, Default)]
pub struct TexLexer {}

impl Lexer<TexLanguage> for TexLexer {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<TexLanguage>) -> LexOutput<TexLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl TexLexer {
    pub fn new(_config: &TexLanguage) -> Self {
        Self {}
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
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

            state.advance_if_dead_lock(safe_point);
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(TexSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        TEX_WHITESPACE.scan(state, TexSyntaxKind::Whitespace)
    }

    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        TEX_COMMENT.scan(state, TexSyntaxKind::Comment, TexSyntaxKind::Comment)
    }

    fn lex_command<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if state.peek() != Some('\\') {
            return false;
        }

        state.advance(1); // consume '\'

        // 读取命令名
        let mut has_name = false;
        while let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() {
                state.advance(ch.len_utf8());
                has_name = true;
            }
            else {
                break;
            }
        }

        if has_name {
            let end = state.get_position();
            let text = state.get_text_in((start + 1..end).into()); // 跳过反斜杠

            let kind = match text.as_ref() {
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

    fn lex_math_delimiters<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if state.consume_if_starts_with("$$") {
            state.add_token(TexSyntaxKind::DoubleDollar, start, state.get_position());
            return true;
        }

        if state.consume_if_starts_with("$") {
            state.add_token(TexSyntaxKind::Dollar, start, state.get_position());
            return true;
        }

        false
    }

    fn lex_braces_and_brackets<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
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

    fn lex_special_chars<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
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

    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let first = match state.peek() {
            Some(c) => c,
            None => return false,
        };

        if !first.is_ascii_digit() {
            return false;
        }

        state.advance(1);
        while let Some(c) = state.peek() {
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

    fn lex_text<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() {
                state.advance(ch.len_utf8());
                while let Some(c) = state.peek() {
                    if c.is_ascii_alphanumeric() {
                        state.advance(c.len_utf8());
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

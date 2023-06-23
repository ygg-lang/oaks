#![doc = include_str!("readme.md")]
use crate::{language::TexLanguage, lexer::token_type::TexTokenType};
pub mod token_type;
use oak_core::{
    Lexer, LexerCache, LexerState, OakError, Source, TextEdit,
    lexer::{CommentConfig, LexOutput, WhitespaceConfig},
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, TexLanguage>;

static TEX_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static TEX_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "%", block_start: "", block_end: "", nested_blocks: false });

#[derive(Clone, Debug)]
pub struct TexLexer<'config> {
    _config: &'config TexLanguage,
}

impl<'config> Lexer<TexLanguage> for TexLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<TexLanguage>) -> LexOutput<TexLanguage> {
        let mut state = State::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> TexLexer<'config> {
    pub fn new(config: &'config TexLanguage) -> Self {
        Self { _config: config }
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

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        TEX_WHITESPACE.scan(state, TexTokenType::Whitespace)
    }

    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        TEX_COMMENT.scan(state, TexTokenType::Comment, TexTokenType::Comment)
    }

    fn lex_command<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if state.peek() != Some('\\') {
            return false;
        }

        state.advance(1); // consume '\'

        // 读取命令名
        let mut has_name = false;
        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() {
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphabetic() {
                        state.advance(ch.len_utf8());
                        has_name = true
                    }
                    else {
                        break;
                    }
                }
            }
            else {
                // Single non-alphabetic character command (e.g., \\, \&, \$, \ )
                state.advance(ch.len_utf8());
                has_name = true
            }
        }

        if has_name {
            let end = state.get_position();
            let text = state.get_text_in((start + 1..end).into()); // 跳过反斜杠

            let kind = match text.as_ref() {
                "begin" => TexTokenType::BeginKeyword,
                "end" => TexTokenType::EndKeyword,
                "documentclass" => TexTokenType::DocumentclassKeyword,
                "usepackage" => TexTokenType::UsepackageKeyword,
                "section" => TexTokenType::SectionKeyword,
                "subsection" => TexTokenType::SubsectionKeyword,
                "subsubsection" => TexTokenType::SubsubsectionKeyword,
                "chapter" => TexTokenType::ChapterKeyword,
                "part" => TexTokenType::PartKeyword,
                "title" => TexTokenType::TitleKeyword,
                "author" => TexTokenType::AuthorKeyword,
                "date" => TexTokenType::DateKeyword,
                "maketitle" => TexTokenType::MaketitleKeyword,
                "tableofcontents" => TexTokenType::TableofcontentsKeyword,
                "item" => TexTokenType::ItemKeyword,
                "label" => TexTokenType::LabelKeyword,
                "ref" => TexTokenType::RefKeyword,
                "cite" => TexTokenType::CiteKeyword,
                "includegraphics" => TexTokenType::IncludegraphicsKeyword,
                "textbf" => TexTokenType::TextbfKeyword,
                "textit" => TexTokenType::TextitKeyword,
                "emph" => TexTokenType::EmphKeyword,
                "frac" => TexTokenType::Frac,
                "sqrt" => TexTokenType::Sqrt,
                "sum" => TexTokenType::Sum,
                "int" => TexTokenType::Int,
                "lim" => TexTokenType::Lim,
                "alpha" => TexTokenType::Alpha,
                "beta" => TexTokenType::Beta,
                "gamma" => TexTokenType::Gamma,
                "delta" => TexTokenType::Delta,
                "epsilon" => TexTokenType::Epsilon,
                "zeta" => TexTokenType::Zeta,
                "eta" => TexTokenType::Eta,
                "theta" => TexTokenType::Theta,
                "iota" => TexTokenType::Iota,
                "kappa" => TexTokenType::Kappa,
                "lambda" => TexTokenType::Lambda,
                "mu" => TexTokenType::Mu,
                "nu" => TexTokenType::Nu,
                "xi" => TexTokenType::Xi,
                "omicron" => TexTokenType::Omicron,
                "pi" => TexTokenType::Pi,
                "rho" => TexTokenType::Rho,
                "sigma" => TexTokenType::Sigma,
                "tau" => TexTokenType::Tau,
                "upsilon" => TexTokenType::Upsilon,
                "phi" => TexTokenType::Phi,
                "chi" => TexTokenType::Chi,
                "psi" => TexTokenType::Psi,
                "omega" => TexTokenType::Omega,
                "varepsilon" => TexTokenType::VarEpsilon,
                "vartheta" => TexTokenType::VarTheta,
                "varkappa" => TexTokenType::VarKappa,
                "varpi" => TexTokenType::VarPi,
                "varrho" => TexTokenType::VarRho,
                "varsigma" => TexTokenType::VarSigma,
                "varphi" => TexTokenType::VarPhi,
                "Gamma" => TexTokenType::UpperGamma,
                "Delta" => TexTokenType::UpperDelta,
                "Theta" => TexTokenType::UpperTheta,
                "Lambda" => TexTokenType::UpperLambda,
                "Xi" => TexTokenType::UpperXi,
                "Pi" => TexTokenType::UpperPi,
                "Sigma" => TexTokenType::UpperSigma,
                "Upsilon" => TexTokenType::UpperUpsilon,
                "Phi" => TexTokenType::UpperPhi,
                "Psi" => TexTokenType::UpperPsi,
                "Omega" => TexTokenType::UpperOmega,
                _ => TexTokenType::Command,
            };

            state.add_token(kind, start, state.get_position());
            return true;
        }

        // 如果没有命令名，只是一个反斜杠
        state.add_token(TexTokenType::Backslash, start, state.get_position());
        true
    }

    fn lex_math_delimiters<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if state.consume_if_starts_with("$$") {
            state.add_token(TexTokenType::DoubleDollar, start, state.get_position());
            return true;
        }

        if state.consume_if_starts_with("$") {
            state.add_token(TexTokenType::Dollar, start, state.get_position());
            return true;
        }

        false
    }

    fn lex_braces_and_brackets<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '{' => TexTokenType::LeftBrace,
                '}' => TexTokenType::RightBrace,
                '[' => TexTokenType::LeftBracket,
                ']' => TexTokenType::RightBracket,
                '(' => TexTokenType::LeftParen,
                ')' => TexTokenType::RightParen,
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
                '&' => TexTokenType::Ampersand,
                '#' => TexTokenType::Hash,
                '^' => TexTokenType::Caret,
                '_' => TexTokenType::Underscore,
                '~' => TexTokenType::Tilde,
                '=' => TexTokenType::Equals,
                '+' => TexTokenType::Plus,
                '-' => TexTokenType::Minus,
                '*' => TexTokenType::Star,
                '/' => TexTokenType::Slash,
                '|' => TexTokenType::Pipe,
                '<' => TexTokenType::Less,
                '>' => TexTokenType::Greater,
                '!' => TexTokenType::Exclamation,
                '?' => TexTokenType::Question,
                '@' => TexTokenType::At,
                ':' => TexTokenType::Colon,
                ';' => TexTokenType::Semicolon,
                ',' => TexTokenType::Comma,
                '.' => TexTokenType::Dot,
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

        state.add_token(TexTokenType::Number, start, state.get_position());
        true
    }

    fn lex_text<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() {
                state.advance(ch.len_utf8());
                while let Some(c) = state.peek() {
                    if c.is_alphanumeric() {
                        state.advance(c.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                state.add_token(TexTokenType::Identifier, start, state.get_position());
                return true;
            }
        }

        false
    }
}

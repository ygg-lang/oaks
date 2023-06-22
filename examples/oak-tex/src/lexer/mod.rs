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

#[derive(Clone, Debug)]
pub struct TexLexer<'config> {
    _config: &'config TexLanguage,
}

impl<'config> Lexer<TexLanguage> for TexLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<TexLanguage>) -> LexOutput<TexLanguage> {
        let mut state = State::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
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

            state.advance_if_dead_lock(safe_point);
        }

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
        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() {
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphabetic() {
                        state.advance(ch.len_utf8());
                        has_name = true;
                    }
                    else {
                        break;
                    }
                }
            }
            else {
                // Single non-alphabetic character command (e.g., \\, \&, \$, \ )
                state.advance(ch.len_utf8());
                has_name = true;
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
                "zeta" => TexSyntaxKind::Zeta,
                "eta" => TexSyntaxKind::Eta,
                "theta" => TexSyntaxKind::Theta,
                "iota" => TexSyntaxKind::Iota,
                "kappa" => TexSyntaxKind::Kappa,
                "lambda" => TexSyntaxKind::Lambda,
                "mu" => TexSyntaxKind::Mu,
                "nu" => TexSyntaxKind::Nu,
                "xi" => TexSyntaxKind::Xi,
                "omicron" => TexSyntaxKind::Omicron,
                "pi" => TexSyntaxKind::Pi,
                "rho" => TexSyntaxKind::Rho,
                "sigma" => TexSyntaxKind::Sigma,
                "tau" => TexSyntaxKind::Tau,
                "upsilon" => TexSyntaxKind::Upsilon,
                "phi" => TexSyntaxKind::Phi,
                "chi" => TexSyntaxKind::Chi,
                "psi" => TexSyntaxKind::Psi,
                "omega" => TexSyntaxKind::Omega,
                "varepsilon" => TexSyntaxKind::VarEpsilon,
                "vartheta" => TexSyntaxKind::VarTheta,
                "varkappa" => TexSyntaxKind::VarKappa,
                "varpi" => TexSyntaxKind::VarPi,
                "varrho" => TexSyntaxKind::VarRho,
                "varsigma" => TexSyntaxKind::VarSigma,
                "varphi" => TexSyntaxKind::VarPhi,
                "Gamma" => TexSyntaxKind::UpperGamma,
                "Delta" => TexSyntaxKind::UpperDelta,
                "Theta" => TexSyntaxKind::UpperTheta,
                "Lambda" => TexSyntaxKind::UpperLambda,
                "Xi" => TexSyntaxKind::UpperXi,
                "Pi" => TexSyntaxKind::UpperPi,
                "Sigma" => TexSyntaxKind::UpperSigma,
                "Upsilon" => TexSyntaxKind::UpperUpsilon,
                "Phi" => TexSyntaxKind::UpperPhi,
                "Psi" => TexSyntaxKind::UpperPsi,
                "Omega" => TexSyntaxKind::UpperOmega,
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
                state.add_token(TexSyntaxKind::Identifier, start, state.get_position());
                return true;
            }
        }

        false
    }
}

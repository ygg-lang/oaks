#![doc = include_str!("readme.md")]
/// Javadoc token types
pub mod token_type;

use crate::{language::JavadocLanguage, lexer::token_type::JavadocTokenType};

use oak_core::{Lexer, LexerCache, LexerState, lexer::LexOutput, source::Source};

pub(crate) type State<'a, S> = LexerState<'a, S, JavadocLanguage>;

/// Javadoc lexer
#[derive(Clone)]
pub struct JavadocLexer<'config> {
    config: &'config JavadocLanguage,
}

impl<'config> JavadocLexer<'config> {
    /// Creates a new Javadoc lexer
    pub fn new(config: &'config JavadocLanguage) -> Self {
        Self { config }
    }

    /// Skips whitespace
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' { state.advance(ch.len_utf8()) } else { break }
        }

        if state.get_position() > start_pos {
            state.add_token(JavadocTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles newlines
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(JavadocTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1)
            }
            state.add_token(JavadocTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles Javadoc comment start
    fn lex_comment_start<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            state.advance(1);
            if let Some('*') = state.peek() {
                state.advance(1);
                if let Some('*') = state.peek() {
                    state.advance(1);
                    state.add_token(JavadocTokenType::CommentStart, start_pos, state.get_position());
                    true
                }
                else {
                    // Backtrack to start position
                    state.set_position(start_pos);
                    false
                }
            }
            else {
                // Backtrack to start position
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// Handles Javadoc comment end
    fn lex_comment_end<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('*') = state.peek() {
            state.advance(1);
            if let Some('/') = state.peek() {
                state.advance(1);
                state.add_token(JavadocTokenType::CommentEnd, start_pos, state.get_position());
                true
            }
            else {
                // Backtrack to start position
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// Handles Javadoc tags
    fn lex_tag<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('@') = state.peek() {
            state.advance(1);
            let mut text = String::new();

            while let Some(ch) = state.peek() {
                if ch.is_alphabetic() || ch == '_' {
                    text.push(ch);
                    state.advance(ch.len_utf8())
                }
                else {
                    break;
                }
            }

            // Check if it is a known Javadoc tag
            let kind = match text.as_str() {
                "param" => JavadocTokenType::ParamTag,
                "return" => JavadocTokenType::ReturnTag,
                "throws" => JavadocTokenType::ThrowsTag,
                "exception" => JavadocTokenType::ExceptionTag,
                "see" => JavadocTokenType::SeeTag,
                "since" => JavadocTokenType::SinceTag,
                "version" => JavadocTokenType::VersionTag,
                "author" => JavadocTokenType::AuthorTag,
                "deprecated" => JavadocTokenType::DeprecatedTag,
                "link" => JavadocTokenType::LinkTag,
                "linkplain" => JavadocTokenType::LinkPlainTag,
                "code" => JavadocTokenType::CodeTag,
                "literal" => JavadocTokenType::LiteralTag,
                "value" => JavadocTokenType::ValueTag,
                "inheritDoc" => JavadocTokenType::InheritDocTag,
                "summary" => JavadocTokenType::SummaryTag,
                _ => JavadocTokenType::Tag,
            };

            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles HTML tags
    fn lex_html_tag<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('<') = state.peek() {
            state.advance(1);
            let mut is_closing = false;

            // Check if it is a closing tag
            if let Some('/') = state.peek() {
                is_closing = true;
                state.advance(1)
            }

            // Read tag name
            let mut tag_name = String::new();
            while let Some(ch) = state.peek() {
                if ch.is_alphabetic() || ch.is_ascii_digit() || ch == '-' {
                    tag_name.push(ch);
                    state.advance(ch.len_utf8())
                }
                else {
                    break;
                }
            }

            // Skip
            while let Some(ch) = state.peek() {
                if ch == '>' {
                    state.advance(1);
                    break;
                }
                else if ch == '<' {
                    // Unclosed tag
                    state.set_position(start_pos);
                    return false;
                }
                else {
                    state.advance(ch.len_utf8())
                }
            }

            let kind = if is_closing {
                JavadocTokenType::HtmlEndTag
            }
            else {
                match tag_name.as_str() {
                    "p" => JavadocTokenType::HtmlPTag,
                    "br" => JavadocTokenType::HtmlBrTag,
                    "code" => JavadocTokenType::HtmlCodeTag,
                    "pre" => JavadocTokenType::HtmlPreTag,
                    "b" => JavadocTokenType::HtmlBTag,
                    "i" => JavadocTokenType::HtmlITag,
                    "em" => JavadocTokenType::HtmlEmTag,
                    "strong" => JavadocTokenType::HtmlStrongTag,
                    "ul" => JavadocTokenType::HtmlUlTag,
                    "ol" => JavadocTokenType::HtmlOlTag,
                    "li" => JavadocTokenType::HtmlLiTag,
                    _ => JavadocTokenType::HtmlTag,
                }
            };

            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles text content
    fn lex_text<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == '@' || ch == '<' || ch == '*' || ch == '/' || ch == '\n' || ch == '\r' || ch == ' ' || ch == '\t' {
                break;
            }
            state.advance(ch.len_utf8())
        }

        if state.get_position() > start_pos {
            state.add_token(JavadocTokenType::Text, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles asterisk (comment line start)
    fn lex_asterisk<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('*') = state.peek() {
            state.advance(1);
            state.add_token(JavadocTokenType::Asterisk, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<JavadocLanguage> for JavadocLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<JavadocLanguage>) -> LexOutput<JavadocLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> JavadocLexer<'config> {
    /// Main lexer loop
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            // Try various lexical rules
            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.lex_comment_start(state) {
                continue;
            }

            if self.lex_comment_end(state) {
                continue;
            }

            if self.lex_tag(state) {
                continue;
            }

            if self.lex_html_tag(state) {
                continue;
            }

            if self.lex_asterisk(state) {
                continue;
            }

            if self.lex_text(state) {
                continue;
            }

            // If no rules match, check if reached end of file
            if let Some(ch) = state.peek() {
                // Skip current character and mark as error
                let start_pos = state.get_position();
                state.advance(ch.len_utf8());
                state.add_token(JavadocTokenType::Error, start_pos, state.get_position())
            }

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }
}

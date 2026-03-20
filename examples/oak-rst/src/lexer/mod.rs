#![doc = include_str!("readme.md")]
/// Token types for the reStructuredText language.
pub mod token_type;

use crate::{language::RstLanguage, lexer::token_type::RstTokenType};
use oak_core::{Lexer, LexerCache, LexerState, TextEdit, errors::OakError, lexer::LexOutput, source::Source};

pub(crate) type State<'a, S> = LexerState<'a, S, RstLanguage>;

/// Lexer for reStructuredText language.
#[derive(Clone, Debug)]
pub struct RstLexer<'config> {
    config: &'config RstLanguage,
}

impl<'config> RstLexer<'config> {
    /// Creates a new RstLexer with the given configuration.
    pub fn new(config: &'config RstLanguage) -> Self {
        Self { config }
    }

    fn run<S: Source + ?Sized>(&self, state: &mut State<S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if let Some(ch) = state.peek() {
                match ch {
                    ' ' | '\t' => {
                        self.skip_whitespace(state);
                    }
                    '\n' | '\r' => {
                        self.lex_newline(state);
                    }
                    '.' => {
                        if self.lex_comment(state) {
                            continue;
                        }
                        if self.lex_footnote_definition(state) {
                            continue;
                        }
                        if self.config.allow_directives && self.lex_directive(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '=' | '~' | '^' | '#' => {
                        if self.lex_heading_decoration(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }

                    '*' | '+' | '-' => {
                        if self.lex_list_marker(state) {
                            continue;
                        }
                        if self.lex_strong(state) {
                            continue;
                        }
                        if self.lex_emphasis(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '`' => {
                        if self.lex_code_block(state) {
                            continue;
                        }
                        if self.lex_literal(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '_' => {
                        if self.lex_emphasis(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '[' => {
                        if self.lex_link_or_reference(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '|' => {
                        if self.config.allow_substitutions && self.lex_substitution_reference(state) {
                            continue;
                        }
                        if self.lex_table(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    ']' => {
                        self.lex_text(state);
                    }
                    '(' => {
                        self.lex_text(state);
                    }
                    ')' => {
                        self.lex_text(state);
                    }
                    ':' => {
                        if self.config.allow_roles && self.lex_role(state) {
                            continue;
                        }
                        if self.lex_definition(state) {
                            continue;
                        }
                        if self.lex_cross_reference(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '!' => {
                        self.lex_text(state);
                    }
                    '0'..='9' => {
                        if self.lex_list_marker(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '\\' => {
                        self.lex_escape(state);
                    }
                    _ => {
                        self.lex_text(state);
                    }
                }
            }

            state.advance_if_dead_lock(safe_point)
        }
        Ok(())
    }

    /// Skips whitespace
    fn skip_whitespace<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
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
            state.add_token(RstTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles newlines
    fn lex_newline<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(RstTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(RstTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles comments
    fn lex_comment<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some('.') {
            // Check if it's a comment (starts with ..)
            if state.source().get_char_at(start_pos + 1) == Some('.') {
                state.advance(2);
                while state.not_at_end() {
                    if let Some(ch) = state.peek() {
                        if ch == '\n' || ch == '\r' {
                            break;
                        }
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                state.add_token(RstTokenType::Comment, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// Handles heading decorations
    fn lex_heading_decoration<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // Check if at the beginning of a line
        if start_pos > 0 {
            if let Some(prev_char) = state.source().get_char_at(start_pos - 1) {
                if prev_char != '\n' && prev_char != '\r' {
                    return false;
                }
            }
        }

        let decoration_char = state.peek().unwrap();
        let mut count = 0;
        let mut pos = start_pos;

        // Count consecutive decoration characters
        while let Some(ch) = state.source().get_char_at(pos) {
            if ch == decoration_char {
                count += 1;
                pos += 1;
            }
            else if ch == ' ' || ch == '\t' {
                pos += 1;
            }
            else {
                break;
            }
        }

        // Check if it's a valid heading decoration (at least 3 characters)
        if count >= 3 {
            // Check if it's followed by a newline
            if let Some(ch) = state.source().get_char_at(pos) {
                if ch == '\n' || ch == '\r' {
                    state.set_position(pos);
                    // Determine heading level based on decoration character
                    let token_type = match decoration_char {
                        '=' => RstTokenType::Heading1,
                        '-' => RstTokenType::Heading2,
                        '~' => RstTokenType::Heading3,
                        '^' => RstTokenType::Heading4,
                        '#' => RstTokenType::Heading5,
                        _ => RstTokenType::Heading6, // Default to level 6 for other characters
                    };
                    state.add_token(token_type, start_pos, state.get_position());
                    return true;
                }
            }
        }

        false
    }

    /// Handles directives
    fn lex_directive<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // Check if at the beginning of a line
        if start_pos > 0 {
            if let Some(prev_char) = state.source().get_char_at(start_pos - 1) {
                if prev_char != '\n' && prev_char != '\r' {
                    return false;
                }
            }
        }

        // Check if it's a directive (starts with .. )
        if state.peek() == Some('.') && state.source().get_char_at(start_pos + 1) == Some('.') {
            state.advance(2);
            // Skip whitespace after ..
            if let Some(ch) = state.peek() {
                if ch == ' ' || ch == '\t' {
                    self.skip_whitespace(state);
                    // Parse directive name
                    let directive_start = state.get_position();
                    while state.not_at_end() {
                        if let Some(ch) = state.peek() {
                            if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' || ch == ':' {
                                break;
                            }
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                    if state.get_position() > directive_start {
                        state.add_token(RstTokenType::Directive, start_pos, state.get_position());

                        // Check for directive arguments (after ::)
                        if state.not_at_end() {
                            if state.peek() == Some(':') && state.source().get_char_at(state.get_position() + 1) == Some(':') {
                                state.advance(2);
                                // Parse argument
                                if state.not_at_end() {
                                    if let Some(ch) = state.peek() {
                                        if ch == ' ' || ch == '\t' {
                                            self.skip_whitespace(state);
                                            let arg_start = state.get_position();
                                            while state.not_at_end() {
                                                if let Some(ch) = state.peek() {
                                                    if ch == '\n' || ch == '\r' {
                                                        break;
                                                    }
                                                    state.advance(ch.len_utf8());
                                                }
                                                else {
                                                    break;
                                                }
                                            }
                                            if state.get_position() > arg_start {
                                                state.add_token(RstTokenType::DirectiveArgument, arg_start, state.get_position());
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        return true;
                    }
                }
            }
        }

        false
    }

    /// Handles tables
    fn lex_table<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // Check if at the beginning of a line
        if start_pos > 0 {
            if let Some(prev_char) = state.source().get_char_at(start_pos - 1) {
                if prev_char != '\n' && prev_char != '\r' {
                    return false;
                }
            }
        }

        // Check for table line (either data line or separator line)
        if state.peek() == Some('|') || state.peek() == Some('+') {
            // Complex table detection
            let mut is_table = false;
            let mut pos = start_pos;

            // Check if this is a table line
            while pos < state.source().length() {
                if let Some(ch) = state.source().get_char_at(pos) {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    // Check for table-related characters
                    if ch == '|' || ch == '+' || ch == '-' || ch == '=' || ch == ':' {
                        is_table = true;
                    }
                    pos += 1;
                }
                else {
                    break;
                }
            }

            if is_table {
                // Process the entire table line
                while state.not_at_end() {
                    if let Some(ch) = state.peek() {
                        if ch == '\n' || ch == '\r' {
                            break;
                        }
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                state.add_token(RstTokenType::Table, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// Handles list markers
    fn lex_list_marker<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // Check if at the beginning of a line or only whitespace before
        let mut check_pos = start_pos;
        while check_pos > 0 {
            check_pos -= 1;
            if let Some(ch) = state.source().get_char_at(check_pos) {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                else if ch != ' ' && ch != '\t' {
                    return false;
                }
            }
        }

        if let Some(ch) = state.peek() {
            match ch {
                '*' | '+' | '-' => {
                    state.advance(1);
                    if let Some(next_ch) = state.peek() {
                        if next_ch == ' ' || next_ch == '\t' {
                            state.add_token(RstTokenType::BulletListMarker, start_pos, state.get_position());
                            return true;
                        }
                    }
                    state.set_position(start_pos);
                    false
                }
                '0'..='9' => {
                    // 解析枚举列表标记
                    let mut pos = start_pos;
                    let mut has_number = false;
                    let mut has_delimiter = false;

                    // 读取数字部分
                    while state.not_at_end() {
                        if let Some(ch) = state.peek() {
                            if ch.is_numeric() {
                                state.advance(1);
                                has_number = true;
                            }
                            else {
                                break;
                            }
                        }
                        else {
                            break;
                        }
                    }

                    // 读取分隔符
                    if let Some(ch) = state.peek() {
                        if ch == '.' || ch == ')' {
                            state.advance(1);
                            has_delimiter = true;
                        }
                    }

                    // 检查是否有空格
                    if has_number && has_delimiter {
                        if let Some(next_ch) = state.peek() {
                            if next_ch == ' ' || next_ch == '\t' {
                                state.add_token(RstTokenType::EnumeratedListMarker, start_pos, state.get_position());
                                return true;
                            }
                        }
                    }

                    state.set_position(start_pos);
                    false
                }
                _ => false,
            }
        }
        else {
            false
        }
    }

    /// Handles strong emphasis
    fn lex_strong<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch == '*' || ch == '_' {
                let marker = ch;
                // Check if it's a strong emphasis marker (two consecutive markers)
                if state.source().get_char_at(start_pos + 1) == Some(marker) {
                    state.advance(2);

                    // Check if it's a valid strong emphasis marker
                    if let Some(next_ch) = state.peek() {
                        if next_ch != ' ' && next_ch != '\t' && next_ch != '\n' && next_ch != '\r' {
                            state.add_token(RstTokenType::Strong, start_pos, state.get_position());
                            return true;
                        }
                    }
                    state.set_position(start_pos);
                }
            }
        }
        false
    }

    /// Handles emphasis
    fn lex_emphasis<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch == '*' || ch == '_' {
                let marker = ch;
                // Check if it's not a strong emphasis marker
                if state.source().get_char_at(start_pos + 1) != Some(marker) {
                    state.advance(1);

                    // Check if it's a valid emphasis marker
                    if let Some(next_ch) = state.peek() {
                        if next_ch != ' ' && next_ch != '\t' && next_ch != '\n' && next_ch != '\r' {
                            state.add_token(RstTokenType::Emphasis, start_pos, state.get_position());
                            return true;
                        }
                    }
                    state.set_position(start_pos);
                }
            }
        }
        false
    }

    /// Handles literal text
    fn lex_literal<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some('`') {
            state.advance(1);
            let mut found_end = false;

            while state.not_at_end() {
                if let Some(ch) = state.peek() {
                    if ch == '`' {
                        state.advance(1);
                        found_end = true;
                        break;
                    }
                    else if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }

            if found_end {
                state.add_token(RstTokenType::Literal, start_pos, state.get_position());
                return true;
            }
            state.set_position(start_pos);
        }
        false
    }

    /// Handles links and references
    fn lex_link_or_reference<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some('[') {
            state.advance(1);

            // Check if it's a footnote reference
            if state.peek() == Some('#') {
                state.advance(1);
                while state.not_at_end() {
                    if let Some(ch) = state.peek() {
                        if ch == ']' {
                            state.advance(1);
                            if state.peek() == Some('_') {
                                state.advance(1);
                                state.add_token(RstTokenType::FootnoteReference, start_pos, state.get_position());
                                return true;
                            }
                        }
                        else if ch == '\n' || ch == '\r' {
                            break;
                        }
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                state.set_position(start_pos);
                return false;
            }

            // Parse link text or reference name
            while state.not_at_end() {
                if let Some(ch) = state.peek() {
                    if ch == ']' {
                        state.advance(1);
                        state.add_token(RstTokenType::Link, start_pos, state.get_position());
                        return true;
                    }
                    else if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }
            state.set_position(start_pos);
        }
        false
    }

    /// Handles footnote and citation definitions
    fn lex_footnote_definition<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // Check if it's a footnote or citation definition (starts with .. [#]::)
        if state.peek() == Some('.') {
            if state.source().get_char_at(start_pos + 1) == Some('.') {
                state.advance(2);

                // Skip whitespace
                while state.not_at_end() {
                    if let Some(ch) = state.peek() {
                        if ch == ' ' || ch == '\t' {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                    else {
                        break;
                    }
                }

                // Check if it's a footnote or citation definition
                if state.peek() == Some('[') {
                    state.advance(1);
                    if state.peek() == Some('#') {
                        state.advance(1);
                        while state.not_at_end() {
                            if let Some(ch) = state.peek() {
                                if ch == ']' {
                                    state.advance(1);
                                    if state.peek() == Some(':') && state.source().get_char_at(state.get_position() + 1) == Some(':') {
                                        state.advance(2);
                                        state.add_token(RstTokenType::FootnoteDefinition, start_pos, state.get_position());
                                        return true;
                                    }
                                }
                                else if ch == '\n' || ch == '\r' {
                                    break;
                                }
                                state.advance(ch.len_utf8());
                            }
                            else {
                                break;
                            }
                        }
                    }
                }
                state.set_position(start_pos);
            }
        }
        false
    }

    /// Handles cross-references
    fn lex_cross_reference<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some(':') {
            state.advance(1);

            // Check if it's a cross-reference
            let mut ref_name = String::new();
            while state.not_at_end() {
                if let Some(ch) = state.peek() {
                    if ch == '`' {
                        state.advance(1);
                        // Parse reference target
                        while state.not_at_end() {
                            if let Some(ch) = state.peek() {
                                if ch == '`' {
                                    state.advance(1);
                                    state.add_token(RstTokenType::Link, start_pos, state.get_position());
                                    return true;
                                }
                                else if ch == '\n' || ch == '\r' {
                                    break;
                                }
                                state.advance(ch.len_utf8());
                            }
                            else {
                                break;
                            }
                        }
                        break;
                    }
                    else if ch == '\n' || ch == '\r' || ch == ' ' || ch == '\t' {
                        break;
                    }
                    ref_name.push(ch);
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }
            state.set_position(start_pos);
        }
        false
    }

    /// Handles definition list items
    fn lex_definition<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // Check if at the beginning of a line
        if start_pos > 0 {
            if let Some(prev_char) = state.source().get_char_at(start_pos - 1) {
                if prev_char != '\n' && prev_char != '\r' {
                    return false;
                }
            }
        }

        if state.peek() == Some(':') {
            state.advance(1);
            state.add_token(RstTokenType::DefinitionDefinition, start_pos, state.get_position());
            return true;
        }

        false
    }

    /// Handles escape sequences
    fn lex_escape<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let _start_pos = state.get_position();

        if state.peek() == Some('\\') {
            state.advance(1);
            if state.not_at_end() {
                state.advance(1);
            }
            // Treat escaped characters as regular text
            self.lex_text(state);
            true
        }
        else {
            false
        }
    }

    /// Lexes plain text
    fn lex_text<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        while state.not_at_end() {
            if let Some(ch) = state.peek() {
                // Stop when encountering special characters
                match ch {
                    ' ' | '\t' | '\n' | '\r' | '.' | '=' | '-' | '~' | '^' | '#' | '@' | '|' | '*' | '+' | '`' | '_' | '[' | ']' | '(' | ')' | ':' | '!' | '\\' => break,
                    _ => {
                        state.advance(ch.len_utf8());
                    }
                }
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(RstTokenType::Text, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Lexes substitution references
    fn lex_substitution_reference<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some('|') {
            state.advance(1);
            // Parse substitution name
            while state.not_at_end() {
                if let Some(ch) = state.peek() {
                    if ch == '|' {
                        state.advance(1);
                        state.add_token(RstTokenType::SubstitutionReference, start_pos, state.get_position());
                        return true;
                    }
                    else if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }
            state.set_position(start_pos);
        }
        false
    }

    /// Lexes roles
    fn lex_role<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some(':') {
            state.advance(1);
            // Parse role name
            while state.not_at_end() {
                if let Some(ch) = state.peek() {
                    if ch == ':' {
                        state.advance(1);
                        // Check if it's followed by backticks for role content
                        if state.peek() == Some('`') {
                            state.advance(1);
                            // Parse role content
                            while state.not_at_end() {
                                if let Some(ch) = state.peek() {
                                    if ch == '`' {
                                        state.advance(1);
                                        state.add_token(RstTokenType::Role, start_pos, state.get_position());
                                        return true;
                                    }
                                    else if ch == '\n' || ch == '\r' {
                                        break;
                                    }
                                    state.advance(ch.len_utf8());
                                }
                                else {
                                    break;
                                }
                            }
                        }
                        break;
                    }
                    else if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }
            state.set_position(start_pos);
        }
        false
    }

    /// Handles code blocks
    fn lex_code_block<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // Check for code block start (```)
        if state.peek() == Some('`') {
            if state.source().get_char_at(start_pos + 1) == Some('`') && state.source().get_char_at(start_pos + 2) == Some('`') {
                state.advance(3);

                // Try to parse language specification
                let lang_start = state.get_position();
                while state.not_at_end() {
                    if let Some(ch) = state.peek() {
                        if ch == '\n' || ch == '\r' {
                            break;
                        }
                        state.advance(ch.len_utf8());
                    }
                }

                if state.get_position() > lang_start {
                    state.add_token(RstTokenType::CodeBlockLanguage, lang_start, state.get_position());
                }

                // Add code block start token
                state.add_token(RstTokenType::CodeBlock, start_pos, state.get_position());
                return true;
            }
        }
        false
    }
}

impl<'config> Lexer<RstLanguage> for RstLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<RstLanguage>) -> LexOutput<RstLanguage> {
        let mut state = State::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> RstLexer<'config> {
    /// Runs the lexer on the given source and returns the output.
    pub fn lex_internal<'a, S: Source + ?Sized>(&self, source: &'a S) -> LexOutput<RstLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

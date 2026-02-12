#![doc = include_str!("readme.md")]
/// Token types for the Markdown language.
pub mod token_type;

use crate::{language::MarkdownLanguage, lexer::token_type::MarkdownTokenType};
use oak_core::{Lexer, LexerCache, LexerState, TextEdit, errors::OakError, lexer::LexOutput, source::Source};

pub(crate) type State<'a, S> = LexerState<'a, S, MarkdownLanguage>;

/// Lexer for Markdown language.
#[derive(Clone, Debug)]
pub struct MarkdownLexer<'config> {
    config: &'config MarkdownLanguage,
}

impl<'config> MarkdownLexer<'config> {
    /// Creates a new MarkdownLexer with the given configuration.
    pub fn new(config: &'config MarkdownLanguage) -> Self {
        Self { config }
    }

    fn run<S: Source + ?Sized>(&self, state: &mut State<S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if let Some(ch) = state.peek() {
                match ch {
                    ' ' | '\t' => {
                        if self.config.allow_indented_code_blocks && self.lex_indented_code_block(state) {
                            continue;
                        }
                        self.skip_whitespace(state);
                    }
                    '\n' | '\r' => {
                        self.lex_newline(state);
                    }
                    '$' if self.config.allow_math => {
                        if self.lex_math(state) {
                            continue;
                        }
                        self.lex_special_char(state);
                    }
                    '^' if self.config.allow_sub_superscript || self.config.allow_footnotes => {
                        if self.config.allow_footnotes && self.lex_footnote(state) {
                            continue;
                        }
                        if self.config.allow_sub_superscript && self.lex_sub_superscript(state) {
                            continue;
                        }
                        self.lex_special_char(state);
                    }
                    '#' => {
                        if self.config.allow_headings && self.lex_heading(state) {
                            continue;
                        }
                        self.lex_special_char(state);
                    }
                    '`' => {
                        if self.config.allow_fenced_code_blocks && self.lex_code_block(state) {
                            continue;
                        }
                        if self.lex_inline_code(state) {
                            continue;
                        }
                        self.lex_special_char(state);
                    }
                    '~' => {
                        if self.lex_code_block(state) {
                            continue;
                        }
                        if self.config.allow_strikethrough && self.lex_strikethrough(state) {
                            continue;
                        }
                        if self.config.allow_sub_superscript && self.lex_sub_superscript(state) {
                            continue;
                        }
                        self.lex_special_char(state);
                    }
                    '*' | '_' => {
                        if self.config.allow_horizontal_rules && self.lex_horizontal_rule(state) {
                            continue;
                        }
                        if self.config.allow_lists && self.lex_list_marker(state) {
                            continue;
                        }
                        if self.lex_emphasis(state) {
                            continue;
                        }
                        self.lex_special_char(state);
                    }
                    '-' => {
                        if self.config.allow_front_matter && self.lex_front_matter(state) {
                            continue;
                        }
                        if self.config.allow_horizontal_rules && self.lex_horizontal_rule(state) {
                            continue;
                        }
                        if self.config.allow_lists && self.lex_list_marker(state) {
                            continue;
                        }
                        self.lex_special_char(state);
                    }
                    '+' => {
                        if self.config.allow_lists && self.lex_list_marker(state) {
                            continue;
                        }
                        self.lex_special_char(state);
                    }
                    '!' => {
                        if self.lex_link_or_image(state) {
                            continue;
                        }
                        self.lex_special_char(state);
                    }
                    '[' => {
                        if self.config.allow_task_lists && self.lex_task_marker(state) {
                            continue;
                        }
                        if self.lex_link_or_image(state) {
                            continue;
                        }
                        self.lex_special_char(state);
                    }
                    '>' => {
                        if self.config.allow_blockquotes && self.lex_blockquote(state) {
                            continue;
                        }
                        self.lex_special_char(state);
                    }
                    '|' if self.config.allow_tables => {
                        self.lex_special_char(state);
                    }
                    '0'..='9' => {
                        if self.lex_list_marker(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '<' => {
                        if self.config.allow_html && self.lex_html_tag(state) {
                            continue;
                        }
                        if self.config.allow_xml && self.lex_xml_tag(state) {
                            continue;
                        }
                        self.lex_special_char(state);
                    }
                    ']' | '(' | ')' | '|' | '.' | ':' | '\\' => {
                        self.lex_special_char(state);
                    }
                    _ => {
                        if self.lex_text(state) {
                            continue;
                        }
                        // If no rules match, skip current character and mark as error
                        let start_pos = state.get_position();
                        state.advance(ch.len_utf8());
                        state.add_token(MarkdownTokenType::Error, start_pos, state.get_position());
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
            state.add_token(MarkdownTokenType::Whitespace, start_pos, state.get_position());
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
            state.add_token(MarkdownTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(MarkdownTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles headings.
    fn lex_heading<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // Check if at the beginning of a line.
        if start_pos > 0 {
            if let Some(prev_char) = state.source().get_char_at(start_pos - 1) {
                if prev_char != '\n' && prev_char != '\r' {
                    return false;
                }
            }
        }

        if let Some('#') = state.peek() {
            let mut level = 0;
            let mut pos = start_pos;

            // Count the number of '#'.
            while let Some('#') = state.source().get_char_at(pos) {
                level += 1;
                pos += 1;
                if level > 6 {
                    return false; // More than 6 levels, not a valid heading.
                }
            }

            // Check if there is whitespace after '#'.
            if let Some(ch) = state.source().get_char_at(pos) {
                if ch != ' ' && ch != '\t' && ch != '\n' && ch != '\r' {
                    return false;
                }
            }

            state.advance(level);

            let heading_kind = match level {
                1 => MarkdownTokenType::Heading1,
                2 => MarkdownTokenType::Heading2,
                3 => MarkdownTokenType::Heading3,
                4 => MarkdownTokenType::Heading4,
                5 => MarkdownTokenType::Heading5,
                6 => MarkdownTokenType::Heading6,
                _ => return false,
            };

            state.add_token(heading_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles inline code.
    fn lex_inline_code<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('`') = state.peek() {
            state.advance(1);
            let mut found_end = false;

            while let Some(ch) = state.peek() {
                if ch == '`' {
                    state.advance(1);
                    found_end = true;
                    break;
                }
                else if ch == '\n' || ch == '\r' {
                    break; // Inline code cannot span lines.
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            if found_end {
                state.add_token(MarkdownTokenType::InlineCode, start_pos, state.get_position());
                true
            }
            else {
                // Backtrack to start position.
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// Handles code blocks.
    fn lex_code_block<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // Check if at the beginning of a line.
        if start_pos > 0 {
            if let Some(prev_char) = state.source().get_char_at(start_pos - 1) {
                if prev_char != '\n' && prev_char != '\r' {
                    return false;
                }
            }
        }

        // Check if it is ``` or ~~~.
        let fence_char = if let Some('`') = state.peek() {
            '`'
        }
        else if let Some('~') = state.peek() {
            '~'
        }
        else {
            return false;
        };

        let mut fence_count = 0;
        let mut pos = start_pos;

        // Count fence characters.
        while let Some(ch) = state.source().get_char_at(pos) {
            if ch == fence_char {
                fence_count += 1;
                pos += 1;
            }
            else {
                break;
            }
        }

        if fence_count < 3 {
            return false; // At least 3 fence characters are required.
        }

        state.advance(fence_count);
        state.add_token(MarkdownTokenType::CodeFence, start_pos, state.get_position());

        // Handle language identifier.
        let lang_start = state.get_position();
        while let Some(ch) = state.peek() {
            if ch == '\n' || ch == '\r' {
                break;
            }
            else if ch != ' ' && ch != '\t' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > lang_start {
            state.add_token(MarkdownTokenType::CodeLanguage, lang_start, state.get_position());
        }

        true
    }

    /// Handles emphasis and strong.
    fn lex_emphasis<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        let marker_char = if let Some('*') = state.peek() {
            '*'
        }
        else if let Some('_') = state.peek() {
            '_'
        }
        else {
            return false;
        };

        let mut marker_count = 0;
        let mut pos = start_pos;

        // Count marker characters.
        while let Some(ch) = state.source().get_char_at(pos) {
            if ch == marker_char {
                marker_count += 1;
                pos += 1;
            }
            else {
                break;
            }
        }

        if marker_count == 0 {
            return false;
        }

        state.advance(marker_count);

        let token_kind = if marker_count >= 2 { MarkdownTokenType::Strong } else { MarkdownTokenType::Emphasis };

        state.add_token(token_kind, start_pos, state.get_position());
        true
    }

    /// Handles strikethrough.
    fn lex_strikethrough<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('~') = state.peek() {
            if let Some('~') = state.source().get_char_at(start_pos + 1) {
                state.advance(2);
                state.add_token(MarkdownTokenType::Strikethrough, start_pos, state.get_position());
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

    /// Handles links and images.
    fn lex_link_or_image<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // Check if it is an image ![.
        let is_image = if let Some('!') = state.peek() {
            state.advance(1);
            true
        }
        else {
            false
        };

        if let Some('[') = state.peek() {
            state.advance(1);

            let token_kind = if is_image { MarkdownTokenType::Image } else { MarkdownTokenType::Link };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            if is_image {
                // Backtrack exclamation.
                state.set_position(start_pos);
            }
            false
        }
    }

    /// Handles list markers.
    fn lex_list_marker<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // Check if at the beginning of a line or only whitespace before.
        let mut check_pos = start_pos;
        while check_pos > 0 {
            check_pos -= 1;
            if let Some(ch) = state.source().get_char_at(check_pos) {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                else if ch != ' ' && ch != '\t' {
                    return false; // Non-whitespace characters before.
                }
            }
        }

        if let Some(ch) = state.peek() {
            match ch {
                '-' | '*' | '+' => {
                    // Unordered list.
                    state.advance(1);
                    if let Some(next_ch) = state.peek() {
                        if next_ch == ' ' || next_ch == '\t' {
                            state.add_token(MarkdownTokenType::ListMarker, start_pos, state.get_position());
                            return true;
                        }
                    }
                    state.set_position(start_pos);
                    false
                }
                '0'..='9' => {
                    // Ordered list.
                    while let Some(digit) = state.peek() {
                        if digit.is_ascii_digit() { state.advance(1) } else { break }
                    }

                    if let Some('.') = state.peek() {
                        state.advance(1);
                        if let Some(next_ch) = state.peek() {
                            if next_ch == ' ' || next_ch == '\t' {
                                state.add_token(MarkdownTokenType::ListMarker, start_pos, state.get_position());
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

    /// Handles task markers.
    fn lex_task_marker<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('[') = state.peek() {
            state.advance(1);
            if let Some(ch) = state.peek() {
                if ch == ' ' || ch == 'x' || ch == 'X' {
                    state.advance(1);
                    if let Some(']') = state.peek() {
                        state.advance(1);
                        state.add_token(MarkdownTokenType::TaskMarker, start_pos, state.get_position());
                        return true;
                    }
                }
            }
            state.set_position(start_pos);
        }
        false
    }

    /// Handles HTML tags or comments.
    fn lex_html_tag<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        self.lex_any_tag(state, MarkdownTokenType::HtmlTag, MarkdownTokenType::HtmlComment)
    }

    /// Handles XML tags or comments.
    fn lex_xml_tag<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        self.lex_any_tag(state, MarkdownTokenType::XmlTag, MarkdownTokenType::XmlComment)
    }

    /// Common tag handling logic.
    fn lex_any_tag<S: Source + ?Sized>(&self, state: &mut State<S>, tag_kind: MarkdownTokenType, comment_kind: MarkdownTokenType) -> bool {
        let start_pos = state.get_position();

        if let Some('<') = state.peek() {
            state.advance(1);

            // Check if it is a comment <!-- -->.
            if let Some('!') = state.peek() {
                if state.source().get_char_at(state.get_position() + 1) == Some('-') && state.source().get_char_at(state.get_position() + 2) == Some('-') {
                    state.advance(3);
                    let mut found_end = false;
                    while let Some(ch) = state.peek() {
                        if ch == '-' && state.source().get_char_at(state.get_position() + 1) == Some('-') && state.source().get_char_at(state.get_position() + 2) == Some('>') {
                            state.advance(3);
                            found_end = true;
                            break;
                        }
                        state.advance(ch.len_utf8());
                    }
                    if found_end {
                        state.add_token(comment_kind, start_pos, state.get_position());
                        return true;
                    }
                }
            }

            // Normal tag parsing.
            let mut found_end = false;
            let mut in_string = None; // Track if inside quotes.

            while let Some(ch) = state.peek() {
                if let Some(quote) = in_string {
                    if ch == quote {
                        in_string = None;
                    }
                }
                else {
                    if ch == '>' {
                        state.advance(1);
                        found_end = true;
                        break;
                    }
                    else if ch == '"' || ch == '\'' {
                        in_string = Some(ch);
                    }
                }
                state.advance(ch.len_utf8());
            }

            if found_end {
                state.add_token(tag_kind, start_pos, state.get_position());
                true
            }
            else {
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// Lexes blockquotes.
    fn lex_blockquote<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // Check if we are at the start of a line or only preceded by whitespace.
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

        if let Some('>') = state.peek() {
            state.advance(1);
            state.add_token(MarkdownTokenType::BlockquoteMarker, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Lexes horizontal rules.
    fn lex_horizontal_rule<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // Check if we are at the start of a line or only preceded by whitespace.
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
            if ch == '-' || ch == '*' || ch == '_' {
                let rule_char = ch;
                let mut count = 0;
                let mut pos = start_pos;

                // Count consecutive separators.
                while let Some(current_ch) = state.source().get_char_at(pos) {
                    if current_ch == rule_char {
                        count += 1;
                        pos += 1
                    }
                    else if current_ch == ' ' || current_ch == '\t' {
                        pos += 1; // Allow spaces.
                    }
                    else {
                        break;
                    }
                }

                if count >= 3 {
                    // Check until the end of the line.
                    while let Some(current_ch) = state.source().get_char_at(pos) {
                        if current_ch == '\n' || current_ch == '\r' {
                            break;
                        }
                        else if current_ch == ' ' || current_ch == '\t' {
                            pos += 1
                        }
                        else {
                            return false; // Other characters found at the end of the line.
                        }
                    }

                    state.set_position(pos);
                    state.add_token(MarkdownTokenType::HorizontalRule, start_pos, state.get_position());
                    return true;
                }
            }
        }
        false
    }

    /// Lexes math formulas.
    fn lex_math<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('$') = state.peek() {
            state.advance(1);
            let mut is_block = false;

            if let Some('$') = state.peek() {
                state.advance(1);
                is_block = true;
            }

            let mut found_end = false;
            while let Some(ch) = state.peek() {
                if ch == '$' {
                    if is_block {
                        if let Some('$') = state.source().get_char_at(state.get_position() + 1) {
                            state.advance(2);
                            found_end = true;
                            break;
                        }
                    }
                    else {
                        state.advance(1);
                        found_end = true;
                        break;
                    }
                }
                state.advance(ch.len_utf8())
            }

            if found_end {
                let kind = if is_block { MarkdownTokenType::MathBlock } else { MarkdownTokenType::MathInline };
                state.add_token(kind, start_pos, state.get_position());
                true
            }
            else {
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// Lexes front matter.
    fn lex_front_matter<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // Must be at the start of the file.
        if start_pos != 0 {
            return false;
        }

        if state.peek() == Some('-') && state.source().get_char_at(1) == Some('-') && state.source().get_char_at(2) == Some('-') {
            state.advance(3);
            // Look for the end marker ---
            let mut found_end = false;
            while state.not_at_end() {
                if state.peek() == Some('\n') || state.peek() == Some('\r') {
                    state.advance(1);
                    if state.peek() == Some('\n') {
                        state.advance(1)
                    }
                    if state.peek() == Some('-') && state.source().get_char_at(state.get_position() + 1) == Some('-') && state.source().get_char_at(state.get_position() + 2) == Some('-') {
                        state.advance(3);
                        found_end = true;
                        break;
                    }
                }
                else {
                    state.advance(1)
                }
            }

            if found_end {
                state.add_token(MarkdownTokenType::FrontMatter, start_pos, state.get_position());
                true
            }
            else {
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// Lexes footnotes.
    fn lex_footnote<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('^') = state.peek() {
            // Check if it's [^...
            let check_pos = start_pos;
            if check_pos > 0 && state.source().get_char_at(check_pos - 1) == Some('[') {
                state.advance(1);
                while let Some(ch) = state.peek() {
                    if ch == ']' {
                        state.advance(1);
                        // Check if it's a definition [^...]:
                        if state.peek() == Some(':') {
                            state.advance(1);
                            state.add_token(MarkdownTokenType::FootnoteDefinition, start_pos - 1, state.get_position())
                        }
                        else {
                            state.add_token(MarkdownTokenType::FootnoteReference, start_pos - 1, state.get_position())
                        }
                        return true;
                    }
                    else if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8())
                }
            }
            state.set_position(start_pos);
        }
        false
    }

    /// Lexes superscripts and subscripts.
    fn lex_sub_superscript<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let marker = ch;
            if marker == '^' || marker == '~' {
                state.advance(1);
                let mut found_end = false;
                while let Some(next_ch) = state.peek() {
                    if next_ch == marker {
                        state.advance(1);
                        found_end = true;
                        break;
                    }
                    else if next_ch == ' ' || next_ch == '\t' || next_ch == '\n' || next_ch == '\r' {
                        break;
                    }
                    state.advance(next_ch.len_utf8())
                }

                if found_end {
                    let kind = if marker == '^' { MarkdownTokenType::Superscript } else { MarkdownTokenType::Subscript };
                    state.add_token(kind, start_pos, state.get_position());
                    true
                }
                else {
                    state.set_position(start_pos);
                    false
                }
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// Handles indented code blocks.
    fn lex_indented_code_block<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // Must be at the beginning of a line.
        if start_pos > 0 {
            if let Some(prev_char) = state.source().get_char_at(start_pos - 1) {
                if prev_char != '\n' && prev_char != '\r' {
                    return false;
                }
            }
        }

        // Check indentation (4 spaces or 1 tab).
        let mut indent_count = 0;
        let mut pos = start_pos;
        while let Some(ch) = state.source().get_char_at(pos) {
            if ch == ' ' {
                indent_count += 1;
                pos += 1;
                if indent_count == 4 {
                    break;
                }
            }
            else if ch == '\t' {
                indent_count = 4;
                pos += 1;
                break;
            }
            else {
                break;
            }
        }

        if indent_count >= 4 {
            state.set_position(pos);
            state.add_token(MarkdownTokenType::CodeBlock, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Lexes special characters.
    fn lex_special_char<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '[' => MarkdownTokenType::LBracket,
                ']' => MarkdownTokenType::RBracket,
                '(' => MarkdownTokenType::LParen,
                ')' => MarkdownTokenType::RParen,
                '<' => MarkdownTokenType::Less,
                '>' => MarkdownTokenType::Greater,
                '*' => MarkdownTokenType::Asterisk,
                '_' => MarkdownTokenType::Underscore,
                '`' => MarkdownTokenType::Backtick,
                '~' => MarkdownTokenType::Tilde,
                '#' => MarkdownTokenType::Hash,
                '|' => MarkdownTokenType::Pipe,
                '-' => MarkdownTokenType::Dash,
                '+' => MarkdownTokenType::Plus,
                '.' => MarkdownTokenType::Dot,
                ':' => MarkdownTokenType::Colon,
                '!' => MarkdownTokenType::Exclamation,
                '\\' => MarkdownTokenType::Escape,
                '$' => MarkdownTokenType::Dollar,
                '^' => MarkdownTokenType::Caret,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Lexes plain text.
    fn lex_text<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            // Stop when encountering a special character.
            match ch {
                ' ' | '\t' | '\n' | '\r' | '#' | '*' | '_' | '`' | '~' | '[' | ']' | '(' | ')' | '<' | '>' | '|' | '-' | '+' | '.' | ':' | '!' | '\\' | '$' | '^' => break,
                _ => {
                    state.advance(ch.len_utf8());
                }
            }
        }

        if state.get_position() > start_pos {
            state.add_token(MarkdownTokenType::Text, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<MarkdownLanguage> for MarkdownLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<MarkdownLanguage>) -> LexOutput<MarkdownLanguage> {
        let mut state = State::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> MarkdownLexer<'config> {
    /// Runs the lexer on the given source and returns the output.
    pub fn lex_internal<'a, S: Source + ?Sized>(&self, source: &'a S) -> LexOutput<MarkdownLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

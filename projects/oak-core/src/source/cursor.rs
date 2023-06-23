use crate::source::{Source, TextChunk, simd::SimdScanner};
use core::range::Range;
use std::fmt;

/// A cursor over a source that allows for efficient navigation and scanning.
///
/// # Examples
///
/// ```rust
/// # #![feature(new_range_api)]
/// # use oak_core::source::{SourceCursor, SourceText};
/// let source = SourceText::new("hello world");
/// let mut cursor = SourceCursor::new(&source);
///
/// assert_eq!(cursor.peek_char(), Some('h'));
/// cursor.set_position(6);
/// assert_eq!(cursor.peek_char(), Some('w'));
/// ```
pub struct SourceCursor<'s, S: Source + ?Sized> {
    source: &'s S,
    offset: usize,
    chunk: TextChunk<'s>,
    scratch: String,
}

impl<'s, S: Source + ?Sized> fmt::Debug for SourceCursor<'s, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SourceCursor").field("offset", &self.offset).field("chunk_start", &self.chunk.start).field("chunk_end", &self.chunk.end()).finish()
    }
}

impl<'s, S: Source + ?Sized> SourceCursor<'s, S> {
    /// Creates a new SourceCursor at the start of the source.
    pub fn new(source: &'s S) -> Self {
        Self::new_at(source, 0)
    }

    /// Creates a new SourceCursor at the specified offset.
    pub fn new_at(source: &'s S, offset: usize) -> Self {
        let end = source.length();
        let offset = offset.min(end);
        let chunk = source.chunk_at(offset);
        Self { source, offset, chunk, scratch: String::new() }
    }

    /// Returns the current byte offset of the cursor.
    #[inline]
    pub fn position(&self) -> usize {
        self.offset
    }

    /// Sets the current byte offset of the cursor.
    /// Returns the previous offset.
    #[inline]
    pub fn set_position(&mut self, offset: usize) -> usize {
        let last = self.offset;
        self.offset = offset.min(self.source.length());
        last
    }

    /// Returns the source that this cursor is iterating over.
    #[inline]
    pub fn source(&self) -> &'s S {
        self.source
    }

    /// Ensures that the current chunk is valid for the current offset.
    fn ensure_chunk(&mut self) {
        let end = self.source.length();
        if self.offset > end {
            self.offset = end
        }
        // If the offset is outside the current chunk, or at the very end of the current chunk
        // (but not at the end of the source), we need to fetch a new chunk.
        if self.offset < self.chunk.start || self.offset > self.chunk.end() || (self.offset == self.chunk.end() && self.offset < end) {
            self.chunk = self.source.chunk_at(self.offset)
        }
    }

    /// Returns the remaining text in the current chunk.
    pub fn rest(&mut self) -> &str {
        self.ensure_chunk();
        self.chunk.slice_from(self.offset)
    }

    /// Returns the end byte offset of the current chunk.
    pub fn chunk_end(&mut self) -> usize {
        self.ensure_chunk();
        self.chunk.end()
    }

    /// Peeks at the next character without advancing the cursor.
    pub fn peek_char(&mut self) -> Option<char> {
        if self.offset >= self.chunk.start {
            let rel = self.offset - self.chunk.start;
            if rel < self.chunk.text.len() {
                // Ensure rel is at a character boundary
                if self.chunk.text.is_char_boundary(rel) {
                    let text = unsafe { self.chunk.text.get_unchecked(rel..) };
                    return text.chars().next();
                }
                else {
                    // If not at a boundary, something is wrong with the offset
                    // We should probably advance to the next boundary
                    let mut i = rel;
                    while i < self.chunk.text.len() && !self.chunk.text.is_char_boundary(i) {
                        i += 1
                    }
                    if i < self.chunk.text.len() {
                        let text = unsafe { self.chunk.text.get_unchecked(i..) };
                        return text.chars().next();
                    }
                }
            }
        }
        self.rest().chars().next()
    }

    /// Peeks at the character at the specified byte offset relative to the current position.
    pub fn peek_next_n(&mut self, n: usize) -> Option<char> {
        let target_offset = self.offset + n;
        if target_offset >= self.source.length() {
            return None;
        }
        if target_offset >= self.chunk.start && target_offset < self.chunk.end() {
            let rel = target_offset - self.chunk.start;
            let text = self.chunk.text.get(rel..).unwrap_or("");
            return text.chars().next();
        }
        self.source.get_char_at(target_offset)
    }

    /// Peeks at the character immediately following the current character.
    pub fn peek_next_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.peek_next_n(ch.len_utf8())
    }

    /// Skips common ASCII whitespace using SIMD if possible.
    pub fn skip_ascii_whitespace(&mut self) -> Range<usize> {
        let start = self.offset;
        loop {
            self.ensure_chunk();
            let rel = self.offset.saturating_sub(self.chunk.start);
            let bytes = if rel < self.chunk.text.len() { unsafe { self.chunk.text.as_bytes().get_unchecked(rel..) } } else { &[] };

            if bytes.is_empty() {
                if self.offset >= self.source.length() {
                    break;
                }
                self.chunk = self.source.chunk_at(self.offset);
                continue;
            }

            let skipped = SimdScanner::skip_ascii_whitespace(bytes);
            self.offset += skipped;

            if skipped < bytes.len() || self.offset >= self.source.length() {
                break;
            }
        }
        Range { start, end: self.offset }
    }

    /// Skips ASCII digits using SIMD if possible.
    pub fn skip_ascii_digits(&mut self) -> Range<usize> {
        let start = self.offset;
        loop {
            self.ensure_chunk();
            let rel = self.offset.saturating_sub(self.chunk.start);
            let bytes = if rel < self.chunk.text.len() { unsafe { self.chunk.text.as_bytes().get_unchecked(rel..) } } else { &[] };

            if bytes.is_empty() {
                if self.offset >= self.source.length() {
                    break;
                }
                self.chunk = self.source.chunk_at(self.offset);
                continue;
            }

            let skipped = SimdScanner::skip_ascii_digits(bytes);
            self.offset += skipped;

            if skipped < bytes.len() || self.offset >= self.source.length() {
                break;
            }
        }
        Range { start, end: self.offset }
    }

    /// Skips ASCII identifier continue characters using SIMD if possible.
    pub fn skip_ascii_ident_continue(&mut self) -> Range<usize> {
        let start = self.offset;
        loop {
            self.ensure_chunk();
            let rel = self.offset.saturating_sub(self.chunk.start);
            let bytes = if rel < self.chunk.text.len() { unsafe { self.chunk.text.as_bytes().get_unchecked(rel..) } } else { &[] };

            if bytes.is_empty() {
                if self.offset >= self.source.length() {
                    break;
                }
                self.chunk = self.source.chunk_at(self.offset);
                continue;
            }

            let skipped = SimdScanner::skip_ascii_ident_continue(bytes);
            self.offset += skipped;

            if skipped < bytes.len() || self.offset >= self.source.length() {
                break;
            }
        }
        Range { start, end: self.offset }
    }

    /// Skips until the specified byte is found.
    pub fn skip_until(&mut self, target: u8) -> Range<usize> {
        let start = self.offset;
        loop {
            self.ensure_chunk();
            let rel = self.offset.saturating_sub(self.chunk.start);
            let bytes = if rel < self.chunk.text.len() { unsafe { self.chunk.text.as_bytes().get_unchecked(rel..) } } else { &[] };

            if bytes.is_empty() {
                if self.offset >= self.source.length() {
                    break;
                }
                self.chunk = self.source.chunk_at(self.offset);
                continue;
            }

            let skipped = SimdScanner::skip_until(bytes, target);
            self.offset += skipped;

            if skipped < bytes.len() || self.offset >= self.source.length() {
                break;
            }
        }
        Range { start, end: self.offset }
    }

    /// Peeks at the next byte without advancing. the cursor.
    #[inline(always)]
    pub fn peek_byte(&mut self) -> Option<u8> {
        if self.offset >= self.chunk.start {
            let rel = self.offset - self.chunk.start;
            let bytes = self.chunk.text.as_bytes();
            if rel < bytes.len() {
                return Some(unsafe { *bytes.get_unchecked(rel) });
            }
        }
        self.ensure_chunk();
        let rel = self.offset - self.chunk.start;
        let bytes = self.chunk.text.as_bytes();
        bytes.get(rel).copied()
    }

    /// Advances the cursor by the specified number of bytes.
    pub fn advance_bytes(&mut self, len: usize) -> usize {
        self.offset = (self.offset + len).min(self.source.length());
        self.offset
    }

    /// Advances the cursor by one character and returns it.
    pub fn advance_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.advance_bytes(ch.len_utf8());
        Some(ch)
    }

    /// Advances the cursor by one byte and returns it.
    #[inline(always)]
    pub fn advance_byte(&mut self) -> Option<u8> {
        let b = self.peek_byte()?;
        self.offset += 1;
        Some(b)
    }

    /// Advances the cursor while the predicate is true and returns the range.
    pub fn take_while(&mut self, mut pred: impl FnMut(char) -> bool) -> Range<usize> {
        let start = self.offset;

        loop {
            // Ensure we have a valid chunk for current offset
            self.ensure_chunk();

            // Get text slice from current offset
            let rel = self.offset.saturating_sub(self.chunk.start);
            let text = if rel < self.chunk.text.len() { unsafe { self.chunk.text.get_unchecked(rel..) } } else { "" };

            if text.is_empty() {
                // If text is empty, it means we are at the end of the chunk (or source).
                // If we are at the end of source, break.
                if self.offset >= self.source.length() {
                    break;
                }
                // Otherwise force move to next chunk
                self.chunk = self.source.chunk_at(self.offset);
                // Continue loop to process next chunk
                continue;
            }

            let mut advanced = 0;
            let mut stop = false;

            // Iterate over characters in the current chunk slice
            for (i, ch) in text.char_indices() {
                if !pred(ch) {
                    advanced = i;
                    stop = true;
                    break;
                }
                advanced = i + ch.len_utf8()
            }

            self.offset += advanced;

            if stop {
                break;
            }

            // If we consumed the whole chunk but didn't stop, we need to check if we are at EOF
            if self.offset >= self.source.length() {
                break;
            }
            // If not at EOF, the loop will continue, ensure_chunk will get the next chunk
        }

        Range { start, end: self.offset }
    }

    /// Advances the cursor while the byte predicate is true and returns the range.
    #[inline(always)]
    pub fn take_while_byte(&mut self, mut pred: impl FnMut(u8) -> bool) -> Range<usize> {
        let start = self.offset;

        loop {
            self.ensure_chunk();
            let rel = self.offset.saturating_sub(self.chunk.start);
            let bytes = if rel < self.chunk.text.len() { unsafe { self.chunk.text.as_bytes().get_unchecked(rel..) } } else { &[] };

            if bytes.is_empty() {
                if self.offset >= self.source.length() {
                    break;
                }
                self.chunk = self.source.chunk_at(self.offset);
                continue;
            }

            let mut advanced = 0;
            let mut stop = false;

            for (i, &b) in bytes.iter().enumerate() {
                if !pred(b) {
                    advanced = i;
                    stop = true;
                    break;
                }
                advanced = i + 1
            }

            self.offset += advanced;
            if stop || self.offset >= self.source.length() {
                break;
            }
        }

        Range { start, end: self.offset }
    }

    /// Returns `true` if the source text at the current position starts with the given pattern.
    pub fn starts_with(&mut self, pattern: &str) -> bool {
        self.ensure_chunk();
        let chunk_text = self.chunk.text;
        let offset_in_chunk = self.offset.saturating_sub(self.chunk.start);

        // Ensure offset_in_chunk is on a character boundary
        let rest = if chunk_text.is_char_boundary(offset_in_chunk) {
            chunk_text.get(offset_in_chunk..).unwrap_or("")
        }
        else {
            // If not on a boundary, try to find the next boundary
            let mut i = offset_in_chunk;
            while i < chunk_text.len() && !chunk_text.is_char_boundary(i) {
                i += 1
            }
            chunk_text.get(i..).unwrap_or("")
        };

        if rest.len() >= pattern.len() {
            return rest.starts_with(pattern);
        }

        self.scratch.clear();
        self.scratch.push_str(rest);
        let mut next = self.chunk.end();
        let end = self.source.length();
        while self.scratch.len() < pattern.len() && next < end {
            let chunk = self.source.chunk_at(next);
            self.scratch.push_str(chunk.text);
            next = chunk.end()
        }
        self.scratch.starts_with(pattern)
    }

    /// Consumes the given pattern if it matches at the current position.
    pub fn consume_if_starts_with(&mut self, pattern: &str) -> bool {
        if !self.starts_with(pattern) {
            return false;
        }
        self.advance_bytes(pattern.len());
        true
    }

    /// Finds the first occurrence of the given pattern in the source text starting from the current position.
    pub fn find_str(&mut self, pattern: &str) -> Option<usize> {
        if pattern.is_empty() {
            return Some(self.offset);
        }

        let pat_len = pattern.len();
        let mut offset = self.offset;
        let end = self.source.length();
        while offset < end {
            self.offset = offset;
            self.ensure_chunk();
            let text = self.chunk.slice_from(offset);
            if let Some(pos) = text.find(pattern) {
                return Some(offset + pos);
            }
            let chunk_end = self.chunk.end();
            if chunk_end >= end {
                return None;
            }

            if pat_len > 1 {
                let keep = pat_len - 1;
                self.scratch.clear();
                let tail = text.get(text.len().saturating_sub(keep)..).unwrap_or("");
                self.scratch.push_str(tail);
                let tail_abs_start = chunk_end.saturating_sub(tail.len());
                let next_chunk = self.source.chunk_at(chunk_end);
                self.scratch.push_str(next_chunk.text);
                if let Some(pos) = self.scratch.find(pattern) {
                    return Some(tail_abs_start + pos);
                }
            }

            offset = chunk_end
        }
        None
    }
}

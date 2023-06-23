use crate::source::{Source, SourceId, TextChunk, TextEdit};
use core::range::Range;
use std::borrow::Cow;
use triomphe::Arc;

const CHUNK_SIZE: usize = 4096;

/// A read-only, rope-based source implementation for efficient handling of large files.
#[derive(Clone, Debug)]
pub struct RopeSource {
    source_id: Option<SourceId>,
    chunks: Arc<[Arc<str>]>,
    starts: Arc<[usize]>,
    len: usize,
}

/// A mutable buffer for rope-based source code, supporting efficient edits.
#[derive(Clone, Debug)]
pub struct RopeBuffer {
    source_id: Option<SourceId>,
    chunks: Vec<Arc<str>>,
    starts: Vec<usize>,
    len: usize,
}

impl RopeBuffer {
    /// Creates a new empty `RopeBuffer`.
    pub fn new(input: impl ToString) -> Self {
        Self::new_with_id(input, None)
    }

    /// Creates a new `RopeBuffer` with the specified input and source ID.
    pub fn new_with_id(input: impl ToString, source_id: impl Into<Option<SourceId>>) -> Self {
        let source_id = source_id.into();
        let text = input.to_string();
        let chunks = chunkify(&text);
        let (starts, len) = rebuild_starts(&chunks);
        Self { source_id, chunks, starts, len }
    }

    /// Returns a read-only snapshot of the current buffer.
    pub fn snapshot(&self) -> RopeSource {
        RopeSource { source_id: self.source_id, chunks: Arc::<[Arc<str>]>::from(self.chunks.clone()), starts: Arc::<[usize]>::from(self.starts.clone()), len: self.len }
    }

    /// Applies multiple text edits to the buffer and returns the affected range.
    pub fn apply_edits_range(&mut self, edits: &[TextEdit]) -> Range<usize> {
        let old_len = self.len;
        if edits.is_empty() {
            return Range { start: old_len, end: old_len };
        }

        let mut order: Vec<usize> = (0..edits.len()).collect();
        order.sort_by_key(|&i| edits[i].span.start);

        let mut reparse_from = old_len;
        let mut reparse_to = 0;
        let mut delta: isize = 0;
        for &i in &order {
            let TextEdit { span, text } = &edits[i];
            reparse_from = reparse_from.min(span.start);
            let start_new = (span.start as isize + delta) as usize;
            let end_new = start_new + text.len();
            reparse_to = reparse_to.max(end_new);
            delta += text.len() as isize - (span.end - span.start) as isize
        }

        for &i in order.iter().rev() {
            let TextEdit { span, text } = &edits[i];
            self.replace_range(Range { start: span.start, end: span.end }, text)
        }

        Range { start: reparse_from, end: reparse_to }
    }

    /// Applies multiple text edits to the buffer and returns the minimum affected offset.
    pub fn apply_edits(&mut self, edits: &[TextEdit]) -> usize {
        self.apply_edits_range(edits).start
    }

    fn replace_range(&mut self, span: Range<usize>, replacement: &str) {
        let start = span.start.min(self.len);
        let end = span.end.min(self.len).max(start);

        self.split_at(start);
        self.split_at(end);

        let start_idx = self.boundary_index(start);
        let end_idx = self.boundary_index(end);
        if start_idx < end_idx {
            self.chunks.drain(start_idx..end_idx);
        }

        if !replacement.is_empty() {
            let new_chunks = chunkify(replacement);
            let insert_at = start_idx.min(self.chunks.len());
            self.chunks.splice(insert_at..insert_at, new_chunks);
        }

        let (starts, len) = rebuild_starts(&self.chunks);
        self.starts = starts;
        self.len = len;
    }

    fn split_at(&mut self, offset: usize) {
        if offset == 0 || offset >= self.len {
            return;
        }
        let (idx, start) = self.chunk_index(offset);
        let rel = offset - start;
        let s = self.chunks[idx].clone();
        let s = s.as_ref();
        if rel == 0 || rel >= s.len() {
            return;
        }
        let Some(left) = s.get(..rel)
        else {
            return;
        };
        let Some(right) = s.get(rel..)
        else {
            return;
        };
        if left.is_empty() || right.is_empty() {
            return;
        }
        self.chunks[idx] = Arc::<str>::from(left.to_string());
        self.chunks.insert(idx + 1, Arc::<str>::from(right.to_string()));
        let (starts, len) = rebuild_starts(&self.chunks);
        self.starts = starts;
        self.len = len;
    }

    fn boundary_index(&self, offset: usize) -> usize {
        if offset >= self.len {
            return self.chunks.len();
        }
        match self.starts.binary_search(&offset) {
            Ok(i) => i,
            Err(i) => i,
        }
    }

    fn chunk_index(&self, offset: usize) -> (usize, usize) {
        let offset = offset.min(self.len.saturating_sub(1));
        match self.starts.binary_search(&offset) {
            Ok(i) => (i, self.starts[i]),
            Err(0) => (0, 0),
            Err(i) => (i - 1, self.starts[i - 1]),
        }
    }
}

impl Source for RopeBuffer {
    fn length(&self) -> usize {
        self.len
    }

    fn chunk_at(&self, offset: usize) -> TextChunk<'_> {
        if offset >= self.len {
            return TextChunk { start: self.len, text: "" };
        }
        let (idx, start) = self.chunk_index(offset);
        TextChunk { start, text: self.chunks[idx].as_ref() }
    }

    fn get_text_in(&self, range: Range<usize>) -> Cow<'_, str> {
        text_in_chunks(&self.chunks, &self.starts, self.len, range)
    }

    fn source_id(&self) -> Option<SourceId> {
        self.source_id
    }
}

impl Source for RopeSource {
    fn length(&self) -> usize {
        self.len
    }

    fn chunk_at(&self, offset: usize) -> TextChunk<'_> {
        if offset >= self.len {
            return TextChunk { start: self.len, text: "" };
        }
        let idx = match self.starts.binary_search(&offset) {
            Ok(i) => i,
            Err(0) => 0,
            Err(i) => i - 1,
        };
        let start = self.starts[idx];
        TextChunk { start, text: self.chunks[idx].as_ref() }
    }

    fn get_text_in(&self, range: Range<usize>) -> Cow<'_, str> {
        text_in_chunks(&self.chunks, &self.starts, self.len, range)
    }

    fn source_id(&self) -> Option<SourceId> {
        self.source_id
    }
}

fn rebuild_starts(chunks: &[Arc<str>]) -> (Vec<usize>, usize) {
    let mut starts = Vec::with_capacity(chunks.len());
    let mut offset = 0usize;
    for c in chunks {
        starts.push(offset);
        offset += c.len()
    }
    (starts, offset)
}

fn chunkify(text: &str) -> Vec<Arc<str>> {
    if text.is_empty() {
        return vec![];
    }
    let mut out = Vec::new();
    let mut start = 0usize;
    while start < text.len() {
        let mut end = (start + CHUNK_SIZE).min(text.len());
        while end > start && !text.is_char_boundary(end) {
            end -= 1
        }
        if end == start {
            end = text.len()
        }
        let part = &text[start..end];
        out.push(Arc::<str>::from(part.to_string()));
        start = end
    }
    out
}

fn text_in_chunks<'a>(chunks: &'a [Arc<str>], starts: &'a [usize], len: usize, range: Range<usize>) -> Cow<'a, str> {
    if range.start >= range.end || range.start >= len {
        return Cow::Borrowed("");
    }
    let start = range.start;
    let end = range.end.min(len);

    let start_idx = match starts.binary_search(&start) {
        Ok(i) => i,
        Err(0) => 0,
        Err(i) => i - 1,
    };
    let end_idx = match starts.binary_search(&end) {
        Ok(i) => i,
        Err(0) => 0,
        Err(i) => i - 1,
    };

    if start_idx == end_idx {
        let base = starts[start_idx];
        let rel_start = start - base;
        let rel_end = end - base;
        let s = chunks[start_idx].as_ref();
        return s.get(rel_start..rel_end).map(Cow::Borrowed).unwrap_or(Cow::Borrowed(""));
    }

    let mut buf = String::new();
    for (i, c) in chunks.iter().enumerate().skip(start_idx).take(end_idx - start_idx + 1) {
        let base = starts[i];
        let cs = c.as_ref();
        let seg_start = if i == start_idx { start.saturating_sub(base) } else { 0 };
        let seg_end = if i == end_idx { end.saturating_sub(base) } else { cs.len() };
        if let Some(seg) = cs.get(seg_start..seg_end) {
            buf.push_str(seg);
        }
    }
    Cow::Owned(buf)
}

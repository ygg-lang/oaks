use crate::source::{Source, SourceId, TextChunk};
use core::range::Range;
use std::borrow::Cow;
use triomphe::Arc;

const CHUNK_SIZE: usize = 4096;

/// A read-only, chunked source implementation for efficient handling of streamed or large files.
#[derive(Clone, Debug)]
pub struct ChunkedSource {
    source_id: Option<SourceId>,
    chunks: Arc<[Arc<str>]>,
    starts: Arc<[usize]>,
    len: usize,
}

/// A mutable buffer for chunked source code, supporting efficient appending of text.
#[derive(Clone, Debug, Default)]
pub struct ChunkedBuffer {
    source_id: Option<SourceId>,
    chunks: Vec<Arc<str>>,
    starts: Vec<usize>,
    len: usize,
}

impl ChunkedBuffer {
    /// Creates a new empty `ChunkedBuffer`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `ChunkedBuffer` with the specified source ID.
    pub fn new_with_id(source_id: impl Into<Option<SourceId>>) -> Self {
        Self { source_id: source_id.into(), ..Self::default() }
    }

    /// Appends the specified string to the end of the buffer.
    pub fn push_str(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }
        for chunk in chunkify(text) {
            self.starts.push(self.len);
            self.len += chunk.len();
            self.chunks.push(chunk)
        }
    }

    /// Returns a read-only snapshot of the current buffer.
    pub fn snapshot(&self) -> ChunkedSource {
        ChunkedSource { source_id: self.source_id, chunks: Arc::<[Arc<str>]>::from(self.chunks.clone()), starts: Arc::<[usize]>::from(self.starts.clone()), len: self.len }
    }
}

impl Source for ChunkedBuffer {
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

impl Source for ChunkedSource {
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

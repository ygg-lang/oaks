#![feature(new_range_api)]

use core::range::Range;
use oak_core::source::{ChunkedBuffer, RopeBuffer, Source, SourceCursor, TextEdit};

#[test]
fn rope_apply_edits_range_offsets() {
    let mut source = RopeBuffer::new("abcdef");
    let edits = vec![TextEdit { span: Range { start: 1, end: 2 }, text: "XX".to_string().into() }, TextEdit { span: Range { start: 4, end: 5 }, text: "".to_string().into() }];

    let range = source.apply_edits_range(&edits);
    assert_eq!(range.start, 1);
    assert_eq!(range.end, 5);
    let full = source.get_text_in(Range { start: 0, end: source.length() });
    assert_eq!(full.as_ref(), "aXXcdf")
}

#[test]
fn rope_cursor_starts_with_cross_chunk() {
    let text = format!("{}b", "a".repeat(4096));
    let source = RopeBuffer::new(text).snapshot();
    let mut cursor = SourceCursor::new_at(&source, 4095);
    assert!(cursor.starts_with("ab"))
}

#[test]
fn streaming_append_and_find() {
    let mut buf = ChunkedBuffer::new();
    buf.push_str("hello");
    buf.push_str(" world");
    let source = buf.snapshot();
    assert_eq!(source.length(), 11);

    let full = source.get_text_in(Range { start: 0, end: source.length() });
    assert_eq!(full.as_ref(), "hello world");

    let mut cursor = SourceCursor::new(&source);
    assert_eq!(cursor.find_str("world"), Some(6))
}

//! Unicode 转义模式测试 - Unicode 转义序列处理测试
//!
//! 测试 Unicode 转义序列的处理，包括：
//! - Unicode 转义解码
//! - 偏移量映射
//! - 位置转换

use lsp_types::Position;
use oak_core::source::{Source, SourceText};
use std::range::Range;

/// Source implementation for handling Unicode escape sequences.
///
/// This source automatically decodes Unicode escape sequences like `\u{1F600}`
/// while maintaining proper offset mapping between the escaped and unescaped text.
pub struct UnicodeEscapeSource<'a> {
    /// The underlying source text (with escapes)
    source: &'a SourceText,
    /// The decoded text (without escapes)
    decoded_text: String,
    /// Mapping from decoded offsets to original offsets
    offset_mapping: Vec<usize>,
}

impl<'a> UnicodeEscapeSource<'a> {
    /// Create a new UnicodeEscapeSource from the given source text.
    pub fn new(source: &'a SourceText) -> Self {
        let full_text = source.get_text_in((0..source.len()).into());
        let (decoded_text, offset_mapping) = Self::decode_unicode_escapes(full_text);

        Self { source, decoded_text, offset_mapping }
    }

    /// Decode Unicode escape sequences and create offset mapping.
    fn decode_unicode_escapes(text: &str) -> (String, Vec<usize>) {
        let mut decoded = String::new();
        let mut offset_mapping = Vec::new();
        let mut chars = text.chars().peekable();
        let mut original_offset = 0;

        while let Some(ch) = chars.next() {
            if ch == '\\' {
                let seq_start = original_offset;
                original_offset += 1; // consumed '\\'
                match chars.peek().copied() {
                    Some('u') => {
                        chars.next(); // consume 'u'
                        original_offset += 1;
                        if chars.peek() == Some(&'{') {
                            // Handle \u{XXXX...}
                            chars.next(); // consume '{'
                            original_offset += 1;
                            let mut hex_digits = String::new();
                            while let Some(d) = chars.peek().copied() {
                                if d == '}' {
                                    break;
                                }
                                hex_digits.push(d);
                                chars.next();
                                original_offset += 1;
                            }
                            if chars.peek() == Some(&'}') {
                                chars.next(); // consume '}'
                                original_offset += 1;
                            }
                            if let Ok(code_point) = u32::from_str_radix(&hex_digits, 16) {
                                if let Some(unicode_char) = char::from_u32(code_point) {
                                    decoded.push(unicode_char);
                                    offset_mapping.push(seq_start);
                                }
                                else {
                                    // invalid code point, keep original
                                    let literal = format!("\\u{{{}}}", hex_digits);
                                    decoded.push_str(&literal);
                                    for _ in 0..literal.len() {
                                        offset_mapping.push(seq_start);
                                    }
                                }
                            }
                            else {
                                // invalid hex, keep original
                                let literal = format!("\\u{{{}}}", hex_digits);
                                decoded.push_str(&literal);
                                for _ in 0..literal.len() {
                                    offset_mapping.push(seq_start);
                                }
                            }
                        }
                        else {
                            // Handle classic \uXXXX (exactly 4 hex digits)
                            let mut hex_digits = String::new();
                            for _ in 0..4 {
                                if let Some(d) = chars.peek().copied() {
                                    hex_digits.push(d);
                                    chars.next();
                                    original_offset += 1;
                                }
                                else {
                                    break;
                                }
                            }
                            if hex_digits.len() == 4 {
                                if let Ok(code_point) = u32::from_str_radix(&hex_digits, 16) {
                                    if let Some(unicode_char) = char::from_u32(code_point) {
                                        decoded.push(unicode_char);
                                        offset_mapping.push(seq_start);
                                    }
                                    else {
                                        let literal = format!("\\u{}", hex_digits);
                                        decoded.push_str(&literal);
                                        for _ in 0..literal.len() {
                                            offset_mapping.push(seq_start);
                                        }
                                    }
                                }
                                else {
                                    let literal = format!("\\u{}", hex_digits);
                                    decoded.push_str(&literal);
                                    for _ in 0..literal.len() {
                                        offset_mapping.push(seq_start);
                                    }
                                }
                            }
                            else {
                                // insufficient digits, keep as literal
                                let literal = format!("\\u{}", hex_digits);
                                decoded.push_str(&literal);
                                for _ in 0..literal.len() {
                                    offset_mapping.push(seq_start);
                                }
                            }
                        }
                    }
                    Some('n') => {
                        chars.next(); // consume 'n'
                        original_offset += 1;
                        decoded.push('\n');
                        offset_mapping.push(seq_start);
                    }
                    Some('t') => {
                        chars.next();
                        original_offset += 1;
                        decoded.push('\t');
                        offset_mapping.push(seq_start);
                    }
                    Some('r') => {
                        chars.next();
                        original_offset += 1;
                        decoded.push('\r');
                        offset_mapping.push(seq_start);
                    }
                    Some(other) => {
                        // Unrecognized escape, keep as literal
                        chars.next();
                        original_offset += 1;
                        decoded.push('\\');
                        offset_mapping.push(seq_start);
                        decoded.push(other);
                        offset_mapping.push(seq_start);
                    }
                    None => {
                        // stray backslash at end
                        decoded.push('\\');
                        offset_mapping.push(seq_start);
                    }
                }
            }
            else {
                decoded.push(ch);
                offset_mapping.push(original_offset);
                original_offset += ch.len_utf8();
            }
        }

        (decoded, offset_mapping)
    }

    /// Convert a decoded offset back to the original offset.
    fn decoded_to_original(&self, decoded_offset: usize) -> usize {
        if decoded_offset >= self.offset_mapping.len() { self.source.length() } else { self.offset_mapping[decoded_offset] }
    }

    // 将字符索引映射为字节偏移位置列表，末尾追加字符串总字节长度
    fn char_positions(&self) -> Vec<usize> {
        let mut positions = Vec::with_capacity(self.decoded_text.chars().count() + 1);
        for (byte_idx, _) in self.decoded_text.char_indices() {
            positions.push(byte_idx);
        }
        positions.push(self.decoded_text.len());
        positions
    }
}

impl<'a> Source for UnicodeEscapeSource<'a> {
    fn length(&self) -> usize {
        // 返回字符数量而非字节长度，确保偏移基于字符
        self.decoded_text.chars().count()
    }

    fn get_text_in(&self, range: Range<usize>) -> &str {
        // 将字符范围转换为安全的字节范围，避免 UTF-8 边界错误
        let positions = self.char_positions();
        let start = *positions.get(range.start).unwrap_or(&self.decoded_text.len());
        let end = *positions.get(range.end).unwrap_or(&self.decoded_text.len());
        &self.decoded_text[start..end]
    }

    fn offset_to_position(&self, offset: usize) -> Position {
        let original_offset = self.decoded_to_original(offset);
        self.source.offset_to_position(original_offset)
    }

    fn position_to_offset(&self, position: Position) -> usize {
        // This is a simplified implementation
        // In a full implementation, we'd need reverse mapping
        let original_offset = self.source.position_to_offset(position);
        // Find the closest decoded offset
        for (decoded_offset, &orig_offset) in self.offset_mapping.iter().enumerate() {
            if orig_offset >= original_offset {
                return decoded_offset;
            }
        }
        self.decoded_text.chars().count()
    }
}

fn create_tsource() -> SourceText {
    let text = "Hello \\u{1F600} World\\nUnicode: \\u{4E16}\\u{754C}";
    SourceText::new(text)
}

#[test]
fn test_unicode_escape_decoding() {
    let source = create_tsource();
    let unicode_source = UnicodeEscapeSource::new(&source);

    // 测试解码后的文本
    let decoded = unicode_source.get_text_in((0..unicode_source.length()).into());
    assert!(decoded.contains("😀")); // \u{1F600} -> 😀
    assert!(decoded.contains("世")); // \u{4E16} -> 世
    assert!(decoded.contains("界")); // \u{754C} -> 界
}

#[test]
fn test_unicode_escape_length() {
    let source = create_tsource();
    let unicode_source = UnicodeEscapeSource::new(&source);

    // 解码后的长度应该比原始长度短（因为转义序列被替换为单个字符）
    assert!(unicode_source.length() < source.len());
}

#[test]
fn test_unicode_char_at() {
    let source = SourceText::new("\\u0041\\u00E9\\u4E16");
    let unicode_source = UnicodeEscapeSource::new(&source);

    // 测试字符访问
    assert_eq!(unicode_source.get_char_at(0), Some('A'));
    assert_eq!(unicode_source.get_char_at(1), Some('é'));
    assert_eq!(unicode_source.get_char_at(2), Some('世'));
}

#[test]
fn test_unicode_escape_offset_mapping() {
    let source = SourceText::new("Hello \\u{1F600} World");
    let unicode_source = UnicodeEscapeSource::new(&source);

    // 测试位置映射
    let pos = unicode_source.offset_to_position(6); // 😀 的位置
    assert_eq!(pos.line, 0);
    assert_eq!(pos.character, 6);
}

#[test]
fn test_invalid_unicode_escape() {
    let source = SourceText::new("Invalid \\u{ZZZZ} escape");
    let unicode_source = UnicodeEscapeSource::new(&source);

    // 无效的转义序列应该保持原样
    let text = unicode_source.get_text_in((0..unicode_source.length()).into());
    assert!(text.contains("\\u{ZZZZ}"));
}

#[test]
fn test_mixed_content() {
    let source = SourceText::new("Text \\u{1F600} emoji \\u{4E16}中文\\nnewline");
    let unicode_source = UnicodeEscapeSource::new(&source);

    let decoded = unicode_source.get_text_in((0..unicode_source.length()).into());
    assert!(decoded.contains("😀"));
    assert!(decoded.contains("世"));
    assert!(decoded.contains("\n"));
}

#[test]
fn test_unicode_escape_error() {
    let source = SourceText::new("Error at \\u{1F600} here");
    let unicode_source = UnicodeEscapeSource::new(&source);

    let error = unicode_source.syntax_error("Unicode 错误", 10);
    assert!(error.to_string().contains("Unicode 错误"));
}

#[test]
fn test_position_to_offset_conversion() {
    let source = SourceText::new("\\u{1F600}\\u{4E16}\\u{754C}");
    let unicode_source = UnicodeEscapeSource::new(&source);

    // 测试位置到偏移的转换
    let offset = unicode_source.position_to_offset(Position { line: 0, character: 1 });
    assert_eq!(offset, 1); // 第二个字符（世）的偏移量
}

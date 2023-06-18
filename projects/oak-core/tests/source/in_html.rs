//! HTML 实体模式测试 - HTML 实体处理测试
//!
//! 测试 HTML 实体（如 &amp;, &lt;, &gt; 等）的处理，包括：
//! - HTML 实体解码
//! - 偏移量映射
//! - 位置转换

use lsp_types::Position;
use oak_core::source::{Source, SourceText};
use std::range::Range;

/// Source implementation for handling HTML entities.
///
/// This source automatically decodes HTML entities like `&amp;`, `&lt;`, `&gt;`
/// while maintaining proper offset mapping between the escaped and unescaped text.
pub struct HtmlEntitySource<'a> {
    /// The underlying source text (with entities)
    source: &'a SourceText,
    /// The decoded text (without entities)
    decoded_text: String,
    /// Mapping from decoded offsets to original offsets
    offset_mapping: Vec<usize>,
}

impl<'a> HtmlEntitySource<'a> {
    /// Create a new HtmlEntitySource from the given source text.
    pub fn new(source: &'a SourceText) -> Self {
        let full_text = source.get_text_in((0..source.len()).into());
        let (decoded_text, offset_mapping) = Self::decode_html_entities(full_text);

        Self { source, decoded_text, offset_mapping }
    }

    /// Decode HTML entities and create offset mapping.
    fn decode_html_entities(text: &str) -> (String, Vec<usize>) {
        let mut decoded = String::new();
        let mut offset_mapping = Vec::new();
        let mut chars = text.chars().peekable();
        let mut original_offset = 0;

        while let Some(ch) = chars.next() {
            if ch == '&' {
                let mut entity = String::new();
                entity.push('&');
                let entity_start_offset = original_offset;

                // Read entity name
                while let Some(entity_ch) = chars.peek() {
                    if *entity_ch == ';' {
                        chars.next(); // consume ';'
                        entity.push(';');
                        original_offset += 1;
                        break;
                    }
                    if entity_ch.is_whitespace() || *entity_ch == '&' {
                        // Invalid entity, treat as regular text
                        break;
                    }
                    entity.push(*entity_ch);
                    chars.next();
                    original_offset += 1;
                }

                // Try to decode the entity
                let decoded_char = match entity.as_str() {
                    "&amp;" => Some('&'),
                    "&lt;" => Some('<'),
                    "&gt;" => Some('>'),
                    "&quot;" => Some('"'),
                    "&apos;" => Some('\''),
                    "&nbsp;" => Some(' '), // Non-breaking space
                    "&copy;" => Some('©'),
                    "&reg;" => Some('®'),
                    "&trade;" => Some('™'),
                    "&hellip;" => Some('…'),
                    "&mdash;" => Some('—'),
                    "&ndash;" => Some('–'),
                    _ => {
                        // Try numeric entities (decimal or hex)
                        if entity.starts_with("&#") && entity.ends_with(";") {
                            let num_part = &entity[2..entity.len() - 1];
                            let maybe_char = if let Some(first) = num_part.chars().next() {
                                if first == 'x' || first == 'X' {
                                    // Hex form: &#xNNNN;
                                    let hex_part = &num_part[1..];
                                    u32::from_str_radix(hex_part, 16).ok().and_then(char::from_u32)
                                }
                                else {
                                    // Decimal form: &#NNNN;
                                    num_part.parse::<u32>().ok().and_then(char::from_u32)
                                }
                            }
                            else {
                                None
                            };
                            maybe_char
                        }
                        else {
                            None
                        }
                    }
                };

                if let Some(decoded_ch) = decoded_char {
                    decoded.push(decoded_ch);
                    offset_mapping.push(entity_start_offset);
                    original_offset += 1;
                }
                else {
                    // Invalid entity, keep original
                    decoded.push_str(&entity);
                    for _ in 0..entity.len() {
                        offset_mapping.push(entity_start_offset);
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
}

impl<'a> Source for HtmlEntitySource<'a> {
    fn length(&self) -> usize {
        self.decoded_text.len()
    }

    fn get_text_in(&self, range: Range<usize>) -> &str {
        &self.decoded_text[range.start..range.end]
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
        self.decoded_text.len()
    }
}

fn create_source() -> SourceText {
    SourceText::new("Hello &amp; &lt;World&gt; &quot;Test&quot; &copy; 2024&#33;")
}

#[test]
fn test_html_entity_decoding() {
    let source = create_source();
    let html_source = HtmlEntitySource::new(&source);

    // 测试解码后的文本
    let decoded = html_source.get_text_in((0..html_source.length()).into());
    assert!(decoded.contains("&")); // &amp; -> &
    assert!(decoded.contains("<")); // &lt; -> <
    assert!(decoded.contains(">")); // &gt; -> >
    assert!(decoded.contains("\"")); // &quot; -> "
    assert!(decoded.contains("©")); // &copy; -> ©
    assert!(decoded.contains("!")); // &#33; -> !
}

#[test]
fn test_html_entity_length() {
    let source = create_source();
    let html_source = HtmlEntitySource::new(&source);

    // 解码后的长度应该比原始长度短（因为实体被替换为单个字符）
    assert!(html_source.length() < source.len());
}

#[test]
fn test_html_entity_char_at() {
    let source = SourceText::new("&amp;&lt;&gt;");
    let html_source = HtmlEntitySource::new(&source);

    // 测试字符访问
    assert_eq!(html_source.get_char_at(0), Some('&'));
    assert_eq!(html_source.get_char_at(1), Some('<'));
    assert_eq!(html_source.get_char_at(2), Some('>'));
}

#[test]
fn test_html_entity_offset_mapping() {
    let source = SourceText::new("Hello &amp; World");
    let html_source = HtmlEntitySource::new(&source);

    // 测试位置映射
    let pos = html_source.offset_to_position(6); // & 的位置
    assert_eq!(pos.line, 0);
    assert_eq!(pos.character, 6);
}

#[test]
fn test_invalid_html_entity() {
    let source = SourceText::new("Invalid &unknown; entity");
    let html_source = HtmlEntitySource::new(&source);

    // 无效的实体应该保持原样
    let text = html_source.get_text_in((0..html_source.length()).into());
    assert!(text.contains("&unknown;"));
}

#[test]
fn test_mixed_content() {
    let source = SourceText::new("Text &amp; entities &lt;mixed&gt; with &#65;SCII");
    let html_source = HtmlEntitySource::new(&source);

    let decoded = html_source.get_text_in((0..html_source.length()).into());
    assert!(decoded.contains("&"));
    assert!(decoded.contains("<"));
    assert!(decoded.contains(">"));
    assert!(decoded.contains("A")); // &#65; -> A
}

#[test]
fn test_html_entity_error() {
    let source = SourceText::new("Error at &amp; here");
    let html_source = HtmlEntitySource::new(&source);

    let error = html_source.syntax_error("HTML 实体错误", 10);
    assert!(error.to_string().contains("HTML 实体错误"));
}

#[test]
fn test_position_to_offset_conversion() {
    let source = SourceText::new("&amp;&lt;&gt;");
    let html_source = HtmlEntitySource::new(&source);

    // 测试位置到偏移的转换
    let offset = html_source.position_to_offset(Position { line: 0, character: 1 });
    assert_eq!(offset, 1); // 第二个字符（<）的偏移量
}

#[test]
fn test_numeric_entities() {
    let source = SourceText::new("&#65;&#8364;&#x4E16;");
    let html_source = HtmlEntitySource::new(&source);

    let decoded = html_source.get_text_in((0..html_source.length()).into());
    assert!(decoded.contains("A")); // &#65; -> A
    assert!(decoded.contains("€")); // &#8364; -> €
    assert!(decoded.contains("世")); // &#x4E16; -> 世
}

#[test]
fn test_html_entity_source() {
    let text = "Hello &amp; &lt;world&gt;!";
    let source = SourceText::new(text);
    let html_source = HtmlEntitySource::new(&source);

    assert_eq!(html_source.length(), 17); // "Hello & <world>!"
    assert_eq!(html_source.get_text_in((0..17).into()), "Hello & <world>!");
}

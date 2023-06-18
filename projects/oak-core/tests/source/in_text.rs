//! 文本模式测试 - 基础 SourceText 功能测试
//!
//! 测试基本的文本处理功能，包括：
//! - 基本的偏移量到位置转换
//! - 文本提取
//! - 字符访问
//! - 位置计算

use lsp_types::Position;
use oak_core::source::{Source, SourceText};

fn create_source() -> SourceText {
    SourceText::new("Hello, World!\nThis is a test.\nLine 3 here.")
}

#[test]
fn test_basic_length() {
    let source = create_source();
    assert_eq!((&source).length(), 42); // 包含换行符的总长度
}

#[test]
fn test_get_text_in() {
    let source = create_source();
    let sref = &source;

    // 测试基本文本提取
    let text = sref.get_text_in((0..5).into());
    assert_eq!(text, "Hello");

    // 测试跨行文本提取
    let cross_line = sref.get_text_in((14..29).into());
    assert_eq!(cross_line, "This is a test.");
}

#[test]
fn test_get_char_at() {
    let source = create_source();

    // 测试字符访问
    assert_eq!((&source).get_char_at(0), Some('H'));
    assert_eq!((&source).get_char_at(7), Some('W'));
    assert_eq!((&source).get_char_at(13), Some('\n'));
    assert_eq!((&source).get_char_at(100), None); // 超出范围
}

#[test]
fn test_offset_to_position() {
    let source = create_source();

    // 测试第一行
    let pos1 = (&source).offset_to_position(0);
    assert_eq!(pos1.line, 0);
    assert_eq!(pos1.character, 0);

    let pos2 = (&source).offset_to_position(7);
    assert_eq!(pos2.line, 0);
    assert_eq!(pos2.character, 7);

    // 测试第二行（在换行符之后）
    let pos3 = (&source).offset_to_position(14);
    assert_eq!(pos3.line, 1);
    assert_eq!(pos3.character, 0);

    // 测试第三行
    let pos4 = (&source).offset_to_position(30);
    assert_eq!(pos4.line, 2);
    assert_eq!(pos4.character, 0);
}

#[test]
fn test_position_to_offset() {
    let source = create_source();

    // 测试第一行
    assert_eq!((&source).position_to_offset(Position { line: 0, character: 0 }), 0);
    assert_eq!((&source).position_to_offset(Position { line: 0, character: 7 }), 7);

    // 测试第二行
    assert_eq!((&source).position_to_offset(Position { line: 1, character: 0 }), 14);
    assert_eq!((&source).position_to_offset(Position { line: 1, character: 5 }), 19);

    // 测试第三行
    assert_eq!((&source).position_to_offset(Position { line: 2, character: 0 }), 30);
}

#[test]
fn test_span_to_lsp_range() {
    let source = create_source();

    // 测试单行范围
    let range1 = (&source).span_to_lsp_range((0..5).into());
    assert_eq!(range1.start.line, 0);
    assert_eq!(range1.start.character, 0);
    assert_eq!(range1.end.line, 0);
    assert_eq!(range1.end.character, 5);

    // 测试跨行范围
    let range2 = (&source).span_to_lsp_range((7..20).into());
    assert_eq!(range2.start.line, 0);
    assert_eq!(range2.start.character, 7);
    assert_eq!(range2.end.line, 1);
    assert_eq!(range2.end.character, 6);
}

#[test]
fn test_find_char_from() {
    let source = create_source();

    // 测试字符查找
    assert_eq!((&source).find_char_from(0, 'W'), Some(7));
    assert_eq!((&source).find_char_from(10, 'i'), Some(16));
    assert_eq!((&source).find_char_from(0, 'x'), None); // 未找到
}

#[test]
fn test_find_str_from() {
    let source = create_source();

    // 测试字符串查找
    assert_eq!((&source).find_str_from(0, "World"), Some(7));
    assert_eq!((&source).find_str_from(0, "test"), Some(24));
    assert_eq!((&source).find_str_from(0, "xyz"), None); // 未找到
}

#[test]
fn test_create_error() {
    let source = create_source();

    // 测试错误创建
    let error = (&source).syntax_error("测试错误", 15);
    assert!(error.to_string().contains("测试错误"));
    assert!(error.to_string().contains("line 1"));
}

#[test]
fn test_empty_source() {
    let source = SourceText::default();
    assert_eq!((&source).length(), 0);
    assert!(source.is_empty());
    assert_eq!((&source).get_char_at(0), None);
    assert_eq!((&source).get_text_in((0..0).into()), "");
}

#[test]
fn test_unicode_handling() {
    let text = "Hello 世界\nUnicode: 🚀";
    let source = SourceText::new(text);

    // 测试Unicode字符处理
    let pos = (&source).offset_to_position(6); // "世" 的开始位置
    assert_eq!(pos.line, 0);
    assert_eq!(pos.character, 6);

    // 测试emoji字符
    let emoji_pos = (&source).offset_to_position(23); // 🚀 的开始位置
    assert_eq!(emoji_pos.line, 1);
    assert_eq!(emoji_pos.character, 9);
}

//! 视图模式测试 - SourceText 切片功能测试
//!
//! 测试 SourceText 的切片功能，包括：
//! - 创建子视图
//! - 视图内的位置映射
//! - 视图的文本操作

use lsp_types::Position;
use oak_core::source::{Source, SourceText};

fn create_test_source() -> SourceText {
    let text = "Line 1: Hello\nLine 2: World\nLine 3: Test";
    SourceText::new(text)
}

#[test]
fn test_source_view_creation() {
    let source = create_test_source();

    // 创建第一行的视图 (0 到 13)
    let view1 = source.view((0..13).into());
    assert_eq!(view1.length(), 13);
    assert_eq!(view1.get_text_in((0..13).into()), "Line 1: Hello");

    // 创建第二行的视图 (14 到 27)
    let view2 = source.view((14..27).into());
    assert_eq!(view2.length(), 13);
    assert_eq!(view2.get_text_in((0..13).into()), "Line 2: World");
}

#[test]
fn test_view_offset_to_position() {
    let source = create_test_source();
    let view = source.view((14..27).into()); // 第二行

    // 测试视图内的位置转换
    let pos0 = view.offset_to_position(0);
    assert_eq!(pos0.line, 1); // 原始文本中的行号（0-based 第二行）
    assert_eq!(pos0.character, 0);

    let pos5 = view.offset_to_position(5);
    assert_eq!(pos5.line, 1); // 原始文本中的行号（0-based 第二行）
    assert_eq!(pos5.character, 5);

    // 视图结束位置
    let pos_end = view.offset_to_position(12);
    assert_eq!(pos_end.line, 1); // 原始文本中的行号（0-based 第二行）
    assert_eq!(pos_end.character, 12);
}

#[test]
fn test_view_position_to_offset() {
    let source = create_test_source();
    let view = source.view((14..27).into()); // 第二行

    // 测试视图内的偏移转换（0-based 行号）
    assert_eq!(view.position_to_offset(Position { line: 1, character: 0 }), 0);
    assert_eq!(view.position_to_offset(Position { line: 1, character: 5 }), 5);
    assert_eq!(view.position_to_offset(Position { line: 1, character: 12 }), 12);
}

#[test]
fn test_view_get_char_at() {
    let source = create_test_source();
    let view = source.view((14..27).into()); // 第二行

    assert_eq!(view.get_char_at(0), Some('L'));
    assert_eq!(view.get_char_at(5), Some('2'));
    assert_eq!(view.get_char_at(8), Some('W'));
    assert_eq!(view.get_char_at(100), None); // 超出范围
}

#[test]
fn test_view_text_extraction() {
    let source = create_test_source();
    let view = source.view((14..27).into()); // 第二行

    // 提取子字符串 - 使用 get_text_in() 获取整个视图文本
    let view_text = view.get_text_in((0..13).into());
    assert_eq!(&view_text[0..5], "Line ");
    assert_eq!(&view_text[5..11], "2: Wor");
    assert_eq!(&view_text[8..13], "World");
}

#[test]
fn test_nested_views() {
    let source = create_test_source();

    // 先创建第二行的视图
    let line2_view = source.view((14..27).into());

    // 再在视图中创建子视图
    let sub_view = line2_view.view((5..11).into()); // "2: Wor"

    assert_eq!(sub_view.length(), 6);
    assert_eq!(sub_view.get_text_in((0..6).into()), "2: Wor");

    // 测试嵌套视图的位置转换
    let pos = sub_view.offset_to_position(2);
    assert_eq!(pos.line, 1); // 原始文本中的行号（0-based 第二行）
    assert_eq!(pos.character, 7); // 相对于原始文本的字符位置
}

#[test]
fn test_view_find_operations() {
    let source = create_test_source();
    let view = source.view((14..27).into()); // 第二行

    // 在视图中查找字符
    assert_eq!(view.find_char_from(0, 'W'), Some(8));
    assert_eq!(view.find_char_from(0, 'o'), Some(9));
    assert_eq!(view.find_char_from(0, 'x'), None); // 未找到

    // 在视图中查找字符串
    assert_eq!(view.find_str_from(0, "World"), Some(8));
    assert_eq!(view.find_str_from(0, "Line"), Some(0));
}

#[test]
fn test_view_error_creation() {
    let source = create_test_source();
    let view = source.view((14..27).into()); // 第二行

    // 在视图中创建错误
    let error = view.syntax_error("视图中的错误", 5);
    let error_msg = error.to_string();
    println!("Error message: {}", error_msg);

    assert!(error_msg.contains("视图中的错误"));
    // 错误位置应该相对于原始文本
    assert!(error_msg.contains("SourceLocation"));
    assert!(error_msg.contains("line: 1"));
    assert!(error_msg.contains("column: 5"));
    // 验证错误消息格式
    assert!(error_msg.contains("Syntax error at"));
}

#[test]
fn test_empty_view() {
    let source = create_test_source();
    let empty_view = source.view((0..0).into());

    assert_eq!(empty_view.length(), 0);
    assert!(empty_view.is_empty());
    assert_eq!(empty_view.get_char_at(0), None);
    assert_eq!(empty_view.get_text_in((0..0).into()), "");
}

#[test]
fn test_view_unicode_handling() {
    let text = "Hello 世界\nUnicode: 🚀 Test";
    let source = SourceText::new(text);
    let view = source.view((6..20).into()); // 包含Unicode字符的视图，扩大范围以包含完整文本

    println!("View length: {}", view.length());
    println!("View text: {}", view.get_text_in((0..view.length()).into()));

    assert_eq!(view.length(), 14);
    assert_eq!(view.get_text_in((0..14).into()), "世界\nUnicode");

    // 测试Unicode字符的位置
    // 注意：offset_to_position 返回的是 LSP Position，其中 character 是字符位置，不是字节偏移量
    let pos = view.offset_to_position(0); // 视图中的第一个字符 '世'
    println!("Position at offset 0: line={}, character={}", pos.line, pos.character);
    assert_eq!(pos.line, 0); // '世' 位于第一行（0-based）
    // character 应该是 6，因为 '世' 在原始文本中的字符位置是 6
    assert_eq!(pos.character, 6);

    // 测试换行符后的位置
    let pos2 = view.offset_to_position(7); // 视图中偏移量7对应的是 'U' 字符
    println!("Position at offset 7: line={}, character={}", pos2.line, pos2.character);
    assert_eq!(pos2.line, 1); // 'U' 在第二行（0-based）
    assert_eq!(pos2.character, 0); // 'U'是第三行的第一个字符

    // 测试换行符本身的位置
    let pos_newline = view.offset_to_position(6); // 视图中偏移量6对应的是换行符
    println!("Position at offset 6 (newline): line={}, character={}", pos_newline.line, pos_newline.character);
    assert_eq!(pos_newline.line, 0); // 换行符在第一行末尾（0-based）
}

//! JavaDoc 注释测试
//!
//! 测试 JavaDoc 注释的处理，包括：
//! - JavaDoc 标签解析（@param, @return, @throws 等）
//! - HTML 标签处理
//! - 文本内容提取
//! - 位置映射

use lsp_types::Position;
use oak_core::source::{Source, SourceText};
use std::range::Range;

/// JavaDoc 源包装器
///
/// 处理 JavaDoc 注释的特殊逻辑，包括：
/// - JavaDoc 标签识别
/// - HTML 内容处理
/// - 文本提取和清理
struct JavaDocSource<'a> {
    source: &'a SourceText,
    cleaned_text: String,
    tag_ranges: Vec<(Range<usize>, JavaDocTag)>,
}

/// JavaDoc 标签类型
#[derive(Debug, Clone, PartialEq)]
enum JavaDocTag {
    Param,
    Return,
    Throws,
    See,
    Since,
    Author,
    Version,
    Deprecated,
    Link,
    Code,
    Literal,
    Unknown(String),
}

impl<'a> JavaDocSource<'a> {
    /// 创建新的 JavaDoc 源
    fn new(source: &'a SourceText) -> Self {
        let text = source.get_text_in((0..source.length()).into());
        let (cleaned_text, tag_ranges) = Self::extract_javadoc_tags(&text);

        Self { source, cleaned_text, tag_ranges }
    }

    /// 提取 JavaDoc 标签
    fn extract_javadoc_tags(text: &str) -> (String, Vec<(Range<usize>, JavaDocTag)>) {
        let mut cleaned_text = String::new();
        let mut tag_ranges = Vec::new();
        let mut current_pos = 0;

        for (i, line) in text.lines().enumerate() {
            // 移除前导的 * 和空格
            let trimmed = line.trim_start_matches('*').trim_start();

            // 查找 @ 标签
            if let Some(at_pos) = trimmed.find('@') {
                let tag_start = at_pos + 1;
                let tag_end =
                    trimmed[tag_start..].find(|c: char| c.is_whitespace()).map(|pos| tag_start + pos).unwrap_or(trimmed.len());

                let tag_name = &trimmed[tag_start..tag_end];
                let tag_type = match tag_name {
                    "param" => JavaDocTag::Param,
                    "return" => JavaDocTag::Return,
                    "throws" | "exception" => JavaDocTag::Throws,
                    "see" => JavaDocTag::See,
                    "since" => JavaDocTag::Since,
                    "author" => JavaDocTag::Author,
                    "version" => JavaDocTag::Version,
                    "deprecated" => JavaDocTag::Deprecated,
                    "link" => JavaDocTag::Link,
                    "code" => JavaDocTag::Code,
                    "literal" => JavaDocTag::Literal,
                    other => JavaDocTag::Unknown(other.to_string()),
                };

                let global_start = current_pos + at_pos;
                let global_end = global_start + tag_name.len();
                tag_ranges.push(((global_start..global_end).into(), tag_type));
            }

            cleaned_text.push_str(trimmed);
            if i < text.lines().count() - 1 {
                cleaned_text.push('\n');
            }
            current_pos = cleaned_text.len();
        }

        (cleaned_text, tag_ranges)
    }

    /// 获取清理后的文本
    fn get_cleaned_text(&self) -> &str {
        &self.cleaned_text
    }

    /// 获取标签范围
    fn get_tag_ranges(&self) -> &[(Range<usize>, JavaDocTag)] {
        &self.tag_ranges
    }

    /// 获取参数标签数量
    fn get_param_count(&self) -> usize {
        self.tag_ranges.iter().filter(|(_, tag)| matches!(tag, JavaDocTag::Param)).count()
    }

    /// 是否有返回标签
    fn has_return_tag(&self) -> bool {
        self.tag_ranges.iter().any(|(_, tag)| matches!(tag, JavaDocTag::Return))
    }

    /// 是否有抛出异常标签
    fn has_throws_tag(&self) -> bool {
        self.tag_ranges.iter().any(|(_, tag)| matches!(tag, JavaDocTag::Throws))
    }
}

/// 创建测试源
fn create_test_source(text: &str) -> SourceText {
    SourceText::new(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_javadoc() {
        let text = r#"/**
         * This is a basic JavaDoc comment.
         * @param name The name parameter
         * @return The greeting message
         */
        "#;

        let source = create_test_source(text);
        let javadoc = JavaDocSource::new(&source);

        assert_eq!(javadoc.get_param_count(), 1);
        assert!(javadoc.has_return_tag());
        assert!(!javadoc.has_throws_tag());

        let cleaned = javadoc.get_cleaned_text();
        assert!(cleaned.contains("This is a basic JavaDoc comment."));
        assert!(!cleaned.contains("@param"));
    }

    #[test]
    fn test_multiple_params() {
        let text = r#"/**
         * Method with multiple parameters.
         * @param first First parameter
         * @param second Second parameter
         * @param third Third parameter
         * @return Result of the operation
         * @throws IllegalArgumentException if arguments are invalid
         */
        "#;

        let source = create_test_source(text);
        let javadoc = JavaDocSource::new(&source);

        assert_eq!(javadoc.get_param_count(), 3);
        assert!(javadoc.has_return_tag());
        assert!(javadoc.has_throws_tag());
    }

    #[test]
    fn test_html_in_javadoc() {
        let text = r#"/**
         * Method with <b>HTML</b> content.
         * <p>This is a paragraph with {@code code}.</p>
         * @param input The input parameter
         * @see OtherClass#otherMethod()
         */
        "#;

        let source = create_test_source(text);
        let javadoc = JavaDocSource::new(&source);

        assert_eq!(javadoc.get_param_count(), 1);

        let cleaned = javadoc.get_cleaned_text();
        assert!(cleaned.contains("Method with"));
        assert!(cleaned.contains("HTML"));
        assert!(cleaned.contains("content."));
    }

    #[test]
    fn test_inline_tags() {
        let text = r#"/**
         * Method with {@link SomeClass} and {@code someCode()}.
         * Also supports {@literal literal text} and {@value #CONSTANT}.
         * @deprecated This method is deprecated
         * @since 1.0
         */
        "#;

        let source = create_test_source(text);
        let javadoc = JavaDocSource::new(&source);

        let tag_ranges = javadoc.get_tag_ranges();
        assert!(tag_ranges.iter().any(|(_, tag)| matches!(tag, JavaDocTag::Deprecated)));
        assert!(tag_ranges.iter().any(|(_, tag)| matches!(tag, JavaDocTag::Since)));
    }

    #[test]
    fn test_javadoc_char_at() {
        let text = r#"/**
         * Hello JavaDoc world.
         * @param test Test parameter
         */
        "#;

        let source = create_test_source(text);
        let javadoc = JavaDocSource::new(&source);

        // 测试原始源文本的字符访问
        assert_eq!((&source).get_char_at(0), Some('/'));
        assert_eq!((&source).get_char_at(1), Some('*'));
        assert_eq!((&source).get_char_at(2), Some('*'));

        // 测试清理后的文本内容
        let cleaned = javadoc.get_cleaned_text();
        assert!(cleaned.starts_with("Hello JavaDoc world."));
    }

    #[test]
    fn test_javadoc_text_content() {
        let text = r#"/**
         * Calculate the area of a circle.
         * @param radius The radius of the circle
         * @return The area of the circle
         * @throws IllegalArgumentException if radius is negative
         */
        "#;

        let source = create_test_source(text);
        let javadoc = JavaDocSource::new(&source);

        // 测试文本内容提取
        let cleaned = javadoc.get_cleaned_text();
        assert!(cleaned.contains("Calculate the area of a circle."));
        assert!(cleaned.contains("radius The radius of the circle"));
        assert!(cleaned.contains("The area of the circle"));
        assert!(cleaned.contains("IllegalArgumentException if radius is negative"));

        // 验证标签解析
        assert_eq!(javadoc.get_param_count(), 1);
        assert!(javadoc.has_return_tag());
        assert!(javadoc.has_throws_tag());
    }

    #[test]
    fn test_complex_javadoc() {
        let text = r#"/**
         * Processes user input and returns a formatted result.
         * 
         * <p>This method performs the following operations:
         * <ul>
         *   <li>Validates input parameters</li>
         *   <li>Processes the data</li>
         *   <li>Returns formatted output</li>
         * </ul>
         * </p>
         * 
         * @param input The user input string
         * @param format The output format specification
         * @param options Processing options
         * @return The formatted result string
         * @throws ValidationException if input is invalid
         * @throws ProcessingException if processing fails
         * @see InputValidator
         * @see OutputFormatter
         * @since 2.0
         * @deprecated Use {@link #processInputV2} instead
         */
        "#;

        let source = create_test_source(text);
        let javadoc = JavaDocSource::new(&source);

        assert_eq!(javadoc.get_param_count(), 3);
        assert!(javadoc.has_return_tag());
        assert!(javadoc.has_throws_tag());

        let tag_ranges = javadoc.get_tag_ranges();
        assert!(tag_ranges.iter().any(|(_, tag)| matches!(tag, JavaDocTag::See)));
        assert!(tag_ranges.iter().any(|(_, tag)| matches!(tag, JavaDocTag::Since)));
        assert!(tag_ranges.iter().any(|(_, tag)| matches!(tag, JavaDocTag::Deprecated)));
    }

    #[test]
    fn test_empty_javadoc() {
        let text = r#"/****/
        "#;

        let source = create_test_source(text);
        let javadoc = JavaDocSource::new(&source);

        assert_eq!(javadoc.get_param_count(), 0);
        assert!(!javadoc.has_return_tag());
        assert!(!javadoc.has_throws_tag());
        assert_eq!(javadoc.get_cleaned_text().trim(), "");
    }

    #[test]
    fn test_javadoc_with_code_examples() {
        let text = r#"/**
         * Example usage:
         * <pre>
         * {@code
         * String result = processor.processInput("test", "json", options);
         * if (result != null) {
         *     System.out.println(result);
         * }
         * }
         * </pre>
         * @param input The input to process
         * @return The processed output
         */
        "#;

        let source = create_test_source(text);
        let javadoc = JavaDocSource::new(&source);

        assert_eq!(javadoc.get_param_count(), 1);
        assert!(javadoc.has_return_tag());

        let cleaned = javadoc.get_cleaned_text();
        assert!(cleaned.contains("Example usage:"));
        assert!(cleaned.contains("<pre>"));
    }
}

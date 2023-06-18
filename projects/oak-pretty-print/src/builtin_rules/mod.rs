use crate::{BasicFormatRule, FormatRule};

/// 创建内置的格式化规则集合
pub fn create_builtin_rules() -> Vec<Box<dyn FormatRule>> {
    let mut rules: Vec<Box<dyn FormatRule>> = Vec::new();

    // 缩进规则
    rules.push(Box::new(create_indent_rule()) as Box<dyn FormatRule>);

    // 空行规则
    rules.push(Box::new(create_blank_line_rule()) as Box<dyn FormatRule>);

    // 括号规则
    rules.push(Box::new(create_bracket_rule()) as Box<dyn FormatRule>);

    // 逗号规则
    rules.push(Box::new(create_comma_rule()) as Box<dyn FormatRule>);

    // 分号规则
    rules.push(Box::new(create_semicolon_rule()) as Box<dyn FormatRule>);

    // 添加其他规则
    rules.push(Box::new(create_comment_rule()) as Box<dyn FormatRule>);
    rules.push(Box::new(create_expression_rule()) as Box<dyn FormatRule>);
    rules.push(Box::new(create_line_length_rule()) as Box<dyn FormatRule>);
    rules.push(Box::new(create_whitespace_rule()) as Box<dyn FormatRule>);

    rules
}

/// 创建缩进规则
fn create_indent_rule() -> BasicFormatRule {
    BasicFormatRule::new(
        "indent".to_string(),
        |_node, context| {
            // 在适当的位置添加缩进
            if context.at_line_start {
                context.write_indent();
            }
            Ok(())
        },
        |_node| true, // 适用于所有节点
    )
    .with_priority(10)
}

/// 创建空行规则
fn create_blank_line_rule() -> BasicFormatRule {
    BasicFormatRule::new(
        "blank_line".to_string(),
        |_node, _context| {
            // TODO: 在声明之间添加空行
            // if let Some(_decl) = node.as_any().downcast_ref::<dyn Declaration>() {
            //     if !context.line_is_empty {
            //         context.write_blank_line();
            //     }
            // }
            Ok(())
        },
        |_node| {
            // TODO: 仅适用于声明节点
            // node.as_any().is::<dyn Declaration>()
            false
        },
    )
    .with_priority(5)
}

/// 创建括号规则
fn create_bracket_rule() -> BasicFormatRule {
    BasicFormatRule::new(
        "bracket".to_string(),
        |_node, _context| {
            // 括号的格式化逻辑
            // 这里需要根据具体的 AST 结构来实现
            // 例如：在开括号后增加缩进，在闭括号前减少缩进
            Ok(())
        },
        |_node| {
            // 适用于包含括号的节点
            // 这里需要根据具体的节点类型来判断
            false // 暂时返回 false，需要具体实现
        },
    )
    .with_priority(8)
}

/// 创建逗号规则
fn create_comma_rule() -> BasicFormatRule {
    BasicFormatRule::new(
        "comma".to_string(),
        |_node, _context| {
            // 逗号后添加空格的逻辑
            // 这里需要根据具体的 AST 结构来实现
            Ok(())
        },
        |_node| {
            // 适用于包含逗号的节点
            // 这里需要根据具体的节点类型来判断
            false // 暂时返回 false，需要具体实现
        },
    )
    .with_priority(6)
}

/// 创建分号规则
fn create_semicolon_rule() -> BasicFormatRule {
    BasicFormatRule::new(
        "semicolon".to_string(),
        |_node, _context| {
            // TODO: 语句结束后添加分号和换行
            // if let Some(_stmt) = node.as_any().downcast_ref::<dyn Statement>() {
            //     context.write(";");
            //     context.ensure_newline();
            // }
            Ok(())
        },
        |_node| {
            // TODO: 适用于语句节点
            // node.as_any().is::<dyn Statement>()
            false
        },
    )
    .with_priority(7)
}

/// 注释格式化规则
pub fn create_comment_rule() -> BasicFormatRule {
    BasicFormatRule::new(
        "comment".to_string(),
        |_node, _context| {
            // 注释的格式化逻辑
            if _context.config.format_comments {
                // 处理注释的格式化
                // 这里可以调用 CommentProcessor 来处理注释
                let _processor = &_context.comment_processor;

                // 示例：格式化行注释
                // 确保行注释前有适当的空格
                if _context.at_line_start {
                    _context.write_indent();
                }

                // 注释内容会由 CommentProcessor 处理
                // 这里只是一个占位符实现
            }
            Ok(())
        },
        |_node| {
            // 适用于注释节点
            // 在实际实现中，需要检查节点是否为注释类型
            // 这里暂时返回 false，因为我们通过 CommentProcessor 处理注释
            false
        },
    )
    .with_priority(3)
}

/// 字符串格式化规则
pub fn create_string_rule() -> BasicFormatRule {
    BasicFormatRule::new(
        "string".to_string(),
        |_node, _context| {
            // 字符串的格式化逻辑
            // TODO: 实现字符串格式化逻辑
            Ok(())
        },
        |_node| {
            // TODO: 实现字符串字面量节点检测
            false
        },
    )
    .with_priority(4)
}

/// 表达式格式化规则
pub fn create_expression_rule() -> BasicFormatRule {
    BasicFormatRule::new(
        "expression".to_string(),
        |_node, _context| {
            // 表达式的格式化逻辑
            // 例如：操作符周围的空格、括号的使用等
            Ok(())
        },
        |_node| {
            // TODO: 实现表达式节点检测
            false
        },
    )
    .with_priority(6)
}

/// 行长度限制规则
pub fn create_line_length_rule() -> BasicFormatRule {
    BasicFormatRule::new(
        "line_length".to_string(),
        |_node, context| {
            // 检查当前行长度，如果超过限制则换行
            let current_line_length = context.output.lines().last().map(|line| line.len()).unwrap_or(0);

            if current_line_length > context.config.max_line_length {
                // 需要换行的逻辑
                // 这里需要更复杂的实现来决定在哪里换行
                context.ensure_newline();
            }
            Ok(())
        },
        |_node| true, // 适用于所有节点
    )
    .with_priority(2)
}

/// 空白字符规则
pub fn create_whitespace_rule() -> BasicFormatRule {
    BasicFormatRule::new(
        "whitespace".to_string(),
        |_node, _context| {
            // 空白字符的处理逻辑
            if _context.config.trim_trailing_whitespace {
                // 修剪尾随空白的逻辑在 FormatContext::finalize 中处理
            }
            Ok(())
        },
        |_node| true, // 适用于所有节点
    )
    .with_priority(1)
}

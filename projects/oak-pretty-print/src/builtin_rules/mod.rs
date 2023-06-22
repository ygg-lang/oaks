use crate::{FormatRule, define_rules, doc};
use alloc::{boxed::Box, vec::Vec};
use oak_core::language::{ElementType, Language, TokenType, UniversalElementRole, UniversalTokenRole};

/// 创建内置的格式化规则集合
pub fn create_builtin_rules<L: Language + 'static>() -> Vec<Box<dyn FormatRule<L>>>
where
    L::ElementType: TokenType + ElementType,
{
    define_rules! {
        // 基础缩进规则
        indent {
            priority: 10,
            node(node, _ctx, format_children) if ElementType::is_universal(&node.green.kind, UniversalElementRole::Container) => {
                let children_doc = format_children(node)?;
                Ok(Some(doc!(indent children_doc)))
            },
        }

        // 语句换行规则
        statement_newline {
            priority: 5,
            node(node, _ctx, format_children) if ElementType::is_universal(&node.green.kind, UniversalElementRole::Statement) => {
                let children_doc = format_children(node)?;
                Ok(Some(doc!([children_doc, line])))
            },
        }

        // 逗号空格规则
        comma_spacing {
            priority: 6,
            token(token, ctx) if TokenType::is_universal(&token.kind, UniversalTokenRole::Punctuation) => {
                if let Some(source) = &ctx.source {
                    let text = &source[token.span.start..token.span.end];
                    if text == "," {
                        return Ok(Some(doc!([",", soft_line_space])));
                    }
                }
                Ok(None)
            },
        }
    }
}

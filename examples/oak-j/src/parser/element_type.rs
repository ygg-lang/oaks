use crate::lexer::JTokenType;
use oak_core::{ElementType, GreenNode, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// J 语法树元素的类型别名
pub type JElement<'a> = Arc<GreenNode<'a, JElementType>>;

/// J 语法树中所有可能的元素类型。
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum JElementType {
    /// Root node
    Root,
    /// Compilation unit
    CompilationUnit,

    /// 句子 (Sentence)
    Sentence,

    /// 赋值语句 (Assignment)
    Assignment,

    /// 表达式 (Expression)
    Expression,

    /// 动词 (Verb)
    Verb,

    /// 名词 (Noun)
    Noun,

    /// 副词 (Adverb)
    Adverb,

    /// 连词 (Conjunction)
    Conjunction,

    /// 括号表达式
    Group,
}

impl From<JTokenType> for JElementType {
    fn from(token: JTokenType) -> Self {
        match token {
            JTokenType::Identifier => Self::Noun,
            JTokenType::NumberLiteral => Self::Noun,
            JTokenType::StringLiteral => Self::Noun,
            JTokenType::IsGlobal | JTokenType::IsLocal => Self::Assignment,
            _ => Self::Expression,
        }
    }
}

impl ElementType for JElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root | Self::CompilationUnit => UniversalElementRole::Root,
            Self::Sentence => UniversalElementRole::Statement,
            Self::Assignment => UniversalElementRole::Binding,
            Self::Expression => UniversalElementRole::Expression,
            _ => UniversalElementRole::None,
        }
    }
}

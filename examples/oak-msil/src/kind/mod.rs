use crate::syntax::MsilSyntaxKind;
use alloc::string::String;
use core::ops::Range;

/// MSIL Token 结构
#[derive(Debug, Clone, PartialEq)]
pub struct MsilToken {
    /// Token 的语法种
    pub kind: MsilSyntaxKind,
    /// Token 在源代码中的位置范围
    pub span: Range<usize>,
    /// Token 的文本内
    pub text: String,
}

impl MsilToken {
    /// 创建新的 MSIL Token
    pub fn new(kind: MsilSyntaxKind, span: Range<usize>, text: String) -> Self {
        Self { kind, span, text }
    }

    /// 获取 Token 的长
    pub fn len(&self) -> usize {
        self.span.end - self.span.start
    }

    /// 检Token 是否为空
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 检查是否为关键Token
    pub fn is_keyword(&self) -> bool {
        matches!(
            self.kind,
            MsilSyntaxKind::AssemblyKeyword
                | MsilSyntaxKind::ExternKeyword
                | MsilSyntaxKind::ModuleKeyword
                | MsilSyntaxKind::ClassKeyword
                | MsilSyntaxKind::MethodKeyword
                | MsilSyntaxKind::PublicKeyword
                | MsilSyntaxKind::PrivateKeyword
                | MsilSyntaxKind::StaticKeyword
        )
    }

    /// 检查是否为标识Token
    pub fn is_identifier(&self) -> bool {
        self.kind == MsilSyntaxKind::IdentifierToken
    }

    /// 检查是否为字面Token
    pub fn is_literal(&self) -> bool {
        matches!(self.kind, MsilSyntaxKind::NumberToken | MsilSyntaxKind::StringToken)
    }

    /// 检查是否为符号 Token
    pub fn is_symbol(&self) -> bool {
        matches!(
            self.kind,
            MsilSyntaxKind::LeftBrace
                | MsilSyntaxKind::RightBrace
                | MsilSyntaxKind::LeftParen
                | MsilSyntaxKind::RightParen
                | MsilSyntaxKind::LeftBracket
                | MsilSyntaxKind::RightBracket
                | MsilSyntaxKind::Dot
                | MsilSyntaxKind::Colon
                | MsilSyntaxKind::Semicolon
                | MsilSyntaxKind::Comma
        )
    }
}

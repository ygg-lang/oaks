use oak_core::SyntaxKind;

/// HTML 语法节点类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HtmlSyntaxKind {
    // 标签相关
    TagOpen,      // <
    TagClose,     // >
    TagSlashOpen, // </
    TagSelfClose, // />

    // 标签名和属性
    TagName,
    AttributeName,
    AttributeValue,

    // 文本内容
    Text,

    // 注释
    Comment, // <!-- -->

    // 特殊字符
    Equal, // =
    Quote, // " '

    // DOCTYPE
    Doctype, // <!DOCTYPE html>

    // CDATA
    CData, // <![CDATA[...]]>

    // 处理指令
    ProcessingInstruction, // <?xml ... ?>

    // 实体引用
    EntityRef, // &amp; &lt;
    CharRef,   // &#123; &#x1A;

    // 空白和换行
    Whitespace,
    Newline,

    // 特殊
    Eof,
    Error,
}

impl SyntaxKind for HtmlSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment | Self::Newline)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, Self::Error)
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::Error)
    }
}

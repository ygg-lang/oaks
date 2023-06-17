use oak_core::SyntaxKind;

/// 统一JSON 语法种类（包含节点与词法
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JsonSyntaxKind {
    // 节点种类
    Root,
    Value,
    Object,
    Array,
    String,
    Number,
    Boolean,
    Null,
    ObjectEntry,
    ArrayElement,
    ErrorNode,

    // 词法种类
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    Colon,        // :
    StringLiteral,
    NumberLiteral,
    BooleanLiteral,
    NullLiteral,
    BareKey, // For JSON5
    Whitespace,
    Comment,
    Eof,
    Error,
}

impl SyntaxKind for JsonSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, JsonSyntaxKind::Whitespace | JsonSyntaxKind::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, JsonSyntaxKind::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, JsonSyntaxKind::Whitespace)
    }

    fn is_token_type(&self) -> bool {
        matches!(
            self,
            JsonSyntaxKind::LeftBrace
                | JsonSyntaxKind::RightBrace
                | JsonSyntaxKind::LeftBracket
                | JsonSyntaxKind::RightBracket
                | JsonSyntaxKind::Comma
                | JsonSyntaxKind::Colon
                | JsonSyntaxKind::StringLiteral
                | JsonSyntaxKind::NumberLiteral
                | JsonSyntaxKind::BooleanLiteral
                | JsonSyntaxKind::NullLiteral
                | JsonSyntaxKind::BareKey
                | JsonSyntaxKind::Whitespace
                | JsonSyntaxKind::Comment
                | JsonSyntaxKind::Eof
                | JsonSyntaxKind::Error
        )
    }

    fn is_element_type(&self) -> bool {
        matches!(
            self,
            JsonSyntaxKind::Root
                | JsonSyntaxKind::Value
                | JsonSyntaxKind::Object
                | JsonSyntaxKind::Array
                | JsonSyntaxKind::String
                | JsonSyntaxKind::Number
                | JsonSyntaxKind::Boolean
                | JsonSyntaxKind::Null
                | JsonSyntaxKind::ObjectEntry
                | JsonSyntaxKind::ArrayElement
                | JsonSyntaxKind::ErrorNode
        )
    }
}

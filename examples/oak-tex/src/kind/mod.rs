use oak_core::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TexSyntaxKind {
    // 节点种类
    Root,
    SourceFile,
    Document,

    // TeX 命令和环境
    Command,
    Environment,
    BeginEnvironment,
    EndEnvironment,

    // TeX 特殊结构
    MathMode,
    InlineMath,
    DisplayMath,
    Group,

    // 参数和选项
    Argument,
    OptionalArgument,
    MandatoryArgument,

    // 文本和内容
    Text,
    Paragraph,
    Section,
    Subsection,
    Subsubsection,

    // 列表和表格
    List,
    Item,
    Table,
    Row,
    Cell,

    // 引用和标签
    Label,
    Reference,
    Citation,

    // 图形和浮动体
    Figure,
    Caption,

    // 错误节点
    Error,

    // TeX 关键字和命令
    DocumentClass,
    UsePackage,
    Begin,
    End,
    Section_,
    Subsection_,
    Subsubsection_,
    Chapter,
    Part,
    Title,
    Author,
    Date,
    MakeTitle,
    TableOfContents,
    NewPage,
    ClearPage,

    // 新增的关键字变体
    BeginKeyword,
    EndKeyword,
    DocumentclassKeyword,
    UsepackageKeyword,
    SectionKeyword,
    SubsectionKeyword,
    SubsubsectionKeyword,
    ChapterKeyword,
    PartKeyword,
    TitleKeyword,
    AuthorKeyword,
    DateKeyword,
    MaketitleKeyword,
    TableofcontentsKeyword,
    ItemKeyword,
    LabelKeyword,
    RefKeyword,
    CiteKeyword,
    IncludegraphicsKeyword,
    TextbfKeyword,
    TextitKeyword,
    EmphKeyword,

    // 数学命令
    Frac,
    Sqrt,
    Sum,
    Int,
    Lim,
    Alpha,
    Beta,
    Gamma,
    Delta,
    Epsilon,

    // 格式化命令
    TextBf,
    TextIt,
    TextSc,
    TextTt,
    Emph,
    Underline,

    // 标识符和字面量
    Identifier,
    StringLiteral,
    Number,

    // 操作符和标点符号
    Backslash,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    Dollar,
    DoubleDollar,
    Ampersand,
    Percent,
    Hash,
    Caret,
    Underscore,
    Tilde,

    // 特殊字符
    Equal,
    Equals,
    Plus,
    Minus,
    Star,
    Slash,
    Pipe,
    Less,
    LessThan,
    Greater,
    GreaterThan,
    Exclamation,
    Question,
    At,
    Colon,
    Semicolon,
    Comma,
    Dot,

    // 空白和注释
    Comment,
    Whitespace,
    Newline,

    // 文件结束
    Eof,
}

impl SyntaxKind for TexSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(
            self,
            Self::Document
                | Self::Command
                | Self::Environment
                | Self::Argument
                | Self::OptionalArgument
                | Self::MandatoryArgument
                | Self::Text
                | Self::List
                | Self::Table
                | Self::Reference
                | Self::Figure
                | Self::Error
        )
    }

    fn is_element_type(&self) -> bool {
        matches!(
            self,
            Self::Document
                | Self::Command
                | Self::Environment
                | Self::Argument
                | Self::OptionalArgument
                | Self::MandatoryArgument
                | Self::Text
                | Self::List
                | Self::Table
                | Self::Reference
                | Self::Figure
                | Self::Error
        )
    }
}

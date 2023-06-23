/// WAT 语法种类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WatTokenType {
    // 节点类型
    Root,
    Module,
    Function,
    Memory,
    Export,
    Import,
    Param,
    Result,
    Local,
    Instruction,

    // 词法种类
    LeftParen,
    RightParen,
    String,
    Number,
    Identifier,

    // 关键
    ModuleKeyword,
    FuncKeyword,
    MemoryKeyword,
    ExportKeyword,
    ImportKeyword,
    ParamKeyword,
    ResultKeyword,
    LocalKeyword,

    // 类型关键
    I32,
    I64,
    F32,
    F64,

    // 空白和注
    Whitespace,
    Comment,

    // 错误和结
    Error,
    Eof,
}

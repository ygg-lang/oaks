/// WAT (WebAssembly Text) tokens types for Component Model
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WatTokenType {
    // Literals
    /// 字符串字面量 tokens
    String,
    /// 数字字面tokens
    Number,

    // Identifiers and Names
    /// 标识tokens
    Identifier,

    // Structural tokens
    /// 左括'('
    LeftParen,
    /// 右括')'
    RightParen,

    // Core WebAssembly Keywords
    /// module 关键
    Module,
    /// func 关键
    Func,
    /// memory 关键
    Memory,
    /// export 关键
    Export,
    /// import 关键
    Import,
    /// param 关键
    Param,
    /// result 关键
    Result,
    /// local 关键
    Local,

    // WebAssembly Types
    /// i32 类型
    I32,
    /// i64 类型
    I64,
    /// f32 类型
    F32,
    /// f64 类型
    F64,
}

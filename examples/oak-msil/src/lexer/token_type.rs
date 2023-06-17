/// MSIL 汇编语言Token 类型
///
/// 这个枚举定义MSIL 汇编语言中所有可能的 tokens 类型
///
/// # 示例
///
/// ```rust
/// # use oak_msil::lexer::MsilTokenType;
///
/// let tokens = MsilTokenType::Assembly;
/// assert_eq!(tokens, MsilTokenType::Assembly);
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MsilTokenType {
    // 指令关键
    /// .assembly 指令
    Assembly,
    /// extern 关键
    Extern,
    /// .module 指令
    Module,
    /// .class 指令
    Class,
    /// .method 指令
    Method,

    // 修饰
    /// public 修饰
    Public,
    /// private 修饰
    Private,
    /// static 修饰
    Static,

    // 基本类型
    /// 标识
    Identifier,
    /// 数字字面
    Number,
    /// 字符串字面量
    StringLiteral,

    // 符号
    /// 左大括号 {
    LeftBrace,
    /// 右大括号 }
    RightBrace,
    /// 左小括号 (
    LeftParen,
    /// 右小括号 )
    RightParen,
    /// 左方括号 [
    LeftBracket,
    /// 右方括号 ]
    RightBracket,
    /// .
    Dot,
    /// 冒号 :
    Colon,
    /// 分号 ;
    Semicolon,
    /// 逗号 ,
    Comma,

    // 空白和注
    /// 空白字符
    Whitespace,
    /// 注释
    Comment,
}

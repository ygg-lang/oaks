/// JASM 汇编语言Token 类型
///
/// 这个枚举定义JASM 汇编语言中所有可能的 tokens 类型
/// 所有变体都不包含数据，使得该类型可以实Copy trait

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JasmTokenType {
    // 关键
    /// class 关键
    Class,
    /// version 关键
    Version,
    /// Method 关键
    Method,
    /// Field 关键
    Field,
    /// String 关键
    String,
    /// SourceFile 关键
    SourceFile,
    /// stack 关键
    Stack,
    /// locals 关键
    Locals,
    /// end 关键
    End,
    /// compiled 关键
    Compiled,
    /// from 关键
    From,
    /// InnerClass 关键
    InnerClass,
    /// NestMembers 关键
    NestMembers,
    /// BootstrapMethod 关键
    BootstrapMethod,

    // 访问修饰
    /// public 修饰
    Public,
    /// private 修饰
    Private,
    /// protected 修饰
    Protected,
    /// static 修饰
    Static,
    /// super 修饰
    Super,
    /// final 修饰
    Final,
    /// abstract 修饰
    Abstract,
    /// synchronized 修饰
    Synchronized,
    /// native 修饰
    Native,
    /// synthetic 修饰
    Synthetic,
    /// deprecated 修饰
    Deprecated,
    /// varargs 修饰
    Varargs,

    // JVM 指令
    /// aload_0 指令
    ALoad0,
    /// aload_1 指令
    ALoad1,
    /// aload_2 指令
    ALoad2,
    /// aload_3 指令
    ALoad3,
    /// iload_0 指令
    ILoad0,
    /// iload_1 指令
    ILoad1,
    /// iload_2 指令
    ILoad2,
    /// iload_3 指令
    ILoad3,
    /// ldc 指令
    Ldc,
    /// ldc_w 指令
    LdcW,
    /// ldc2_w 指令
    Ldc2W,
    /// invokespecial 指令
    InvokeSpecial,
    /// invokevirtual 指令
    InvokeVirtual,
    /// invokestatic 指令
    InvokeStatic,
    /// invokeinterface 指令
    InvokeInterface,
    /// invokedynamic 指令
    InvokeDynamic,
    /// getstatic 指令
    GetStatic,
    /// putstatic 指令
    PutStatic,
    /// getfield 指令
    GetField,
    /// putfield 指令
    PutField,
    /// return 指令
    Return,
    /// ireturn 指令
    IReturn,
    /// areturn 指令
    AReturn,
    /// lreturn 指令
    LReturn,
    /// freturn 指令
    FReturn,
    /// dreturn 指令
    DReturn,
    /// nop 指令
    Nop,
    /// dup 指令
    Dup,
    /// pop 指令
    Pop,
    /// new 指令
    New,

    // 符号
    /// 左大括号 {
    LeftBrace,
    /// 右大括号 }
    RightBrace,
    /// 左小括号 (
    LeftParen,
    /// 右小括号 )
    RightParen,
    /// 左中括号 [
    LeftBracket,
    /// 右中括号 ]
    RightBracket,
    /// 冒号 :
    Colon,
    /// 分号 ;
    Semicolon,
    /// 点号 .
    Dot,
    /// 逗号 ,
    Comma,
    /// 斜杠 /
    Slash,

    // 字面
    /// 字符串字面量
    StringLiteral,
    /// 数字字面
    Number,
    /// 类型描述
    TypeDescriptor,

    // 标识
    /// 标识
    Identifier,

    // 空白和注
    /// 空白字符
    Whitespace,
    /// 注释
    Comment,

    // 特殊
    /// 文件结束
    Eof,
}

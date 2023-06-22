use crate::{kind::HlslSyntaxKind, language::HlslLanguage};
use oak_core::{Lexer, LexerCache, LexerState, OakError, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, HlslLanguage>;

pub struct HlslLexer<'config> {
    _config: &'config HlslLanguage,
}

impl<'config> Clone for HlslLexer<'config> {
    fn clone(&self) -> Self {
        Self { _config: self._config }
    }
}

impl<'config> HlslLexer<'config> {
    pub fn new(config: &'config HlslLanguage) -> Self {
        Self { _config: config }
    }

    /// 主要的词法分析循环
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            // 尝试各种词法规则
            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.lex_comment(state) {
                continue;
            }

            if self.lex_preprocessor(state) {
                continue;
            }

            if self.lex_string(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_operator_or_delimiter(state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(HlslSyntaxKind::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(HlslSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(HlslSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(HlslSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        // 单行注释 //
        if let Some('/') = state.peek() {
            if let Some('/') = state.peek_next_n(1) {
                state.advance(2);
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                state.add_token(HlslSyntaxKind::Comment, start_pos, state.get_position());
                return true;
            }
        }

        // 多行注释 /* ... */
        if let Some('/') = state.peek() {
            if let Some('*') = state.peek_next_n(1) {
                state.advance(2);
                while state.not_at_end() {
                    if let Some('*') = state.peek() {
                        if let Some('/') = state.peek_next_n(1) {
                            state.advance(2);
                            break;
                        }
                    }
                    if let Some(ch) = state.peek() {
                        state.advance(ch.len_utf8());
                    }
                }
                state.add_token(HlslSyntaxKind::Comment, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理预处理器指令
    fn lex_preprocessor<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1);

            // 跳过空白
            while let Some(ch) = state.peek() {
                if ch == ' ' || ch == '\t' {
                    state.advance(1);
                }
                else {
                    break;
                }
            }

            // 读取指令名称
            let directive_start = state.get_position();
            while let Some(ch) = state.peek() {
                if ch.is_ascii_alphabetic() || ch == '_' {
                    state.advance(1);
                }
                else {
                    break;
                }
            }

            if state.get_position() > directive_start {
                let directive = state.get_text_in((directive_start..state.get_position()).into()).to_string();

                // 读取指令的其余部分直到行尾
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                let token_kind = match directive.as_str() {
                    "include" => HlslSyntaxKind::Include,
                    "define" => HlslSyntaxKind::Define,
                    "undef" => HlslSyntaxKind::Undef,
                    "if" => HlslSyntaxKind::If_,
                    "ifdef" => HlslSyntaxKind::Ifdef,
                    "ifndef" => HlslSyntaxKind::Ifndef,
                    "else" => HlslSyntaxKind::Else_,
                    "elif" => HlslSyntaxKind::Elif,
                    "endif" => HlslSyntaxKind::Endif,
                    "line" => HlslSyntaxKind::Line,
                    "error" => HlslSyntaxKind::Error,
                    "pragma" => HlslSyntaxKind::Pragma,
                    _ => HlslSyntaxKind::Hash,
                };

                state.add_token(token_kind, start_pos, state.get_position());
                return true;
            }
            else {
                // 只是一个 # 符号
                state.add_token(HlslSyntaxKind::Hash, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理字符串字面量
    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);
            let mut escaped = false;

            while let Some(ch) = state.peek() {
                if escaped {
                    escaped = false;
                }
                else if ch == '\\' {
                    escaped = true;
                }
                else if ch == '"' {
                    state.advance(1);
                    break;
                }
                else if ch == '\n' || ch == '\r' {
                    break; // 字符串不能跨行
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(HlslSyntaxKind::StringLiteral, start_pos, state.get_position());
            return true;
        }

        false
    }

    /// 处理数字字面量
    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() || (ch == '.' && state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit())) {
                // 处理十六进制数
                if ch == '0' && state.peek_next_n(1) == Some('x') {
                    state.advance(2);
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_hexdigit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
                else {
                    // 整数部分
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }

                    // 小数点和小数部分
                    if let Some('.') = state.peek() {
                        if state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit()) {
                            state.advance(1);
                            while let Some(ch) = state.peek() {
                                if ch.is_ascii_digit() {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }
                        }
                    }

                    // 指数部分
                    if let Some(e_char) = state.peek() {
                        if e_char == 'e' || e_char == 'E' {
                            let saved_pos = state.get_position();
                            state.advance(1);

                            // 可选的符号
                            if let Some(sign) = state.peek() {
                                if sign == '+' || sign == '-' {
                                    state.advance(1);
                                }
                            }

                            // 指数数字
                            let exp_start = state.get_position();
                            while let Some(ch) = state.peek() {
                                if ch.is_ascii_digit() {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }

                            if state.get_position() == exp_start {
                                // 没有有效的指数，回退
                                state.set_position(saved_pos);
                            }
                        }
                    }
                }

                // 处理后缀 (f, h, l, u 等)
                if let Some(suffix) = state.peek() {
                    if suffix == 'f' || suffix == 'F' || suffix == 'h' || suffix == 'H' || suffix == 'l' || suffix == 'L' || suffix == 'u' || suffix == 'U' {
                        state.advance(1);
                    }
                }

                state.add_token(HlslSyntaxKind::NumberLiteral, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理标识符和关键字
    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_in((start_pos..state.get_position()).into());
                let token_kind = match text.as_ref() {
                    // 基本数据类型
                    "bool" => HlslSyntaxKind::Bool,
                    "int" => HlslSyntaxKind::Int,
                    "uint" => HlslSyntaxKind::Uint,
                    "half" => HlslSyntaxKind::Half,
                    "float" => HlslSyntaxKind::Float,
                    "double" => HlslSyntaxKind::Double,
                    "min16float" => HlslSyntaxKind::Min16float,
                    "min10float" => HlslSyntaxKind::Min10float,
                    "min16int" => HlslSyntaxKind::Min16int,
                    "min12int" => HlslSyntaxKind::Min12int,
                    "min16uint" => HlslSyntaxKind::Min16uint,

                    // 向量类型
                    "bool2" => HlslSyntaxKind::Bool2,
                    "bool3" => HlslSyntaxKind::Bool3,
                    "bool4" => HlslSyntaxKind::Bool4,
                    "int2" => HlslSyntaxKind::Int2,
                    "int3" => HlslSyntaxKind::Int3,
                    "int4" => HlslSyntaxKind::Int4,
                    "uint2" => HlslSyntaxKind::Uint2,
                    "uint3" => HlslSyntaxKind::Uint3,
                    "uint4" => HlslSyntaxKind::Uint4,
                    "half2" => HlslSyntaxKind::Half2,
                    "half3" => HlslSyntaxKind::Half3,
                    "half4" => HlslSyntaxKind::Half4,
                    "float2" => HlslSyntaxKind::Float2,
                    "float3" => HlslSyntaxKind::Float3,
                    "float4" => HlslSyntaxKind::Float4,
                    "double2" => HlslSyntaxKind::Double2,
                    "double3" => HlslSyntaxKind::Double3,
                    "double4" => HlslSyntaxKind::Double4,

                    // 矩阵类型
                    "float2x2" => HlslSyntaxKind::Float2x2,
                    "float2x3" => HlslSyntaxKind::Float2x3,
                    "float2x4" => HlslSyntaxKind::Float2x4,
                    "float3x2" => HlslSyntaxKind::Float3x2,
                    "float3x3" => HlslSyntaxKind::Float3x3,
                    "float3x4" => HlslSyntaxKind::Float3x4,
                    "float4x2" => HlslSyntaxKind::Float4x2,
                    "float4x3" => HlslSyntaxKind::Float4x3,
                    "float4x4" => HlslSyntaxKind::Float4x4,
                    "double2x2" => HlslSyntaxKind::Double2x2,
                    "double2x3" => HlslSyntaxKind::Double2x3,
                    "double2x4" => HlslSyntaxKind::Double2x4,
                    "double3x2" => HlslSyntaxKind::Double3x2,
                    "double3x3" => HlslSyntaxKind::Double3x3,
                    "double3x4" => HlslSyntaxKind::Double3x4,
                    "double4x2" => HlslSyntaxKind::Double4x2,
                    "double4x3" => HlslSyntaxKind::Double4x3,
                    "double4x4" => HlslSyntaxKind::Double4x4,

                    // 纹理类型
                    "Texture1D" => HlslSyntaxKind::Texture1D,
                    "Texture1DArray" => HlslSyntaxKind::Texture1DArray,
                    "Texture2D" => HlslSyntaxKind::Texture2D,
                    "Texture2DArray" => HlslSyntaxKind::Texture2DArray,
                    "Texture2DMS" => HlslSyntaxKind::Texture2DMS,
                    "Texture2DMSArray" => HlslSyntaxKind::Texture2DMSArray,
                    "Texture3D" => HlslSyntaxKind::Texture3D,
                    "TextureCube" => HlslSyntaxKind::TextureCube,
                    "TextureCubeArray" => HlslSyntaxKind::TextureCubeArray,

                    // 采样器类型
                    "sampler" => HlslSyntaxKind::Sampler,
                    "SamplerState" => HlslSyntaxKind::SamplerState,
                    "SamplerComparisonState" => HlslSyntaxKind::SamplerComparisonState,

                    // 缓冲区类型
                    "Buffer" => HlslSyntaxKind::Buffer,
                    "StructuredBuffer" => HlslSyntaxKind::StructuredBuffer,
                    "ByteAddressBuffer" => HlslSyntaxKind::ByteAddressBuffer,
                    "RWBuffer" => HlslSyntaxKind::RWBuffer,
                    "RWStructuredBuffer" => HlslSyntaxKind::RWStructuredBuffer,
                    "RWByteAddressBuffer" => HlslSyntaxKind::RWByteAddressBuffer,
                    "AppendStructuredBuffer" => HlslSyntaxKind::AppendStructuredBuffer,
                    "ConsumeStructuredBuffer" => HlslSyntaxKind::ConsumeStructuredBuffer,

                    // 控制流关键字
                    "if" => HlslSyntaxKind::If,
                    "else" => HlslSyntaxKind::Else,
                    "for" => HlslSyntaxKind::For,
                    "while" => HlslSyntaxKind::While,
                    "do" => HlslSyntaxKind::Do,
                    "switch" => HlslSyntaxKind::Switch,
                    "case" => HlslSyntaxKind::Case,
                    "default" => HlslSyntaxKind::Default,
                    "break" => HlslSyntaxKind::Break,
                    "continue" => HlslSyntaxKind::Continue,
                    "return" => HlslSyntaxKind::Return,
                    "discard" => HlslSyntaxKind::Discard,

                    // 函数和变量修饰符
                    "static" => HlslSyntaxKind::Static,
                    "const" => HlslSyntaxKind::Const,
                    "volatile" => HlslSyntaxKind::Volatile,
                    "extern" => HlslSyntaxKind::Extern,
                    "shared" => HlslSyntaxKind::Shared,
                    "groupshared" => HlslSyntaxKind::Groupshared,
                    "uniform" => HlslSyntaxKind::Uniform,
                    "in" => HlslSyntaxKind::In,
                    "out" => HlslSyntaxKind::Out,
                    "inout" => HlslSyntaxKind::Inout,
                    "inline" => HlslSyntaxKind::Inline,
                    "target" => HlslSyntaxKind::Target,

                    // 语义修饰符
                    "register" => HlslSyntaxKind::Register,
                    "packoffset" => HlslSyntaxKind::Packoffset,

                    // 着色器类型
                    "struct" => HlslSyntaxKind::Struct,
                    "cbuffer" => HlslSyntaxKind::Cbuffer,
                    "tbuffer" => HlslSyntaxKind::Tbuffer,
                    "interface" => HlslSyntaxKind::Interface,
                    "class" => HlslSyntaxKind::Class,

                    // 布尔字面量
                    "true" | "false" => HlslSyntaxKind::BooleanLiteral,

                    _ => HlslSyntaxKind::Identifier,
                };

                state.add_token(token_kind, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理运算符和分隔符
    fn lex_operator_or_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        HlslSyntaxKind::PlusAssign
                    }
                    else if let Some('+') = state.peek() {
                        state.advance(1);
                        HlslSyntaxKind::Increment
                    }
                    else {
                        HlslSyntaxKind::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        HlslSyntaxKind::MinusAssign
                    }
                    else if let Some('-') = state.peek() {
                        state.advance(1);
                        HlslSyntaxKind::Decrement
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        HlslSyntaxKind::Arrow
                    }
                    else {
                        HlslSyntaxKind::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        HlslSyntaxKind::MultiplyAssign
                    }
                    else {
                        HlslSyntaxKind::Multiply
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        HlslSyntaxKind::DivideAssign
                    }
                    else {
                        HlslSyntaxKind::Divide
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        HlslSyntaxKind::ModuloAssign
                    }
                    else {
                        HlslSyntaxKind::Modulo
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        HlslSyntaxKind::Equal
                    }
                    else {
                        HlslSyntaxKind::Assign
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        HlslSyntaxKind::NotEqual
                    }
                    else {
                        HlslSyntaxKind::LogicalNot
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        HlslSyntaxKind::LessEqual
                    }
                    else if let Some('<') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            HlslSyntaxKind::LeftShiftAssign
                        }
                        else {
                            HlslSyntaxKind::LeftShift
                        }
                    }
                    else {
                        HlslSyntaxKind::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        HlslSyntaxKind::GreaterEqual
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            HlslSyntaxKind::RightShiftAssign
                        }
                        else {
                            HlslSyntaxKind::RightShift
                        }
                    }
                    else {
                        HlslSyntaxKind::Greater
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        HlslSyntaxKind::LogicalAnd
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        HlslSyntaxKind::BitwiseAndAssign
                    }
                    else {
                        HlslSyntaxKind::BitwiseAnd
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        HlslSyntaxKind::LogicalOr
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        HlslSyntaxKind::BitwiseOrAssign
                    }
                    else {
                        HlslSyntaxKind::BitwiseOr
                    }
                }
                '^' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        HlslSyntaxKind::BitwiseXorAssign
                    }
                    else {
                        HlslSyntaxKind::BitwiseXor
                    }
                }
                '~' => {
                    state.advance(1);
                    HlslSyntaxKind::BitwiseNot
                }
                '?' => {
                    state.advance(1);
                    HlslSyntaxKind::Conditional
                }
                '.' => {
                    state.advance(1);
                    HlslSyntaxKind::Dot
                }
                ':' => {
                    state.advance(1);
                    if let Some(':') = state.peek() {
                        state.advance(1);
                        HlslSyntaxKind::DoubleColon
                    }
                    else {
                        HlslSyntaxKind::Colon
                    }
                }
                ';' => {
                    state.advance(1);
                    HlslSyntaxKind::Semicolon
                }
                ',' => {
                    state.advance(1);
                    HlslSyntaxKind::Comma
                }
                '(' => {
                    state.advance(1);
                    HlslSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    HlslSyntaxKind::RightParen
                }
                '[' => {
                    state.advance(1);
                    HlslSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    HlslSyntaxKind::RightBracket
                }
                '{' => {
                    state.advance(1);
                    HlslSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    HlslSyntaxKind::RightBrace
                }
                '\\' => {
                    state.advance(1);
                    HlslSyntaxKind::Backslash
                }
                _ => return false,
            };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<HlslLanguage> for HlslLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<HlslLanguage>) -> LexOutput<HlslLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

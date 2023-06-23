#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::HlslLanguage, lexer::token_type::HlslTokenType};
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
                state.add_token(HlslTokenType::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point)
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
            state.add_token(HlslTokenType::Whitespace, start_pos, state.get_position());
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
            state.add_token(HlslTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1)
            }
            state.add_token(HlslTokenType::Newline, start_pos, state.get_position());
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
                    state.advance(ch.len_utf8())
                }
                state.add_token(HlslTokenType::Comment, start_pos, state.get_position());
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
                        state.advance(ch.len_utf8())
                    }
                }
                state.add_token(HlslTokenType::Comment, start_pos, state.get_position());
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
                if ch == ' ' || ch == '\t' { state.advance(1) } else { break }
            }

            // 读取指令名称
            let directive_start = state.get_position();
            while let Some(ch) = state.peek() {
                if ch.is_ascii_alphabetic() || ch == '_' { state.advance(1) } else { break }
            }

            if state.get_position() > directive_start {
                let directive = state.get_text_in((directive_start..state.get_position()).into()).to_string();

                // 读取指令的其余部分直到行尾
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8())
                }

                let token_kind = match directive.as_str() {
                    "include" => HlslTokenType::Include,
                    "define" => HlslTokenType::Define,
                    "undef" => HlslTokenType::Undef,
                    "if" => HlslTokenType::If_,
                    "ifdef" => HlslTokenType::Ifdef,
                    "ifndef" => HlslTokenType::Ifndef,
                    "else" => HlslTokenType::Else_,
                    "elif" => HlslTokenType::Elif,
                    "endif" => HlslTokenType::Endif,
                    "line" => HlslTokenType::Line,
                    "error" => HlslTokenType::Error,
                    "pragma" => HlslTokenType::Pragma,
                    _ => HlslTokenType::Hash,
                };

                state.add_token(token_kind, start_pos, state.get_position());
                return true;
            }
            else {
                // 只是一个 # 符号
                state.add_token(HlslTokenType::Hash, start_pos, state.get_position());
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

            state.add_token(HlslTokenType::StringLiteral, start_pos, state.get_position());
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
                                    state.advance(1)
                                }
                            }

                            // 指数数字
                            let exp_start = state.get_position();
                            while let Some(ch) = state.peek() {
                                if ch.is_ascii_digit() { state.advance(1) } else { break }
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

                state.add_token(HlslTokenType::NumberLiteral, start_pos, state.get_position());
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
                    "bool" => HlslTokenType::Bool,
                    "int" => HlslTokenType::Int,
                    "uint" => HlslTokenType::Uint,
                    "half" => HlslTokenType::Half,
                    "float" => HlslTokenType::Float,
                    "double" => HlslTokenType::Double,
                    "min16float" => HlslTokenType::Min16float,
                    "min10float" => HlslTokenType::Min10float,
                    "min16int" => HlslTokenType::Min16int,
                    "min12int" => HlslTokenType::Min12int,
                    "min16uint" => HlslTokenType::Min16uint,

                    // 向量类型
                    "bool2" => HlslTokenType::Bool2,
                    "bool3" => HlslTokenType::Bool3,
                    "bool4" => HlslTokenType::Bool4,
                    "int2" => HlslTokenType::Int2,
                    "int3" => HlslTokenType::Int3,
                    "int4" => HlslTokenType::Int4,
                    "uint2" => HlslTokenType::Uint2,
                    "uint3" => HlslTokenType::Uint3,
                    "uint4" => HlslTokenType::Uint4,
                    "half2" => HlslTokenType::Half2,
                    "half3" => HlslTokenType::Half3,
                    "half4" => HlslTokenType::Half4,
                    "float2" => HlslTokenType::Float2,
                    "float3" => HlslTokenType::Float3,
                    "float4" => HlslTokenType::Float4,
                    "double2" => HlslTokenType::Double2,
                    "double3" => HlslTokenType::Double3,
                    "double4" => HlslTokenType::Double4,

                    // 矩阵类型
                    "float2x2" => HlslTokenType::Float2x2,
                    "float2x3" => HlslTokenType::Float2x3,
                    "float2x4" => HlslTokenType::Float2x4,
                    "float3x2" => HlslTokenType::Float3x2,
                    "float3x3" => HlslTokenType::Float3x3,
                    "float3x4" => HlslTokenType::Float3x4,
                    "float4x2" => HlslTokenType::Float4x2,
                    "float4x3" => HlslTokenType::Float4x3,
                    "float4x4" => HlslTokenType::Float4x4,
                    "double2x2" => HlslTokenType::Double2x2,
                    "double2x3" => HlslTokenType::Double2x3,
                    "double2x4" => HlslTokenType::Double2x4,
                    "double3x2" => HlslTokenType::Double3x2,
                    "double3x3" => HlslTokenType::Double3x3,
                    "double3x4" => HlslTokenType::Double3x4,
                    "double4x2" => HlslTokenType::Double4x2,
                    "double4x3" => HlslTokenType::Double4x3,
                    "double4x4" => HlslTokenType::Double4x4,

                    // 纹理类型
                    "Texture1D" => HlslTokenType::Texture1D,
                    "Texture1DArray" => HlslTokenType::Texture1DArray,
                    "Texture2D" => HlslTokenType::Texture2D,
                    "Texture2DArray" => HlslTokenType::Texture2DArray,
                    "Texture2DMS" => HlslTokenType::Texture2DMS,
                    "Texture2DMSArray" => HlslTokenType::Texture2DMSArray,
                    "Texture3D" => HlslTokenType::Texture3D,
                    "TextureCube" => HlslTokenType::TextureCube,
                    "TextureCubeArray" => HlslTokenType::TextureCubeArray,

                    // 采样器类型
                    "sampler" => HlslTokenType::Sampler,
                    "SamplerState" => HlslTokenType::SamplerState,
                    "SamplerComparisonState" => HlslTokenType::SamplerComparisonState,

                    // 缓冲区类型
                    "Buffer" => HlslTokenType::Buffer,
                    "StructuredBuffer" => HlslTokenType::StructuredBuffer,
                    "ByteAddressBuffer" => HlslTokenType::ByteAddressBuffer,
                    "RWBuffer" => HlslTokenType::RWBuffer,
                    "RWStructuredBuffer" => HlslTokenType::RWStructuredBuffer,
                    "RWByteAddressBuffer" => HlslTokenType::RWByteAddressBuffer,
                    "AppendStructuredBuffer" => HlslTokenType::AppendStructuredBuffer,
                    "ConsumeStructuredBuffer" => HlslTokenType::ConsumeStructuredBuffer,

                    // 控制流关键字
                    "if" => HlslTokenType::If,
                    "else" => HlslTokenType::Else,
                    "for" => HlslTokenType::For,
                    "while" => HlslTokenType::While,
                    "do" => HlslTokenType::Do,
                    "switch" => HlslTokenType::Switch,
                    "case" => HlslTokenType::Case,
                    "default" => HlslTokenType::Default,
                    "break" => HlslTokenType::Break,
                    "continue" => HlslTokenType::Continue,
                    "return" => HlslTokenType::Return,
                    "discard" => HlslTokenType::Discard,

                    // 函数和变量修饰符
                    "static" => HlslTokenType::Static,
                    "const" => HlslTokenType::Const,
                    "volatile" => HlslTokenType::Volatile,
                    "extern" => HlslTokenType::Extern,
                    "shared" => HlslTokenType::Shared,
                    "groupshared" => HlslTokenType::Groupshared,
                    "uniform" => HlslTokenType::Uniform,
                    "in" => HlslTokenType::In,
                    "out" => HlslTokenType::Out,
                    "inout" => HlslTokenType::Inout,
                    "inline" => HlslTokenType::Inline,
                    "target" => HlslTokenType::Target,

                    // 语义修饰符
                    "register" => HlslTokenType::Register,
                    "packoffset" => HlslTokenType::Packoffset,

                    // 着色器类型
                    "struct" => HlslTokenType::Struct,
                    "cbuffer" => HlslTokenType::Cbuffer,
                    "tbuffer" => HlslTokenType::Tbuffer,
                    "interface" => HlslTokenType::Interface,
                    "class" => HlslTokenType::Class,

                    // 布尔字面量
                    "true" | "false" => HlslTokenType::BooleanLiteral,

                    _ => HlslTokenType::Identifier,
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
                        HlslTokenType::PlusAssign
                    }
                    else if let Some('+') = state.peek() {
                        state.advance(1);
                        HlslTokenType::Increment
                    }
                    else {
                        HlslTokenType::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        HlslTokenType::MinusAssign
                    }
                    else if let Some('-') = state.peek() {
                        state.advance(1);
                        HlslTokenType::Decrement
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        HlslTokenType::Arrow
                    }
                    else {
                        HlslTokenType::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        HlslTokenType::MultiplyAssign
                    }
                    else {
                        HlslTokenType::Multiply
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        HlslTokenType::DivideAssign
                    }
                    else {
                        HlslTokenType::Divide
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        HlslTokenType::ModuloAssign
                    }
                    else {
                        HlslTokenType::Modulo
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        HlslTokenType::Equal
                    }
                    else {
                        HlslTokenType::Assign
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        HlslTokenType::NotEqual
                    }
                    else {
                        HlslTokenType::LogicalNot
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        HlslTokenType::LessEqual
                    }
                    else if let Some('<') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            HlslTokenType::LeftShiftAssign
                        }
                        else {
                            HlslTokenType::LeftShift
                        }
                    }
                    else {
                        HlslTokenType::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        HlslTokenType::GreaterEqual
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            HlslTokenType::RightShiftAssign
                        }
                        else {
                            HlslTokenType::RightShift
                        }
                    }
                    else {
                        HlslTokenType::Greater
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        HlslTokenType::LogicalAnd
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        HlslTokenType::BitwiseAndAssign
                    }
                    else {
                        HlslTokenType::BitwiseAnd
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        HlslTokenType::LogicalOr
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        HlslTokenType::BitwiseOrAssign
                    }
                    else {
                        HlslTokenType::BitwiseOr
                    }
                }
                '^' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        HlslTokenType::BitwiseXorAssign
                    }
                    else {
                        HlslTokenType::BitwiseXor
                    }
                }
                '~' => {
                    state.advance(1);
                    HlslTokenType::BitwiseNot
                }
                '?' => {
                    state.advance(1);
                    HlslTokenType::Conditional
                }
                '.' => {
                    state.advance(1);
                    HlslTokenType::Dot
                }
                ':' => {
                    state.advance(1);
                    if let Some(':') = state.peek() {
                        state.advance(1);
                        HlslTokenType::DoubleColon
                    }
                    else {
                        HlslTokenType::Colon
                    }
                }
                ';' => {
                    state.advance(1);
                    HlslTokenType::Semicolon
                }
                ',' => {
                    state.advance(1);
                    HlslTokenType::Comma
                }
                '(' => {
                    state.advance(1);
                    HlslTokenType::LeftParen
                }
                ')' => {
                    state.advance(1);
                    HlslTokenType::RightParen
                }
                '[' => {
                    state.advance(1);
                    HlslTokenType::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    HlslTokenType::RightBracket
                }
                '{' => {
                    state.advance(1);
                    HlslTokenType::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    HlslTokenType::RightBrace
                }
                '\\' => {
                    state.advance(1);
                    HlslTokenType::Backslash
                }
                '#' => {
                    state.advance(1);
                    HlslTokenType::Hash
                }
                '@' => {
                    state.advance(1);
                    HlslTokenType::At
                }
                _ => return false,
            };

            state.add_token(token_kind, start_pos, state.get_position());
            return true;
        }

        false
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

//! GSGL 词法分析
//!
//! 实现GSGL 语言的词法分析，将源代码转换token 序列

use crate::{language::GsglLanguage, syntax::GsglSyntaxKind};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, GsglLanguage>;

/// GSGL 词法分析器
pub struct GsglLexer<'config> {
    config: &'config GsglLanguage,
}

impl<'config> GsglLexer<'config> {
    /// 创建新的 GSGL 词法分析器
    pub fn new(config: &'config GsglLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn skip_whitespace(&self, state: &mut State) -> bool {
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
            state.add_token(GsglSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(GsglSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(GsglSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        // 单行注释 //
        if state.peek() == Some('/') && state.peek_next_n(1) == Some('/') {
            state.advance(2);

            // 读取到行尾
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(GsglSyntaxKind::Comment, start_pos, state.get_position());
            return true;
        }

        // 多行注释 /* */
        if state.peek() == Some('/') && state.peek_next_n(1) == Some('*') {
            state.advance(2);

            while let Some(ch) = state.peek() {
                if ch == '*' && state.peek_next_n(1) == Some('/') {
                    state.advance(2);
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(GsglSyntaxKind::Comment, start_pos, state.get_position());
            return true;
        }

        false
    }

    /// 处理字符串字面量
    fn lex_string(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1);
                        break;
                    }
                    else if ch == '\\' {
                        // 转义字符
                        state.advance(1);
                        if let Some(escaped) = state.peek() {
                            state.advance(escaped.len_utf8());
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(GsglSyntaxKind::String, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理数字字面
    fn lex_number(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                // 整数部分
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // 小数部分
                if state.peek() == Some('.') && state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit()) {
                    state.advance(1); // 跳过 '.'
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                // 指数部分
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' {
                        state.advance(1);
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1);
                            }
                        }
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

                // 浮点数后缀
                if let Some(ch) = state.peek() {
                    if ch == 'f' || ch == 'F' {
                        state.advance(1);
                    }
                }

                state.add_token(GsglSyntaxKind::Number, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理标识符或关键
    fn lex_identifier_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = source.get_text_in((start_pos..state.get_position()).into()).unwrap_or("");
                let kind = self.keyword_or_identifier(text);
                state.add_token(kind, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 判断是关键字还是标识
    fn keyword_or_identifier(&self, text: &str) -> GsglSyntaxKind {
        match text {
            "shader" => GsglSyntaxKind::Shader,
            "vertex" => GsglSyntaxKind::Vertex,
            "fragment" => GsglSyntaxKind::Fragment,
            "geometry" => GsglSyntaxKind::Geometry,
            "compute" => GsglSyntaxKind::Compute,
            "uniform" => GsglSyntaxKind::Uniform,
            "attribute" => GsglSyntaxKind::Attribute,
            "varying" => GsglSyntaxKind::Varying,
            "in" => GsglSyntaxKind::In,
            "out" => GsglSyntaxKind::Out,
            "inout" => GsglSyntaxKind::Inout,
            "const" => GsglSyntaxKind::Const,
            "struct" => GsglSyntaxKind::Struct,
            "if" => GsglSyntaxKind::If,
            "else" => GsglSyntaxKind::Else,
            "for" => GsglSyntaxKind::For,
            "while" => GsglSyntaxKind::While,
            "do" => GsglSyntaxKind::Do,
            "break" => GsglSyntaxKind::Break,
            "continue" => GsglSyntaxKind::Continue,
            "return" => GsglSyntaxKind::Return,
            "discard" => GsglSyntaxKind::Discard,
            "true" => GsglSyntaxKind::True,
            "false" => GsglSyntaxKind::False,
            "float" => GsglSyntaxKind::Float,
            "int" => GsglSyntaxKind::Int,
            "bool" => GsglSyntaxKind::Bool,
            "vec2" => GsglSyntaxKind::Vec2,
            "vec3" => GsglSyntaxKind::Vec3,
            "vec4" => GsglSyntaxKind::Vec4,
            "mat2" => GsglSyntaxKind::Mat2,
            "mat3" => GsglSyntaxKind::Mat3,
            "mat4" => GsglSyntaxKind::Mat4,
            "sampler2D" => GsglSyntaxKind::Sampler2D,
            "samplerCube" => GsglSyntaxKind::SamplerCube,
            "void" => GsglSyntaxKind::Void,
            "sin" => GsglSyntaxKind::Sin,
            "cos" => GsglSyntaxKind::Cos,
            "tan" => GsglSyntaxKind::Tan,
            "sqrt" => GsglSyntaxKind::Sqrt,
            "pow" => GsglSyntaxKind::Pow,
            "abs" => GsglSyntaxKind::Abs,
            "min" => GsglSyntaxKind::Min,
            "max" => GsglSyntaxKind::Max,
            "clamp" => GsglSyntaxKind::Clamp,
            "mix" => GsglSyntaxKind::Mix,
            "step" => GsglSyntaxKind::Step,
            "smoothstep" => GsglSyntaxKind::Smoothstep,
            "length" => GsglSyntaxKind::Length,
            "distance" => GsglSyntaxKind::Distance,
            "dot" => GsglSyntaxKind::DotFunc,
            "cross" => GsglSyntaxKind::Cross,
            "normalize" => GsglSyntaxKind::Normalize,
            "reflect" => GsglSyntaxKind::Reflect,
            "refract" => GsglSyntaxKind::Refract,
            "texture2D" => GsglSyntaxKind::Texture2D,
            "textureCube" => GsglSyntaxKind::TextureCube,
            _ => GsglSyntaxKind::Identifier,
        }
    }

    /// 处理预处理器指令
    fn lex_preprocessor(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some('#') {
            state.advance(1);

            // 读取预处理器指令名称
            while let Some(ch) = state.peek() {
                if ch.is_ascii_alphabetic() {
                    state.advance(1);
                }
                else {
                    break;
                }
            }

            // 读取到行尾
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            let text = source.get_text_in((start_pos..state.get_position()).into()).unwrap_or("");
            let kind = match text {
                s if s.starts_with("#include") => GsglSyntaxKind::Include,
                s if s.starts_with("#define") => GsglSyntaxKind::Define,
                s if s.starts_with("#ifdef") => GsglSyntaxKind::Ifdef,
                s if s.starts_with("#ifndef") => GsglSyntaxKind::Ifndef,
                s if s.starts_with("#endif") => GsglSyntaxKind::Endif,
                s if s.starts_with("#version") => GsglSyntaxKind::Version,
                _ => GsglSyntaxKind::Preprocessor,
            };

            state.add_token(kind, start_pos, state.get_position());
            return true;
        }

        false
    }

    /// 处理操作符和分隔
    fn lex_operator_or_delimiter(&self, state: &mut State, _source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '+' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        GsglSyntaxKind::PlusAssign
                    }
                    else {
                        GsglSyntaxKind::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        GsglSyntaxKind::MinusAssign
                    }
                    else {
                        GsglSyntaxKind::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        GsglSyntaxKind::StarAssign
                    }
                    else {
                        GsglSyntaxKind::Star
                    }
                }
                '/' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        GsglSyntaxKind::SlashAssign
                    }
                    else {
                        GsglSyntaxKind::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    GsglSyntaxKind::Percent
                }
                '=' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        GsglSyntaxKind::Eq
                    }
                    else {
                        GsglSyntaxKind::Assign
                    }
                }
                '!' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        GsglSyntaxKind::Ne
                    }
                    else {
                        GsglSyntaxKind::Not
                    }
                }
                '<' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        GsglSyntaxKind::Le
                    }
                    else if state.peek() == Some('<') {
                        state.advance(1);
                        GsglSyntaxKind::LeftShift
                    }
                    else {
                        GsglSyntaxKind::Lt
                    }
                }
                '>' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        GsglSyntaxKind::Ge
                    }
                    else if state.peek() == Some('>') {
                        state.advance(1);
                        GsglSyntaxKind::RightShift
                    }
                    else {
                        GsglSyntaxKind::Gt
                    }
                }
                '&' => {
                    state.advance(1);
                    if state.peek() == Some('&') {
                        state.advance(1);
                        GsglSyntaxKind::And
                    }
                    else {
                        GsglSyntaxKind::BitAnd
                    }
                }
                '|' => {
                    state.advance(1);
                    if state.peek() == Some('|') {
                        state.advance(1);
                        GsglSyntaxKind::Or
                    }
                    else {
                        GsglSyntaxKind::BitOr
                    }
                }
                '^' => {
                    state.advance(1);
                    GsglSyntaxKind::BitXor
                }
                '~' => {
                    state.advance(1);
                    GsglSyntaxKind::BitNot
                }
                '(' => {
                    state.advance(1);
                    GsglSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    GsglSyntaxKind::RightParen
                }
                '{' => {
                    state.advance(1);
                    GsglSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    GsglSyntaxKind::RightBrace
                }
                '[' => {
                    state.advance(1);
                    GsglSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    GsglSyntaxKind::RightBracket
                }
                ';' => {
                    state.advance(1);
                    GsglSyntaxKind::Semicolon
                }
                ',' => {
                    state.advance(1);
                    GsglSyntaxKind::Comma
                }
                '.' => {
                    state.advance(1);
                    GsglSyntaxKind::Dot
                }
                ':' => {
                    state.advance(1);
                    GsglSyntaxKind::Colon
                }
                '?' => {
                    state.advance(1);
                    GsglSyntaxKind::Question
                }
                _ => return false,
            };

            state.add_token(kind, start_pos, state.get_position());
            return true;
        }

        false
    }
}

impl<'config> Lexer<GsglLanguage> for GsglLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<GsglSyntaxKind> {
        let mut state = State::new(source);

        while state.not_at_end() {
            // 跳过空白字符
            if self.skip_whitespace(&mut state) {
                continue;
            }

            // 处理换行
            if self.lex_newline(&mut state) {
                continue;
            }

            // 处理注释
            if self.lex_comment(&mut state, source) {
                continue;
            }

            // 处理预处理器指令
            if self.lex_preprocessor(&mut state, source) {
                continue;
            }

            // 处理字符串字面量
            if self.lex_string(&mut state, source) {
                continue;
            }

            // 处理数字字面量
            if self.lex_number(&mut state, source) {
                continue;
            }

            // 处理标识符或关键字
            if self.lex_identifier_or_keyword(&mut state, source) {
                continue;
            }

            // 处理操作符和分隔符
            if self.lex_operator_or_delimiter(&mut state, source) {
                continue;
            }

            // 如果都不匹配，则为错误字符
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(GsglSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let pos = state.get_position();
        state.add_token(GsglSyntaxKind::Eof, pos, pos);

        state.finish()
    }
}

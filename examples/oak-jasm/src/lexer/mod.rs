use crate::{language::JasmLanguage, syntax::JasmSyntaxKind};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, JasmLanguage>;

static JASM_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static JASM_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["//"] });
static JASM_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });

#[derive(Clone)]
pub struct JasmLexer<'config> {
    config: &'config JasmLanguage,
}

impl<'config> Lexer<JasmLanguage> for JasmLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<JasmLanguage>,
    ) -> LexOutput<JasmLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> JasmLexer<'config> {
    pub fn new(config: &'config JasmLanguage) -> Self {
        Self { config }
    }

    /// 主要的词法分析循环
    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.skip_comment(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_punctuation(state) {
                continue;
            }

            state.safe_check(safe_point);
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(JasmSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    /// 跳过空白字符（不包括换行符）
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start {
            state.add_token(JasmSyntaxKind::Whitespace, start, state.get_position());
            return true;
        }
        false
    }

    /// 处理换行
    fn lex_newline<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if state.current() == Some('\n') {
            state.advance(1);
            state.add_token(JasmSyntaxKind::Newline, start, state.get_position());
            return true;
        }
        false
    }

    /// 跳过注释
    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        if rest.starts_with("//") {
            // 跳过注释标记
            state.advance(2);

            // 读取到行尾
            while let Some(ch) = state.peek() {
                if ch != '\n' {
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }

            state.add_token(JasmSyntaxKind::Comment, start, state.get_position());
            return true;
        }
        false
    }

    /// 处理字符串字面量
    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if state.current() == Some('"') {
            // 跳过开始引号
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch != '"' {
                    if ch == '\\' {
                        state.advance(1); // 转义字符
                        if let Some(_) = state.peek() {
                            state.advance(1); // 被转义的字符
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }
                else {
                    // 找到结束引号
                    state.advance(1);
                    break;
                }
            }

            state.add_token(JasmSyntaxKind::StringLiteral, start, state.get_position());
            return true;
        }
        false
    }

    /// 处理数字字面量
    fn lex_number_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let first = match state.current() {
            Some(c) => c,
            None => return false,
        };

        // 检查是否以数字或负号开始
        if !first.is_ascii_digit() && first != '-' && first != '+' {
            return false;
        }

        // 如果是符号，检查后面是否跟数字
        if first == '-' || first == '+' {
            if let Some(next) = state.peek_next_n(1) {
                if !next.is_ascii_digit() {
                    return false;
                }
            }
            else {
                return false;
            }
        }

        state.advance(1);
        let mut has_dot = false;
        let mut has_exp = false;

        while let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);
            }
            else if ch == '.' && !has_dot && !has_exp {
                has_dot = true;
                state.advance(1);
            }
            else if (ch == 'e' || ch == 'E') && !has_exp {
                has_exp = true;
                state.advance(1);
                // 处理指数符号
                if let Some(sign) = state.peek() {
                    if sign == '+' || sign == '-' {
                        state.advance(1);
                    }
                }
            }
            else {
                break;
            }
        }

        // 检查是否为有效数字
        let end = state.get_position();
        let text = state.get_text_in((start..end).into());

        // 简单验证：不能只是符号或只是点
        if text == "-" || text == "+" || text == "." {
            // 回退
            state.set_position(start);
            return false;
        }

        state.add_token(JasmSyntaxKind::Number, start, state.get_position());
        true
    }

    /// 处理标识符或关键字
    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let ch = match state.current() {
            Some(c) => c,
            None => return false,
        };

        // 标识符必须以字母或下划线开始
        if !(ch.is_ascii_alphabetic() || ch == '_') {
            return false;
        }

        state.advance(1);
        while let Some(c) = state.current() {
            if c.is_ascii_alphanumeric() || c == '_' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        let end = state.get_position();
        let text = state.get_text_in((start..end).into());

        // 检查是否为关键字或指令
        let kind = self.classify_identifier(text);
        state.add_token(kind, start, state.get_position());
        true
    }

    /// 分类标识符为关键字、指令或普通标识符
    fn classify_identifier(&self, text: &str) -> JasmSyntaxKind {
        match text {
            // 关键字
            "class" => JasmSyntaxKind::ClassKw,
            "version" => JasmSyntaxKind::VersionKw,
            "method" => JasmSyntaxKind::MethodKw,
            "field" => JasmSyntaxKind::FieldKw,
            "string" => JasmSyntaxKind::StringKw,
            "sourcefile" => JasmSyntaxKind::SourceFileKw,
            "stack" => JasmSyntaxKind::StackKw,
            "locals" => JasmSyntaxKind::LocalsKw,
            "end" => JasmSyntaxKind::EndKw,
            "compiled" => JasmSyntaxKind::CompiledKw,
            "from" => JasmSyntaxKind::FromKw,
            "innerclass" => JasmSyntaxKind::InnerClassKw,
            "nestmembers" => JasmSyntaxKind::NestMembersKw,
            "bootstrapmethod" => JasmSyntaxKind::BootstrapMethodKw,

            // 访问修饰符
            "public" => JasmSyntaxKind::Public,
            "private" => JasmSyntaxKind::Private,
            "protected" => JasmSyntaxKind::Protected,
            "static" => JasmSyntaxKind::Static,
            "super" => JasmSyntaxKind::Super,
            "final" => JasmSyntaxKind::Final,
            "abstract" => JasmSyntaxKind::Abstract,
            "synchronized" => JasmSyntaxKind::Synchronized,
            "native" => JasmSyntaxKind::Native,
            "synthetic" => JasmSyntaxKind::Synthetic,
            "deprecated" => JasmSyntaxKind::Deprecated,
            "varargs" => JasmSyntaxKind::Varargs,

            // 字节码指令
            "aload_0" => JasmSyntaxKind::ALoad0,
            "aload_1" => JasmSyntaxKind::ALoad1,
            "aload_2" => JasmSyntaxKind::ALoad2,
            "aload_3" => JasmSyntaxKind::ALoad3,
            "iload_0" => JasmSyntaxKind::ILoad0,
            "iload_1" => JasmSyntaxKind::ILoad1,
            "iload_2" => JasmSyntaxKind::ILoad2,
            "iload_3" => JasmSyntaxKind::ILoad3,
            "ldc" => JasmSyntaxKind::Ldc,
            "ldc_w" => JasmSyntaxKind::LdcW,
            "ldc2_w" => JasmSyntaxKind::Ldc2W,
            "invokespecial" => JasmSyntaxKind::InvokeSpecial,
            "invokevirtual" => JasmSyntaxKind::InvokeVirtual,
            "invokestatic" => JasmSyntaxKind::InvokeStatic,
            "invokeinterface" => JasmSyntaxKind::InvokeInterface,
            "invokedynamic" => JasmSyntaxKind::InvokeDynamic,
            "getstatic" => JasmSyntaxKind::GetStatic,
            "putstatic" => JasmSyntaxKind::PutStatic,
            "getfield" => JasmSyntaxKind::GetField,
            "putfield" => JasmSyntaxKind::PutField,
            "return" => JasmSyntaxKind::Return,
            "ireturn" => JasmSyntaxKind::IReturn,
            "areturn" => JasmSyntaxKind::AReturn,
            "lreturn" => JasmSyntaxKind::LReturn,
            "freturn" => JasmSyntaxKind::FReturn,
            "dreturn" => JasmSyntaxKind::DReturn,
            "nop" => JasmSyntaxKind::Nop,
            "dup" => JasmSyntaxKind::Dup,
            "pop" => JasmSyntaxKind::Pop,
            "new" => JasmSyntaxKind::New,

            // 默认为标识符
            _ => JasmSyntaxKind::IdentifierToken,
        }
    }

    /// 处理标点符号
    fn lex_punctuation<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.current() {
            let kind = match ch {
                '{' => JasmSyntaxKind::LeftBrace,
                '}' => JasmSyntaxKind::RightBrace,
                '(' => JasmSyntaxKind::LeftParen,
                ')' => JasmSyntaxKind::RightParen,
                '[' => JasmSyntaxKind::LeftBracket,
                ']' => JasmSyntaxKind::RightBracket,
                ':' => JasmSyntaxKind::Colon,
                ';' => JasmSyntaxKind::Semicolon,
                '.' => JasmSyntaxKind::Dot,
                ',' => JasmSyntaxKind::Comma,
                '/' => JasmSyntaxKind::Slash,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(kind, start, state.get_position());
            return true;
        }

        false
    }
}

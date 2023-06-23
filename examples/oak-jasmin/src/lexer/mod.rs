#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::JasminLanguage, lexer::token_type::JasminTokenType};
use oak_core::{Lexer, LexerCache, LexerState, OakError, TextEdit, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, JasminLanguage>;

/// Jasmin 词法分析器
#[derive(Clone)]
pub struct JasminLexer<'config> {
    _config: &'config JasminLanguage,
}

impl<'config> JasminLexer<'config> {
    pub fn new(config: &'config JasminLanguage) -> Self {
        Self { _config: config }
    }

    /// 判断是关键字还是标识符
    fn keyword_or_identifier(&self, text: &str) -> JasminTokenType {
        match text {
            ".class" => JasminTokenType::ClassKw,
            ".version" => JasminTokenType::VersionKw,
            ".method" => JasminTokenType::MethodKw,
            ".field" => JasminTokenType::FieldKw,
            ".string" => JasminTokenType::StringKw,
            ".source" => JasminTokenType::SourceFileKw,
            ".stack" => JasminTokenType::StackKw,
            ".locals" => JasminTokenType::LocalsKw,
            ".end" => JasminTokenType::EndKw,
            ".compiled" => JasminTokenType::CompiledKw,
            ".from" => JasminTokenType::FromKw,
            ".inner" => JasminTokenType::InnerClassKw,
            ".nest" => JasminTokenType::NestMembersKw,
            ".bootstrap" => JasminTokenType::BootstrapMethodKw,

            "public" => JasminTokenType::Public,
            "private" => JasminTokenType::Private,
            "protected" => JasminTokenType::Protected,
            "static" => JasminTokenType::Static,
            "super" => JasminTokenType::Super,
            "final" => JasminTokenType::Final,
            "abstract" => JasminTokenType::Abstract,
            "synchronized" => JasminTokenType::Synchronized,
            "native" => JasminTokenType::Native,
            "synthetic" => JasminTokenType::Synthetic,
            "deprecated" => JasminTokenType::Deprecated,
            "varargs" => JasminTokenType::Varargs,

            "aload_0" => JasminTokenType::ALoad0,
            "aload_1" => JasminTokenType::ALoad1,
            "aload_2" => JasminTokenType::ALoad2,
            "aload_3" => JasminTokenType::ALoad3,
            "iload_0" => JasminTokenType::ILoad0,
            "iload_1" => JasminTokenType::ILoad1,
            "iload_2" => JasminTokenType::ILoad2,
            "iload_3" => JasminTokenType::ILoad3,
            "ldc" => JasminTokenType::Ldc,
            "ldc_w" => JasminTokenType::LdcW,
            "ldc2_w" => JasminTokenType::Ldc2W,
            "invokespecial" => JasminTokenType::InvokeSpecial,
            "invokevirtual" => JasminTokenType::InvokeVirtual,
            "invokestatic" => JasminTokenType::InvokeStatic,
            "invokeinterface" => JasminTokenType::InvokeInterface,
            "invokedynamic" => JasminTokenType::InvokeDynamic,
            "getstatic" => JasminTokenType::GetStatic,
            "putstatic" => JasminTokenType::PutStatic,
            "getfield" => JasminTokenType::GetField,
            "putfield" => JasminTokenType::PutField,
            "return" => JasminTokenType::Return,
            "areturn" => JasminTokenType::AReturn,
            "ireturn" => JasminTokenType::IReturn,
            "pop" => JasminTokenType::Pop,
            "new" => JasminTokenType::New,

            _ => {
                // 检查是否是类型描述符
                if self.is_type_descriptor(text) { JasminTokenType::TypeDescriptor } else { JasminTokenType::IdentifierToken }
            }
        }
    }

    /// 检查是否是类型描述符
    fn is_type_descriptor(&self, text: &str) -> bool {
        if text.is_empty() {
            return false;
        }

        // 基本类型
        if matches!(text, "B" | "C" | "D" | "F" | "I" | "J" | "S" | "Z" | "V") {
            return true;
        }

        // 数组类型
        if text.starts_with('[') {
            return true;
        }

        // 对象类型
        if text.starts_with('L') && text.ends_with(';') {
            return true;
        }

        false
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();
        let mut consumed = false;

        while let Some(ch) = state.peek() {
            if ch.is_whitespace() {
                consumed = true;
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if consumed {
            state.add_token(JasminTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 跳过注释
    fn skip_comment<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch == ';' {
                let start_pos = state.get_position();
                // 跳过到行尾
                while let Some(ch) = state.peek() {
                    state.advance(ch.len_utf8());
                    if ch == '\n' {
                        break;
                    }
                }
                state.add_token(JasminTokenType::Comment, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理字符串字面量
    fn lex_string<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch == '"' {
                let start_pos = state.get_position();
                state.advance(1); // 跳过开始的引号

                while let Some(ch) = state.peek() {
                    if ch == '"' {
                        state.advance(1); // 跳过结束的引号
                        break;
                    }
                    else if ch == '\\' {
                        state.advance(1); // 跳过转义字符
                        if state.peek().is_some() {
                            state.advance(1); // 跳过被转义的字符
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(JasminTokenType::StringLiteral, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理数字字面量
    fn lex_number<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(first) = state.peek() {
            // 只处理以数字开头的情况，简化逻辑
            if !first.is_ascii_digit() {
                return false;
            }

            // 消费数字
            while let Some(ch) = state.peek() {
                if ch.is_ascii_digit() {
                    state.advance(ch.len_utf8());
                }
                else if ch == '.' {
                    // 浮点数
                    state.advance(1);
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                    break;
                }
                else {
                    break;
                }
            }

            state.add_token(JasminTokenType::Number, start_pos, state.get_position());
            return true;
        }

        false
    }

    /// 处理标识符或关键字
    fn lex_identifier_or_keyword<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        let first = match state.peek() {
            Some(ch) => ch,
            None => return false,
        };

        // 标识符必须以字母、下划线或点开始
        if !first.is_ascii_alphabetic() && first != '_' && first != '.' {
            return false;
        }

        // 消费第一个字符
        state.advance(first.len_utf8());

        // 消费后续字符
        while let Some(ch) = state.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '/' || ch == '$' || ch == '<' || ch == '>' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        let end = state.get_position();
        let text = state.get_text_in(oak_core::Range { start, end });
        let kind = self.keyword_or_identifier(&text);
        state.add_token(kind, start, state.get_position());
        true
    }

    /// 处理操作符和分隔符
    fn lex_operator_or_delimiter<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        let ch = match state.peek() {
            Some(ch) => ch,
            None => return false,
        };

        let kind = match ch {
            '{' => JasminTokenType::LeftBrace,
            '}' => JasminTokenType::RightBrace,
            '(' => JasminTokenType::LeftParen,
            ')' => JasminTokenType::RightParen,
            '[' => JasminTokenType::LeftBracket,
            ']' => JasminTokenType::RightBracket,
            ':' => JasminTokenType::Colon,
            ';' => JasminTokenType::Semicolon,
            '.' => JasminTokenType::Dot,
            ',' => JasminTokenType::Comma,
            '/' => JasminTokenType::Slash,
            _ => return false,
        };

        state.advance(ch.len_utf8());
        state.add_token(kind, start, state.get_position());
        true
    }

    /// 主要的词法分析循环
    fn run<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            // 尝试各种词法规则
            if self.skip_whitespace(state) {
                continue;
            }

            if self.skip_comment(state) {
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
                state.add_token(JasminTokenType::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }
}

impl<'config> Lexer<JasminLanguage> for JasminLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<JasminLanguage>) -> LexOutput<JasminLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

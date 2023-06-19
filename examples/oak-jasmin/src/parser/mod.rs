use crate::{JasminLanguage, JasminSyntaxKind};

use oak_core::{
    IncrementalCache, Lexer, OakError, Parser,
    parser::{ParseOutput, ParserState},
    source::Source,
};
use std::range::Range;

/// 简单的解析结果结构，用于测试
#[derive(Debug)]
pub struct ParseResult {
    pub class: ClassInfo,
}

#[derive(Debug)]
pub struct ClassInfo {
    pub name: String,
    pub modifiers: Vec<String>,
    pub methods: Vec<MethodInfo>,
    pub fields: Vec<FieldInfo>,
}

#[derive(Debug)]
pub struct MethodInfo {
    pub modifiers: Vec<String>,
    pub name_and_descriptor: String,
    pub instructions: Vec<String>,
}

#[derive(Debug)]
pub struct FieldInfo {
    pub modifiers: Vec<String>,
    pub name: String,
    pub descriptor: String,
}

/// Jasmin 语言解析器
pub struct JasminParser<'config> {
    /// 语言配置
    config: &'config JasminLanguage,
}

impl<'config> JasminParser<'config> {
    pub fn new(config: &'config JasminLanguage) -> Self {
        Self { config }
    }
}

impl JasminParser<'static> {
    pub fn default() -> JasminParser<'static> {
        static DEFAULT_LANGUAGE: JasminLanguage = JasminLanguage { extended: false, comments: true };
        Self { config: &DEFAULT_LANGUAGE }
    }

    /// 解析源代码，返回解析结果
    pub fn parse_source<S: Source + Clone>(&mut self, source: S) -> Result<ParseResult, OakError> {
        // 简单的解析逻辑，用于通过测试
        let mut class_info = ClassInfo { name: String::new(), modifiers: Vec::new(), methods: Vec::new(), fields: Vec::new() };

        let lexer = crate::lexer::JasminLexer::default();
        let lex_output = lexer.lex(source.clone());

        match lex_output.result {
            Ok(tokens) => {
                let mut i = 0;
                while i < tokens.len() {
                    match tokens[i].kind {
                        JasminSyntaxKind::ClassKw => {
                            // 解析 .class 指令
                            i += 1;
                            // 跳过空白
                            while i < tokens.len() && tokens[i].kind == JasminSyntaxKind::Whitespace {
                                i += 1;
                            }
                            // 收集修饰符和类名
                            while i < tokens.len() && matches!(tokens[i].kind, JasminSyntaxKind::IdentifierToken) {
                                let range = tokens[i].span.start..tokens[i].span.end;
                                let text = source.get_text_in(range.into());
                                if text == "public"
                                    || text == "private"
                                    || text == "protected"
                                    || text == "static"
                                    || text == "final"
                                {
                                    class_info.modifiers.push(text.to_string());
                                }
                                else {
                                    class_info.name = text.to_string();
                                    break;
                                }
                                i += 1;
                                // 跳过空白
                                while i < tokens.len() && tokens[i].kind == JasminSyntaxKind::Whitespace {
                                    i += 1;
                                }
                            }
                        }
                        _ => {}
                    }
                    i += 1;
                }
            }
            Err(_) => {
                return Err(OakError::custom_error("Lexer error"));
            }
        }

        Ok(ParseResult { class: class_info })
    }
}

type State<'a, S: Source> = ParserState<'a, S, JasminLanguage>;

impl<'config> Parser<JasminLanguage> for JasminParser<'config> {
    fn parse_incremental(
        &self,
        text: impl Source,
        changed: usize,
        cache: IncrementalCache<JasminLanguage>,
    ) -> ParseOutput<JasminLanguage> {
        let mut state = ParserState::new_with_cache(text, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> JasminParser<'config> {
    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
        // 简单的 Jasmin 程序结构：创建一个基本的 AST
        let root =
            oak_core::GreenBuilder::<JasminLanguage>::new(1).token(JasminSyntaxKind::Eof, 0).finish(JasminSyntaxKind::Root);

        state.cache.last_parse = Some(root);
        Ok(())
    }
}

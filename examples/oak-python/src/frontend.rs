use crate::{ast::Program, language::PythonLanguage, parser::PythonParser};
use oak_core::{OakError, Parser, parser::session::ParseSession, source::Source};

/// Python 语言前端
pub struct PythonFrontend<'a, S: Source + ?Sized> {
    source: &'a S,
}

impl<'a, S: Source + ?Sized> PythonFrontend<'a, S> {
    /// 创建新的前端实例
    pub fn new(source: &'a S) -> Self {
        Self { source }
    }

    /// 解析 Python 源代码为 AST
    pub fn parse_to_ast(&self) -> Result<Program, OakError> {
        let config = PythonLanguage {};
        let parser = PythonParser::new(&config);
        let mut cache = ParseSession::<PythonLanguage>::default();

        let output = parser.parse(self.source, &[], &mut cache);

        match output.result {
            Ok(_green_node) => {
                // TODO: 实现从 GreenNode 到 AST 的转换
                // 目前由于 PythonBuilder 尚未完成，我们暂时返回一个空的 Program
                // 或者我们可以尝试手动构建一些基础的 AST 节点
                Ok(Program { statements: vec![] })
            }
            Err(e) => Err(e),
        }
    }
}

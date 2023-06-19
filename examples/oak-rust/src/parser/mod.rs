use crate::{RustLanguage, RustSyntaxKind};
use oak_core::{
    Builder, GreenNode, IncrementalCache, Lexer, OakError, Parser, SyntaxKind,
    errors::OakDiagnostics,
    parser::{OperatorInfo, ParserState, PrattParser},
    source::Source,
    tree::{Arc, GreenLeaf, GreenTree},
};
mod parse;

/// Rust 语言解析器（不可变），通过 &mut ParserState 推进
pub struct RustParser<'config> {
    /// 语言配置
    config: &'config RustLanguage,
    /// 表达式解析器（启用 Pratt）
    pratt: PrattParser<RustLanguage>,
}

impl<'config> RustParser<'config> {
    pub fn new(config: &'config RustLanguage) -> Self {
        let mut pratt = PrattParser::<RustLanguage>::new();
        Self::configure_operators(&mut pratt);
        Self { config, pratt }
    }

    /// 配置操作符优先级
    fn configure_operators(pratt: &mut PrattParser<RustLanguage>) {
        use RustSyntaxKind::*;
        // 赋值（右结合，最低优先级）
        pratt
            .infix(Eq, OperatorInfo::right(10), BinaryExpression)
            .infix(PlusEq, OperatorInfo::right(10), BinaryExpression)
            .infix(MinusEq, OperatorInfo::right(10), BinaryExpression)
            .infix(StarEq, OperatorInfo::right(10), BinaryExpression)
            .infix(SlashEq, OperatorInfo::right(10), BinaryExpression)
            .infix(PercentEq, OperatorInfo::right(10), BinaryExpression)
            .infix(CaretEq, OperatorInfo::right(10), BinaryExpression)
            .infix(AndEq, OperatorInfo::right(10), BinaryExpression)
            .infix(OrEq, OperatorInfo::right(10), BinaryExpression)
            .infix(ShlEq, OperatorInfo::right(10), BinaryExpression)
            .infix(ShrEq, OperatorInfo::right(10), BinaryExpression);

        // 逻辑或/与
        pratt.infix(OrOr, OperatorInfo::left(14), BinaryExpression).infix(AndAnd, OperatorInfo::left(15), BinaryExpression);

        // 位运算：或、异或、与
        pratt.infix(Or, OperatorInfo::left(20), BinaryExpression).infix(Caret, OperatorInfo::left(21), BinaryExpression).infix(
            And,
            OperatorInfo::left(22),
            BinaryExpression,
        );

        // 比较（非结合）
        pratt
            .infix(EqEq, OperatorInfo::none(25), BinaryExpression)
            .infix(Ne, OperatorInfo::none(25), BinaryExpression)
            .infix(Lt, OperatorInfo::none(30), BinaryExpression)
            .infix(Gt, OperatorInfo::none(30), BinaryExpression)
            .infix(Le, OperatorInfo::none(30), BinaryExpression)
            .infix(Ge, OperatorInfo::none(30), BinaryExpression);

        // 位移
        pratt.infix(Shl, OperatorInfo::left(35), BinaryExpression).infix(Shr, OperatorInfo::left(35), BinaryExpression);

        // 加减
        pratt.infix(Plus, OperatorInfo::left(40), BinaryExpression).infix(Minus, OperatorInfo::left(40), BinaryExpression);

        // 乘除取余
        pratt
            .infix(Star, OperatorInfo::left(50), BinaryExpression)
            .infix(Slash, OperatorInfo::left(50), BinaryExpression)
            .infix(Percent, OperatorInfo::left(50), BinaryExpression);

        // 前缀：逻辑非、正负、解引用、借用
        pratt
            .prefix(Not, OperatorInfo::right(60), UnaryExpression)
            .prefix(Minus, OperatorInfo::right(60), UnaryExpression)
            .prefix(Plus, OperatorInfo::right(60), UnaryExpression)
            .prefix(Star, OperatorInfo::right(60), UnaryExpression)
            .prefix(And, OperatorInfo::right(60), UnaryExpression);
    }
}

impl<'config> Clone for RustParser<'config> {
    fn clone(&self) -> Self {
        RustParser::new(self.config)
    }
}

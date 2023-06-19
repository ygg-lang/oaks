use crate::{ast::SqlRoot, kind::SqlSyntaxKind};
use oak_core::Language;

/// SQL 语言实现
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SqlLanguage {
    /// 是否区分大小
    pub case_sensitive: bool,
    /// 是否允许双引号标识符
    pub quoted_identifiers: bool,
    /// 是否允许反引号标识符
    pub backtick_identifiers: bool,
    /// 是否允许方括号标识符
    pub bracket_identifiers: bool,
}

impl SqlLanguage {
    /// 创建标准 SQL 语言实例
    pub fn standard() -> Self {
        Self::default()
    }

    /// 创建 MySQL 风格SQL 语言实例
    pub fn mysql() -> Self {
        Self { case_sensitive: false, quoted_identifiers: true, backtick_identifiers: true, bracket_identifiers: false }
    }

    /// 创建 PostgreSQL 风格SQL 语言实例
    pub fn postgresql() -> Self {
        Self { case_sensitive: false, quoted_identifiers: true, backtick_identifiers: false, bracket_identifiers: false }
    }

    /// 创建 SQL Server 风格SQL 语言实例
    pub fn sqlserver() -> Self {
        Self { case_sensitive: false, quoted_identifiers: true, backtick_identifiers: false, bracket_identifiers: true }
    }
}

impl Default for SqlLanguage {
    fn default() -> Self {
        Self { case_sensitive: false, quoted_identifiers: true, backtick_identifiers: false, bracket_identifiers: false }
    }
}

impl Language for SqlLanguage {
    type SyntaxKind = SqlSyntaxKind;
    type TypedRoot = SqlRoot;
}

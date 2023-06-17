use crate::kind::TclSyntaxKind;
use oak_core::Language;

/// Tcl 语言配置
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TclLanguage {
    /// Tcl 版本
    pub version: TclVersion,
    /// 是否启用扩展语法
    pub extensions: bool,
}

/// Tcl 版本
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TclVersion {
    /// Tcl 8.0
    Tcl80,
    /// Tcl 8.1
    Tcl81,
    /// Tcl 8.2
    Tcl82,
    /// Tcl 8.3
    Tcl83,
    /// Tcl 8.4
    Tcl84,
    /// Tcl 8.5
    Tcl85,
    /// Tcl 8.6
    Tcl86,
    /// Tcl 8.7
    Tcl87,
    /// Tcl 9.0
    Tcl90,
}

impl TclLanguage {
    /// 创建标准 Tcl 8.6 配置
    pub fn standard() -> Self {
        Self { version: TclVersion::Tcl86, extensions: false }
    }

    /// 创建 Tcl 8.5 配置
    pub fn tcl85() -> Self {
        Self { version: TclVersion::Tcl85, extensions: false }
    }

    /// 创建 Tcl 8.6 配置
    pub fn tcl86() -> Self {
        Self { version: TclVersion::Tcl86, extensions: false }
    }

    /// 创建 Tcl 9.0 配置
    pub fn tcl90() -> Self {
        Self { version: TclVersion::Tcl90, extensions: true }
    }

    /// 创建支持扩展语法的 Tcl 配置
    pub fn with_extensions() -> Self {
        Self { version: TclVersion::Tcl86, extensions: true }
    }
}

impl Language for TclLanguage {
    type SyntaxKind = TclSyntaxKind;
}

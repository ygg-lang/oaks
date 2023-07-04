use crate::{ast::ValkyrieRoot, lexer::ValkyrieTokenType, parser::ValkyrieElementType};
use oak_core::{Language, LanguageCategory};

/// Valkyrie language configuration and metadata.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValkyrieLanguage {
    /// Support [t_grammar](crate::ast::Trammar).
    pub support_t_grammar: bool,
    /// Support [x_grammar](crate::ast::XGrammar).
    pub support_x_grammar: bool,
    /// Support [shader](crate::ast::ShaderDeclaration) declaration.
    pub support_shader_extension: bool,
    /// Support [model](crate::ast::ModelDeclaration) declaration.
    pub support_orm_extension: bool,
    /// Support [model](crate::ast::ModelDeclaration) declaration.
    pub support_ecs_extension: bool,
    /// Allow legacy [struct](crate::ast::StructureDeclaration) syntax.
    pub allow_legacy_struct: bool,
    /// Allow legacy [for-loop](crate::ast::LoopKind::For) syntax.
    pub allow_legacy_for: bool,
    /// Allow legacy [fn](crate::ast::LoopKind::For), [fun](crate::ast::LoopKind::For), [func](crate::ast::LoopKind::For), [function](crate::ast::LoopKind::For) syntax.
    pub allow_legacy_function: bool,
}

impl ValkyrieLanguage {
    /// Creates a new Valkyrie language configuration.
    pub fn shader_language() -> Self {
        Self { support_shader_extension: true, ..Default::default() }
    }
    /// Creates a new Valkyrie language configuration.
    pub fn model_language() -> Self {
        Self { support_orm_extension: true, ..Default::default() }
    }
    /// Creates a new Valkyrie language configuration with shader support enabled.
    pub fn with_shader_support(self) -> Self {
        Self { support_shader_extension: true, ..self }
    }
    /// Creates a new Valkyrie language configuration with model support enabled.
    pub fn with_model_support(self) -> Self {
        Self { support_orm_extension: true, ..self }
    }
    /// Creates a new Valkyrie language configuration with ECS support enabled.
    pub fn ecs_language() -> Self {
        Self { support_ecs_extension: true, ..Default::default() }
    }
    /// Creates a new Valkyrie language configuration with ECS support enabled.
    pub fn with_ecs_support(self) -> Self {
        Self { support_ecs_extension: true, ..self }
    }
}

impl Default for ValkyrieLanguage {
    fn default() -> Self {
        Self { support_shader_extension: false, support_orm_extension: false, support_ecs_extension: false, support_x_grammar: false, support_t_grammar: true, allow_legacy_for: false, allow_legacy_function: false, allow_legacy_struct: false }
    }
}

impl Language for ValkyrieLanguage {
    const NAME: &'static str = "valkyrie";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = ValkyrieTokenType;
    type ElementType = ValkyrieElementType;
    type TypedRoot = ValkyrieRoot;
}

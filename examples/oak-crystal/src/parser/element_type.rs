use crate::lexer::CrystalTokenType;
use oak_core::{ElementType, UniversalElementRole};
use serde::{Deserialize, Serialize};

/// Crystal element type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CrystalElementType {
    /// Root node
    Root,
    /// Program node
    Program,
    /// Source file node
    SourceFile,
    /// class definition
    ClassDef,
    /// module definition
    ModuleDef,
    /// method definition
    MethodDef,
    /// block
    Block,
    /// if expression
    IfExpr,
    /// unless expression
    UnlessExpr,
    /// case expression
    CaseExpr,
    /// when clause
    WhenClause,
    /// while expression
    WhileExpr,
    /// until expression
    UntilExpr,
    /// for expression
    ForExpr,
    /// begin expression
    BeginExpr,
    /// rescue clause
    RescueClause,
    /// ensure clause
    EnsureClause,
    /// call expression
    CallExpr,
    /// index expression
    IndexExpr,
    /// member expression
    MemberExpr,
    /// binary expression
    BinaryExpr,
    /// unary expression
    UnaryExpr,
    /// assignment expression
    AssignExpr,
    /// literal expression
    LiteralExpr,
    /// identifier expression
    IdentifierExpr,
    /// array expression
    ArrayExpr,
    /// hash expression
    HashExpr,
    /// hash pair
    HashPair,
    /// block expression
    BlockExpr,
    /// lambda expression
    LambdaExpr,
    /// yield expression
    YieldExpr,
    /// return expression
    ReturnExpr,
    /// break expression
    BreakExpr,
    /// next expression
    NextExpr,
    /// super expression
    SuperExpr,
    /// self expression
    SelfExpr,
    /// parenthesized expression
    ParenExpr,
    /// type expression
    TypeExpr,
    /// generic type
    GenericType,
    /// union type
    UnionType,
    /// tuple type
    TupleType,
    /// named tuple type
    NamedTupleType,
    /// procedure type
    ProcType,
    /// pattern
    Pattern,
    /// identifier pattern
    IdentifierPattern,
    /// literal pattern
    LiteralPattern,
    /// array pattern
    ArrayPattern,
    /// hash pattern
    HashPattern,
    /// tuple pattern
    TuplePattern,
    /// parameter list
    ParamList,
    /// parameter
    Param,
    /// splat parameter
    SplatParam,
    /// double splat parameter
    DoubleSplatParam,
    /// block parameter
    BlockParam,
    /// annotation
    Annotation,
    /// macro definition
    MacroDef,
    /// macro call
    MacroCall,
    /// macro expression
    MacroExpr,
    /// alias
    Alias,
    /// include
    Include,
    /// extend
    Extend,
    /// require
    Require,
    /// private
    Private,
    /// protected
    Protected,
    /// public
    Public,
    /// abstract
    Abstract,
    /// virtual
    Virtual,
    /// override
    Override,
    /// struct definition
    StructDef,
    /// enum definition
    EnumDef,
    /// union definition
    UnionDef,
    /// lib definition
    LibDef,
    /// raise expression
    RaiseExpr,
    /// range expression
    RangeExpr,
    /// exclusive range expression
    ExclusiveRangeExpr,
    /// regex literal
    RegexLiteral,
    /// string interpolation
    StringInterpolation,
    /// interpolation expression
    InterpolationExpr,
    /// symbol literal
    SymbolLiteral,
    /// constant reference
    ConstantRef,
    /// instance variable
    InstanceVar,
    /// class variable
    ClassVar,
    /// global variable
    GlobalVar,
    /// getter method
    Getter,
    /// setter method
    Setter,
    /// operator definition
    OperatorDef,
}

impl ElementType for CrystalElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<CrystalTokenType> for CrystalElementType {
    fn from(token: CrystalTokenType) -> Self {
        match token {
            CrystalTokenType::Error => Self::Root, // Default or Error?
            _ => Self::Root,
        }
    }
}

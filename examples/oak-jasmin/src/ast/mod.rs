#![doc = include_str!("readme.md")]

/// Jasmin root node.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JasminRoot {
    /// Class definition.
    pub class: JasminClass,
}

/// Jasmin class declaration AST node.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JasminClass {
    /// Access modifiers (public, private, etc.).
    pub modifiers: Vec<String>,
    /// Class name.
    pub name: String,
    /// Version info (e.g., 65:0).
    pub version: Option<String>,
    /// Method list.
    pub methods: Vec<JasminMethod>,
    /// Field list.
    pub fields: Vec<JasminField>,
    /// Source file info.
    pub source_file: Option<String>,
}

/// Jasmin method declaration AST node.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JasminMethod {
    /// Access modifiers (public, static, etc.).
    pub modifiers: Vec<String>,
    /// Method name and type descriptor (e.g., "main:([Ljava/lang/String;)V").
    pub name_and_descriptor: String,
    /// Stack size.
    pub stack_size: Option<u32>,
    /// Local variables count.
    pub locals_count: Option<u32>,
    /// Instruction list.
    pub instructions: Vec<JasminInstruction>,
}

/// Jasmin field declaration AST node.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JasminField {
    /// Access modifiers (public, static, etc.).
    pub modifiers: Vec<String>,
    /// Field name and type descriptor (e.g., "value:I").
    pub name_and_descriptor: String,
}

/// Jasmin instruction AST node.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum JasminInstruction {
    /// Simple instruction (e.g., aload_0, return).
    Simple(String),
    /// Instruction with argument (e.g., ldc "Hello").
    WithArgument {
        /// Instruction name.
        instruction: String,
        /// Argument.
        argument: String,
    },
    /// Method call instruction (e.g., invokespecial Method java/lang/Object."<init>":"()V").
    MethodCall {
        /// Instruction name.
        instruction: String,
        /// Method reference.
        method_ref: String,
    },
    /// Field access instruction (e.g., getstatic Field java/lang/System.out:"Ljava/io/PrintStream;").
    FieldAccess {
        /// Instruction name.
        instruction: String,
        /// Field reference.
        field_ref: String,
    },
}

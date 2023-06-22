use oak_core::{Arc, language::UniversalElementRole};
use serde::{Deserialize, Serialize};

pub use core::range::Range;
pub use oak_folding::{FoldingRange, FoldingRangeKind};

/// Represents a position in a source file (line and character).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    /// Line number (0-indexed).
    pub line: u32,
    /// Character offset in the line (0-indexed).
    pub character: u32,
}

/// Represents a range in a source file using line/character positions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LspRange {
    /// The start position of the range.
    pub start: Position,
    /// The end position of the range.
    pub end: Position,
}

/// Represents a location inside a resource.
/// Can be either byte-based (Rust model) or line/column based (LSP model).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(bound(serialize = "R: Serialize", deserialize = "R: Deserialize<'de>"))]
pub struct Location<R = Range<usize>> {
    /// The URI of the resource.
    #[serde(with = "oak_core::serde_arc_str")]
    pub uri: Arc<str>,
    /// The range within the resource.
    pub range: R,
}

/// A specialized location type for byte-based ranges.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LocationRange {
    /// The URI of the resource.
    #[serde(with = "oak_core::serde_arc_str")]
    pub uri: Arc<str>,
    /// The byte range within the resource.
    #[serde(with = "oak_core::serde_range")]
    pub range: Range<usize>,
}

impl From<LocationRange> for Location<Range<usize>> {
    fn from(loc: LocationRange) -> Self {
        Self { uri: loc.uri, range: loc.range }
    }
}

impl From<Location<Range<usize>>> for LocationRange {
    fn from(loc: Location<Range<usize>>) -> Self {
        Self { uri: loc.uri, range: loc.range }
    }
}

impl From<oak_navigation::Location> for Location<Range<usize>> {
    fn from(loc: oak_navigation::Location) -> Self {
        Self { uri: loc.uri, range: loc.range }
    }
}

impl From<oak_navigation::Location> for LocationRange {
    fn from(loc: oak_navigation::Location) -> Self {
        Self { uri: loc.uri, range: loc.range }
    }
}

/// A source position with line, column, and offset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SourcePosition {
    /// Line number (1-indexed).
    pub line: u32,
    /// Column number (1-indexed).
    pub column: u32,
    /// Byte offset (0-indexed).
    pub offset: usize,
    /// Length of the token/element.
    pub length: usize,
}

/// A source location with an optional URL.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SourceLocation {
    /// Line number (1-indexed).
    pub line: u32,
    /// Column number (1-indexed).
    pub column: u32,
    /// Optional URL of the source file.
    pub url: Option<url::Url>,
}

impl Default for SourceLocation {
    fn default() -> Self {
        Self { line: 1, column: 1, url: None }
    }
}

/// A document highlight is a range inside a text document which deserves
/// special attention. Usually a document highlight is visualized by changing
/// the background color of its range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DocumentHighlightKind {
    Text = 1,
    Read = 2,
    Write = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DocumentHighlight {
    pub range: LspRange,
    pub kind: Option<DocumentHighlightKind>,
}

/// Represents a color in RGBA space.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

/// Represents a color range from a document.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ColorInformation {
    pub range: LspRange,
    pub color: Color,
}

/// Represents hover information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hover {
    /// The hover's content as a markdown string.
    pub contents: String,
    /// An optional span to which this hover applies.
    #[serde(with = "oak_core::serde_range::option")]
    pub range: Option<Range<usize>>,
}

/// Represents an item in the document structure (e.g., in an outline or breadcrumbs).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureItem {
    /// The name of this item (e.g., function name, class name).
    pub name: String,
    /// More detail about this item (e.g., function signature, type).
    pub detail: Option<String>,
    /// The universal role of this element.
    pub role: UniversalElementRole,
    /// The symbol kind.
    pub kind: SymbolKind,
    /// The range of the entire element in the source code.
    #[serde(with = "oak_core::serde_range")]
    pub range: Range<usize>,
    /// The range that should be selected when clicking on this item.
    /// Usually the range of the identifier.
    #[serde(with = "oak_core::serde_range")]
    pub selection_range: Range<usize>,
    /// Whether this item is deprecated.
    pub deprecated: bool,
    /// Nested structure items (e.g., methods within a class).
    pub children: Vec<StructureItem>,
}

/// Parameters for the `initialize` request.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InitializeParams {
    pub root_uri: Option<String>,
    pub workspace_folders: Vec<WorkspaceFolder>,
}

/// A workspace folder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceFolder {
    pub uri: String,
    pub name: String,
}

/// Represents a symbol kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SymbolKind {
    File = 1,
    Module = 2,
    Namespace = 3,
    Package = 4,
    Class = 5,
    Method = 6,
    Property = 7,
    Field = 8,
    Constructor = 9,
    Enum = 10,
    Interface = 11,
    Function = 12,
    Variable = 13,
    Constant = 14,
    String = 15,
    Number = 16,
    Boolean = 17,
    Array = 18,
    Object = 19,
    Key = 20,
    Null = 21,
    EnumMember = 22,
    Struct = 23,
    Event = 24,
    Operator = 25,
    TypeParameter = 26,
}

/// Represents a workspace symbol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceSymbol {
    /// The name of the symbol.
    pub name: String,
    /// The kind of the symbol.
    pub kind: SymbolKind,
    /// The location of the symbol.
    pub location: LocationRange,
    /// The name of the container this symbol is in.
    pub container_name: Option<String>,
}

/// Represents information about a symbol (e.g., function, variable, class).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolInformation {
    /// The name of the symbol.
    pub name: String,
    /// The kind of the symbol.
    pub kind: SymbolKind,
    /// The location of the symbol.
    pub location: LocationRange,
    /// The name of the container this symbol is in.
    pub container_name: Option<String>,
}

/// Represents a change to the workspace.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkspaceEdit {
    /// The changes to the workspace.
    pub changes: std::collections::HashMap<String, Vec<TextEdit>>,
}

/// Represents a text edit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEdit {
    /// The range of the text edit.
    #[serde(with = "oak_core::serde_range")]
    pub range: Range<usize>,
    /// The new text.
    pub new_text: String,
}

/// Represents a completion item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionItem {
    /// The label of the completion item.
    pub label: String,
    /// The kind of the completion item.
    pub kind: Option<CompletionItemKind>,
    /// A human-readable string with additional information about this item.
    pub detail: Option<String>,
    /// A human-readable string that contains documentation about this item.
    pub documentation: Option<String>,
    /// The text that should be inserted when selecting this completion item.
    pub insert_text: Option<String>,
}

/// Represents a completion item kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompletionItemKind {
    Text = 1,
    Method = 2,
    Function = 3,
    Constructor = 4,
    Field = 5,
    Variable = 6,
    Class = 7,
    Interface = 8,
    Module = 9,
    Property = 10,
    Unit = 11,
    Value = 12,
    Enum = 13,
    Keyword = 14,
    Snippet = 15,
    Color = 16,
    File = 17,
    Reference = 18,
    Folder = 19,
    EnumMember = 20,
    Constant = 21,
    Struct = 22,
    Event = 23,
    Operator = 24,
    TypeParameter = 25,
}

/// Represents a diagnostic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    /// The range of the diagnostic.
    #[serde(with = "oak_core::serde_range")]
    pub range: Range<usize>,
    /// The severity of the diagnostic.
    pub severity: Option<DiagnosticSeverity>,
    /// The diagnostic's code.
    pub code: Option<String>,
    /// The source of the diagnostic.
    pub source: Option<String>,
    /// The diagnostic's message.
    pub message: String,
}

/// Represents a diagnostic severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

/// Represents a semantic token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticToken {
    pub delta_line: u32,
    pub delta_start: u32,
    pub length: u32,
    pub token_type: u32,
    pub token_modifiers_bitset: u32,
}

/// Represents semantic tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticTokens {
    pub result_id: Option<String>,
    pub data: Vec<SemanticToken>,
}

/// Represents a selection range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionRange {
    #[serde(with = "oak_core::serde_range")]
    pub range: Range<usize>,
    pub parent: Option<Box<SelectionRange>>,
}

/// Represents parameter information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterInformation {
    pub label: String,
    pub documentation: Option<String>,
}

/// Represents signature information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureInformation {
    pub label: String,
    pub documentation: Option<String>,
    pub parameters: Option<Vec<ParameterInformation>>,
    pub active_parameter: Option<u32>,
}

/// Represents signature help.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureHelp {
    pub signatures: Vec<SignatureInformation>,
    pub active_signature: Option<u32>,
    pub active_parameter: Option<u32>,
}

/// Represents an inlay hint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlayHint {
    pub position: Position,
    pub label: String,
    pub kind: Option<InlayHintKind>,
    pub tooltip: Option<String>,
    pub padding_left: Option<bool>,
    pub padding_right: Option<bool>,
}

/// Represents an inlay hint kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InlayHintKind {
    Type = 1,
    Parameter = 2,
}

/// Represents a code action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeAction {
    pub title: String,
    pub kind: Option<String>,
    pub diagnostics: Option<Vec<Diagnostic>>,
    pub edit: Option<WorkspaceEdit>,
    pub command: Option<Command>,
    pub is_preferred: Option<bool>,
    pub disabled: Option<CodeActionDisabled>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeActionDisabled {
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub title: String,
    pub command: String,
    pub arguments: Option<Vec<serde_json::Value>>,
}

impl From<UniversalElementRole> for SymbolKind {
    fn from(role: UniversalElementRole) -> Self {
        match role {
            UniversalElementRole::Root => SymbolKind::File,
            UniversalElementRole::Container => SymbolKind::Module,
            UniversalElementRole::Definition => SymbolKind::Function,
            UniversalElementRole::Binding => SymbolKind::Variable,
            UniversalElementRole::Reference => SymbolKind::Variable,
            UniversalElementRole::Typing => SymbolKind::Class,
            UniversalElementRole::Statement => SymbolKind::Function,
            UniversalElementRole::Expression => SymbolKind::Variable,
            UniversalElementRole::Call => SymbolKind::Function,
            UniversalElementRole::Metadata => SymbolKind::Property,
            UniversalElementRole::Attribute => SymbolKind::Property,
            UniversalElementRole::Documentation => SymbolKind::String,
            UniversalElementRole::Value => SymbolKind::Constant,
            UniversalElementRole::Error => SymbolKind::Null,
            _ => SymbolKind::Function,
        }
    }
}

impl From<oak_symbols::SymbolInformation> for WorkspaceSymbol {
    fn from(s: oak_symbols::SymbolInformation) -> Self {
        Self { name: s.name, kind: SymbolKind::from(s.role), location: LocationRange { uri: s.uri, range: s.range }, container_name: s.container_name }
    }
}

impl From<oak_symbols::SymbolInformation> for StructureItem {
    fn from(s: oak_symbols::SymbolInformation) -> Self {
        Self { name: s.name, detail: None, role: s.role, kind: SymbolKind::from(s.role), range: s.range.clone(), selection_range: s.range.clone(), deprecated: false, children: vec![] }
    }
}

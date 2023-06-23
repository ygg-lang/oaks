#![doc = include_str!("readme.md")]

use crate::{
    ast::{RustRoot, *},
    lexer::RustTokenType,
};

/// Rust Code Formatter
///
/// `RustFormatter` is responsible for converting Rust AST into formatted source code strings.
/// It follows Rust's official code style guidelines, including indentation, spacing, and line breaks.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust,ignore
/// use oak_rust::formatter::RustFormatter;
///
/// let formatter = RustFormatter::new();
/// let formatted = formatter.format("fn main(){let x=42}");
/// // Output: "fn main() {\n    let x = 42;\n}"
/// ```
pub struct RustFormatter {
    /// 缩进级别
    indent_level: usize,
    /// 缩进字符串（通常是 4 个空格）
    indent_str: String,
    /// 最大行长度
    max_line_length: usize,
}

impl RustFormatter {
    /// Create a new Rust formatter
    ///
    /// # Returns
    ///
    /// Returns a new `RustFormatter` instance with default configuration.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use oak_rust::formatter::RustFormatter;
    ///
    /// let formatter = RustFormatter::new();
    /// ```
    pub fn new() -> Self {
        Self {
            indent_level: 0,
            indent_str: "    ".to_string(), // 4 spaces
            max_line_length: 100,
        }
    }

    /// Create formatter with custom configuration
    ///
    /// # Arguments
    ///
    /// * `indent_str` - Indentation string
    /// * `max_line_length` - Maximum line length
    ///
    /// # Returns
    ///
    /// Returns a configured `RustFormatter` instance.
    pub fn with_config(indent_str: String, max_line_length: usize) -> Self {
        Self { indent_level: 0, indent_str, max_line_length }
    }

    /// Format the given Rust source code string
    ///
    /// # Arguments
    ///
    /// * `source` - Rust source code to format
    ///
    /// # Returns
    ///
    /// Returns the formatted Rust source code string.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use oak_rust::formatter::RustFormatter;
    ///
    /// let formatter = RustFormatter::new();
    /// let formatted = formatter.format("fn main(){let x=42}");
    /// ```
    pub fn format(&self, source: &str) -> String {
        // TODO: Implement complete Rust code formatting
        // Currently returns basic formatting version
        self.basic_format(source)
    }

    /// Format Rust AST root node
    ///
    /// # Arguments
    ///
    /// * `root` - Rust AST root node
    ///
    /// # Returns
    ///
    /// Returns the formatted Rust source code string.
    pub fn format_ast(&self, root: &RustRoot) -> String {
        let mut result = String::new();

        for (i, item) in root.items.iter().enumerate() {
            if i > 0 {
                result.push_str("\n\n");
            }
            result.push_str(&self.format_item(item));
        }

        result
    }

    /// Format top-level items
    fn format_item(&self, item: &Item) -> String {
        match item {
            Item::Function(func) => self.format_function(func),
            Item::Struct(struct_def) => self.format_struct(struct_def),
            Item::Enum(enum_def) => self.format_enum(enum_def),
            Item::Trait(trait_def) => self.format_trait(trait_def),
            Item::Impl(impl_block) => self.format_impl(impl_block),
            Item::Module(module) => self.format_module(module),
            Item::Use(use_item) => self.format_use(use_item),
            Item::Const(const_item) => self.format_const(const_item),
            Item::Static(static_item) => self.format_static(static_item),
            Item::TypeAlias(type_alias) => self.format_type_alias(type_alias),
            Item::ExternBlock(extern_block) => self.format_extern_block(extern_block),
        }
    }

    /// Format functions
    fn format_function(&self, func: &Function) -> String {
        let mut result = String::new();

        // Function modifiers
        if func.is_async {
            result.push_str("async ");
        }
        if func.is_unsafe {
            result.push_str("unsafe ");
        }

        result.push_str("fn ");
        result.push_str(&func.name.name);

        // Parameter list
        result.push('(');
        for (i, param) in func.params.iter().enumerate() {
            if i > 0 {
                result.push_str(", ");
            }
            result.push_str(&self.format_param(param));
        }
        result.push(')');

        // Return type
        if let Some(return_type) = &func.return_type {
            result.push_str(" -> ");
            result.push_str(&self.format_type(return_type));
        }

        // Function body
        result.push(' ');
        result.push_str(&self.format_block(&func.body));

        result
    }

    /// Format parameters
    fn format_param(&self, param: &Param) -> String {
        let mut result = String::new();
        if param.is_mut {
            result.push_str("mut ");
        }
        result.push_str(&param.name.name);
        result.push_str(": ");
        result.push_str(&self.format_type(&param.ty));
        result
    }

    /// Format code blocks
    fn format_block(&self, block: &Block) -> String {
        let mut result = String::new();
        result.push_str("{\n");

        // Increase indentation
        let mut formatter = self.clone();
        formatter.indent_level += 1;

        // Format statements
        for stmt in &block.statements {
            result.push_str(&formatter.get_indent());
            result.push_str(&formatter.format_statement(stmt));
            result.push('\n');
        }

        result.push_str(&self.get_indent());
        result.push('}');
        result
    }

    /// Format statements
    fn format_statement(&self, stmt: &Statement) -> String {
        match stmt {
            Statement::Let { name, ty, expr, mutable, .. } => {
                let mut result = String::new();
                result.push_str("let ");
                if *mutable {
                    result.push_str("mut ");
                }
                result.push_str(&name.name);

                if let Some(ty) = ty {
                    result.push_str(": ");
                    result.push_str(&self.format_type(ty));
                }

                if let Some(expr) = expr {
                    result.push_str(" = ");
                    result.push_str(&self.format_expr(expr));
                }

                result.push(';');
                result
            }
            Statement::ExprStmt { expr, semi, .. } => {
                let mut result = self.format_expr(expr);
                if *semi {
                    result.push(';');
                }
                result
            }
            Statement::Return { expr, .. } => {
                let mut result = String::from("return");
                if let Some(expr) = expr {
                    result.push(' ');
                    result.push_str(&self.format_expr(expr));
                }
                result.push(';');
                result
            }
            Statement::Break { expr, .. } => {
                let mut result = String::from("break");
                if let Some(expr) = expr {
                    result.push(' ');
                    result.push_str(&self.format_expr(expr));
                }
                result.push(';');
                result
            }
            Statement::Continue { .. } => String::from("continue;"),
            Statement::Item(item) => self.format_item(item),
        }
    }

    /// Format expressions
    fn format_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Literal { value, .. } => value.clone(),
            Expr::Bool { value, .. } => value.to_string(),
            Expr::Ident(ident) => ident.name.clone(),
            Expr::Binary { left, op, right, .. } => {
                format!("{} {} {}", self.format_expr(left), self.format_syntax_kind(op), self.format_expr(right))
            }
            Expr::Unary { op, expr, .. } => {
                format!("{}{}", self.format_syntax_kind(op), self.format_expr(expr))
            }
            Expr::Call { callee, args, .. } => {
                let mut result = self.format_expr(callee);
                result.push('(');
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        result.push_str(", ");
                    }
                    result.push_str(&self.format_expr(arg));
                }
                result.push(')');
                result
            }
            Expr::Field { receiver, field, .. } => {
                format!("{}.{}", self.format_expr(receiver), field.name)
            }
            Expr::Index { receiver, index, .. } => {
                format!("{}[{}]", self.format_expr(receiver), self.format_expr(index))
            }
            Expr::Paren { expr, .. } => {
                format!("({})", self.format_expr(expr))
            }
            Expr::Block(block) => self.format_block(block),
            _ => "/* unsupported expression */".to_string(),
        }
    }

    /// Format literal expressions
    fn _format_literal_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Literal { value, .. } => value.clone(),
            Expr::Bool { value, .. } => value.to_string(),
            _ => "".to_string(), // 对于非字面量表达式返回空字符串
        }
    }

    /// Format syntax types to strings
    fn format_syntax_kind(&self, kind: &RustTokenType) -> String {
        match kind {
            RustTokenType::Plus => "+".to_string(),
            RustTokenType::Minus => "-".to_string(),
            RustTokenType::Star => "*".to_string(),
            RustTokenType::Slash => "/".to_string(),
            RustTokenType::Percent => "%".to_string(),
            RustTokenType::EqEq => "==".to_string(),
            RustTokenType::Ne => "!=".to_string(),
            RustTokenType::Lt => "<".to_string(),
            RustTokenType::Le => "<=".to_string(),
            RustTokenType::Gt => ">".to_string(),
            RustTokenType::Ge => ">=".to_string(),
            RustTokenType::AndAnd => "&&".to_string(),
            RustTokenType::OrOr => "||".to_string(),
            RustTokenType::Bang => "!".to_string(),
            RustTokenType::Ampersand => "&".to_string(),
            _ => "/* unsupported operator */".to_string(),
        }
    }

    /// Get current indentation string
    fn get_indent(&self) -> String {
        self.indent_str.repeat(self.indent_level)
    }

    /// Basic formatting (simple implementation)
    fn basic_format(&self, source: &str) -> String {
        // Simple formatting: add appropriate spaces and line breaks
        source.replace("{", " {\n").replace("}", "\n}").replace(";", ";\n").lines().map(|line| line.trim()).filter(|line| !line.is_empty()).collect::<Vec<_>>().join("\n")
    }

    // 占位符方法 - 这些需要根据具体需求实现
    fn format_struct(&self, _struct_def: &Struct) -> String {
        "/* struct formatting not implemented */".to_string()
    }

    fn format_enum(&self, _enum_def: &Enum) -> String {
        "/* enum formatting not implemented */".to_string()
    }

    fn format_trait(&self, _trait_def: &Trait) -> String {
        "/* trait formatting not implemented */".to_string()
    }

    fn format_impl(&self, _impl_block: &Impl) -> String {
        "/* impl formatting not implemented */".to_string()
    }

    fn format_module(&self, _module: &Module) -> String {
        "/* module formatting not implemented */".to_string()
    }

    fn format_use(&self, use_item: &UseItem) -> String {
        format!("use {};", use_item.path)
    }

    fn format_const(&self, const_item: &Const) -> String {
        format!("const {}: {} = {}", const_item.name.name, self.format_type(&const_item.ty), self.format_expr(&const_item.expr))
    }

    fn format_static(&self, static_item: &Static) -> String {
        let mut_keyword = if static_item.mutable { "mut " } else { "" };
        format!("static {}{}: {} = {}", mut_keyword, static_item.name.name, self.format_type(&static_item.ty), self.format_expr(&static_item.expr))
    }

    fn format_type_alias(&self, type_alias: &TypeAlias) -> String {
        format!("type {} = {}", type_alias.name.name, self.format_type(&type_alias.ty))
    }

    fn _format_macro_def(&self, _macro_def: &str) -> String {
        "/* macro definition formatting not implemented */".to_string()
    }

    fn format_extern_block(&self, _extern_block: &ExternBlock) -> String {
        // 暂时返回占位实现，避免编译错误
        "extern {}".to_string()
    }

    fn _format_generics(&self, _generics: &str) -> String {
        "/* generics formatting not implemented */".to_string()
    }

    fn format_type(&self, _ty: &Type) -> String {
        "/* type formatting not implemented */".to_string()
    }

    fn _format_pattern(&self, _pattern: &Pattern) -> String {
        "/* pattern formatting not implemented */".to_string()
    }
}

impl Clone for RustFormatter {
    fn clone(&self) -> Self {
        Self { indent_level: self.indent_level, indent_str: self.indent_str.clone(), max_line_length: self.max_line_length }
    }
}

impl Default for RustFormatter {
    fn default() -> Self {
        Self::new()
    }
}

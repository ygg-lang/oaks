use oak_core::Parser;
//! Converter from JASM AST to JVM program.

use crate::{formats::jasm::ast::*, program::*};
use oak_core::{OakDiagnostics, OakError};
type Result<T> = std::result::Result<T, OakError>;

/// JASM to JVM conversion.
pub struct JasmToJvmConverter {
    /// Constants.
    constant_pool: JvmConstantPool,
}

impl JasmToJvmConverter {
    /// Creates a new converter.
    pub fn new() -> Self {
        Self { constant_pool: JvmConstantPool::new() }
    }

    /// Converts JASM AST to JVM program.
    pub fn convert(&mut self, ast: JasmRoot) -> OakDiagnostics<JvmProgram> {
        match self.convert_class(ast.class) {
            Ok(program) => OakDiagnostics::success(program),
            Err(error) => OakDiagnostics::failure(error),
        }
    }

    /// Converts class definition.
    fn convert_class(&mut self, class: JasmClass) -> Result<JvmProgram> {
        let mut program = JvmProgram::new(class.name.clone());

        // Sets access flags.
        program.access_flags = JvmAccessFlags::from_modifiers(&class.modifiers);

        // Sets version information.
        if let Some(version_str) = &class.version {
            if let Some((major_str, minor_str)) = version_str.split_once(':') {
                if let (Ok(major), Ok(minor)) = (major_str.parse(), minor_str.parse()) {
                    program.version = JvmVersion { major, minor }
                }
            }
        }

        // Sets source file.
        if let Some(source_file) = class.source_file {
            program.set_source_file(source_file)
        }

        // Converts methods.
        for jasm_method in class.methods {
            let jvm_method = self.convert_method(jasm_method)?;
            program.add_method(jvm_method)
        }

        // Converts fields.
        for jasm_field in class.fields {
            let jvm_field = self.convert_field(jasm_field)?;
            program.add_field(jvm_field)
        }

        // Sets constants.
        program.constant_pool = self.constant_pool.clone();

        Ok(program)
    }

    /// Converts method definition.
    fn convert_method(&mut self, method: JasmMethod) -> Result<JvmMethod> {
        // Parses method name and descriptor.
        let (name, descriptor) = self.parse_method_signature(&method.name_and_descriptor)?;

        let mut jvm_method = JvmMethod::new(name, descriptor);

        // Sets access flags.
        jvm_method.access_flags = JvmAccessFlags::from_modifiers(&method.modifiers);

        // Sets stack size and local variable count.
        jvm_method.max_stack = method.stack_size.unwrap_or(0) as u16;
        jvm_method.max_locals = method.locals_count.unwrap_or(0) as u16;

        // Converts instructions.
        for jasm_instruction in method.instructions {
            let jvm_instruction = self.convert_instruction(jasm_instruction)?;
            jvm_method.add_instruction(jvm_instruction)
        }

        Ok(jvm_method)
    }

    /// Converts field definition.
    fn convert_field(&mut self, field: JasmField) -> Result<JvmField> {
        // Parses field name and descriptor.
        let (name, descriptor) = self.parse_field_signature(&field.name_and_descriptor)?;

        let mut jvm_field = JvmField::new(name, descriptor);

        // Sets access flags.
        jvm_field.access_flags = JvmAccessFlags::from_modifiers(&field.modifiers);

        Ok(jvm_field)
    }

    /// Converts instruction
    fn convert_instruction(&mut self, instruction: JasmInstruction) -> Result<JvmInstruction> {
        match instruction {
            JasmInstruction::Simple(name) => {
                // Creates JvmInstruction variant directly based on instruction name
                match name.as_str() {
                    "nop" => Ok(JvmInstruction::Nop),
                    "aconst_null" => Ok(JvmInstruction::AconstNull),
                    "iconst_m1" => Ok(JvmInstruction::IconstM1),
                    "iconst_0" => Ok(JvmInstruction::Iconst0),
                    "iconst_1" => Ok(JvmInstruction::Iconst1),
                    "iconst_2" => Ok(JvmInstruction::Iconst2),
                    "iconst_3" => Ok(JvmInstruction::Iconst3),
                    "iconst_4" => Ok(JvmInstruction::Iconst4),
                    "iconst_5" => Ok(JvmInstruction::Iconst5),
                    "lconst_0" => Ok(JvmInstruction::Lconst0),
                    "lconst_1" => Ok(JvmInstruction::Lconst1),
                    "fconst_0" => Ok(JvmInstruction::Fconst0),
                    "fconst_1" => Ok(JvmInstruction::Fconst1),
                    "fconst_2" => Ok(JvmInstruction::Fconst2),
                    "dconst_0" => Ok(JvmInstruction::Dconst0),
                    "dconst_1" => Ok(JvmInstruction::Dconst1),
                    "iload_0" => Ok(JvmInstruction::Iload0),
                    "iload_1" => Ok(JvmInstruction::Iload1),
                    "iload_2" => Ok(JvmInstruction::Iload2),
                    "iload_3" => Ok(JvmInstruction::Iload3),
                    "aload_0" => Ok(JvmInstruction::Aload0),
                    "aload_1" => Ok(JvmInstruction::Aload1),
                    "aload_2" => Ok(JvmInstruction::Aload2),
                    "aload_3" => Ok(JvmInstruction::Aload3),
                    "istore_0" => Ok(JvmInstruction::Istore0),
                    "istore_1" => Ok(JvmInstruction::Istore1),
                    "istore_2" => Ok(JvmInstruction::Istore2),
                    "istore_3" => Ok(JvmInstruction::Istore3),
                    "astore_0" => Ok(JvmInstruction::Astore0),
                    "astore_1" => Ok(JvmInstruction::Astore1),
                    "astore_2" => Ok(JvmInstruction::Astore2),
                    "astore_3" => Ok(JvmInstruction::Astore3),
                    "iadd" => Ok(JvmInstruction::Iadd),
                    "isub" => Ok(JvmInstruction::Isub),
                    "imul" => Ok(JvmInstruction::Imul),
                    "idiv" => Ok(JvmInstruction::Idiv),
                    "irem" => Ok(JvmInstruction::Irem),
                    "ineg" => Ok(JvmInstruction::Ineg),
                    "pop" => Ok(JvmInstruction::Pop),
                    "pop2" => Ok(JvmInstruction::Pop2),
                    "dup" => Ok(JvmInstruction::Dup),
                    "swap" => Ok(JvmInstruction::Swap),
                    "ireturn" => Ok(JvmInstruction::Ireturn),
                    "lreturn" => Ok(JvmInstruction::Lreturn),
                    "freturn" => Ok(JvmInstruction::Freturn),
                    "dreturn" => Ok(JvmInstruction::Dreturn),
                    "areturn" => Ok(JvmInstruction::Areturn),
                    "return" => Ok(JvmInstruction::Return),
                    "arraylength" => Ok(JvmInstruction::Arraylength),
                    "athrow" => Ok(JvmInstruction::Athrow),
                    "monitorenter" => Ok(JvmInstruction::Monitorenter),
                    "monitorexit" => Ok(JvmInstruction::Monitorexit),
                    _ => Err(OakError::custom_error(format!("Unknown simple instruction: {}", name))),
                }
            }
            JasmInstruction::WithArgument { instruction, argument } => {
                    // Handles instructions with arguments
                    match instruction.as_str() {
                    "bipush" => {
                        let value = argument
                            .parse::<i8>()
                            .map_err(|_| OakError::custom_error(format!("Invalid bipush value: {}", argument)))?;
                        Ok(JvmInstruction::Bipush { value })
                    }
                    "sipush" => {
                        let value = argument
                            .parse::<i16>()
                            .map_err(|_| OakError::custom_error(format!("Invalid sipush value: {}", argument)))?;
                        Ok(JvmInstruction::Sipush { value })
                    }
                    "iload" => {
                        let index = argument
                            .parse::<u16>()
                            .map_err(|_| OakError::custom_error(format!("Invalid iload index: {}", argument)))?;
                        Ok(JvmInstruction::Iload { index })
                    }
                    "aload" => {
                        let index = argument
                            .parse::<u16>()
                            .map_err(|_| OakError::custom_error(format!("Invalid aload index: {}", argument)))?;
                        Ok(JvmInstruction::Aload { index })
                    }
                    "istore" => {
                        let index = argument
                            .parse::<u16>()
                            .map_err(|_| OakError::custom_error(format!("Invalid istore index: {}", argument)))?;
                        Ok(JvmInstruction::Istore { index })
                    }
                    "astore" => {
                        let index = argument
                            .parse::<u16>()
                            .map_err(|_| OakError::custom_error(format!("Invalid astore index: {}", argument)))?;
                        Ok(JvmInstruction::Astore { index })
                    }
                    "ldc" => {
                        if argument.starts_with("String ") {
                            // String literal
                            let string_literal = &argument[7..]; // Removes "String " prefix
                            let string_value = self.parse_string_literal(string_literal)?;
                            Ok(JvmInstruction::Ldc { symbol: string_value })
                        }
                        else if let Ok(int_value) = argument.parse::<i32>() {
                            Ok(JvmInstruction::Ldc { symbol: int_value.to_string() })
                        }
                        else {
                            Ok(JvmInstruction::Ldc { symbol: argument })
                        }
                    }
                    "new" => Ok(JvmInstruction::New { class_name: argument }),
                    "checkcast" => Ok(JvmInstruction::Checkcast { class_name: argument }),
                    "instanceof" => Ok(JvmInstruction::Instanceof { class_name: argument }),
                    "anewarray" => Ok(JvmInstruction::Anewarray { class_name: argument }),
                    "newarray" => {
                        let atype = argument
                            .parse::<u8>()
                            .map_err(|_| OakError::custom_error(format!("Invalid newarray type: {}", argument)))?;
                        Ok(JvmInstruction::Newarray { atype })
                    }
                    _ => Err(OakError::custom_error(format!("Unknown instruction with argument: {}", instruction))),
                }
            }
            JasmInstruction::MethodCall { instruction, method_ref } => {
                let (class_name, method_name, descriptor) = self.parse_method_reference(&method_ref)?;

                // Adds to constant pool
                self.add_method_reference(&class_name, &method_name, &descriptor);

                match instruction.as_str() {
                    "invokevirtual" => Ok(JvmInstruction::Invokevirtual { class_name, method_name, descriptor }),
                    "invokespecial" => Ok(JvmInstruction::Invokespecial { class_name, method_name, descriptor }),
                    "invokestatic" => Ok(JvmInstruction::Invokestatic { class_name, method_name, descriptor }),
                    "invokeinterface" => Ok(JvmInstruction::Invokeinterface { class_name, method_name, descriptor }),
                    "invokedynamic" => Ok(JvmInstruction::Invokedynamic { class_name, method_name, descriptor }),
                    _ => Err(OakError::custom_error(format!("Unknown method call instruction: {}", instruction))),
                }
            }
            JasmInstruction::FieldAccess { instruction, field_ref } => {
                let (class_name, field_name, descriptor) = self.parse_field_reference(&field_ref)?;

                // Adds to constant pool
                self.add_field_reference(&class_name, &field_name, &descriptor);

                match instruction.as_str() {
                    "getstatic" => Ok(JvmInstruction::Getstatic { class_name, field_name, descriptor }),
                    "putstatic" => Ok(JvmInstruction::Putstatic { class_name, field_name, descriptor }),
                    "getfield" => Ok(JvmInstruction::Getfield { class_name, field_name, descriptor }),
                    "putfield" => Ok(JvmInstruction::Putfield { class_name, field_name, descriptor }),
                    _ => Err(OakError::custom_error(format!("Unknown field access instruction: {}", instruction))),
                }
            }
        }
    }

    /// Parses method signature
    fn parse_method_signature(&self, signature: &str) -> Result<(String, String)> {
        if let Some(colon_pos) = signature.find(':') {
            let name = signature[..colon_pos].to_string();
            let descriptor = signature[colon_pos + 1..].to_string();
            Ok((name, descriptor))
        }
        else {
            Err(OakError::custom_error(format!("Invalid method signature: {}", signature)))
        }
    }

    /// Parses field signature
    fn parse_field_signature(&self, signature: &str) -> Result<(String, String)> {
        if let Some(colon_pos) = signature.find(':') {
            let name = signature[..colon_pos].to_string();
            let descriptor = signature[colon_pos + 1..].to_string();
            Ok((name, descriptor))
        }
        else {
            Err(OakError::custom_error(format!("Invalid field signature: {}", signature)))
        }
    }

    /// Parses method reference
    fn parse_method_reference(&self, reference: &str) -> Result<(String, String, String)> {
        // Format: java/lang/Object."<init>":"()V"
        if let Some(dot_pos) = method_ref.find('.') {
            let class_name = method_ref[..dot_pos].to_string();
            let rest = &method_ref[dot_pos + 1..];

            if let Some(colon_pos) = rest.find(':') {
                let method_name = rest[..colon_pos].trim_matches('"').to_string();
                let descriptor = rest[colon_pos + 1..].trim_matches('"').to_string();
                Ok((class_name, method_name, descriptor))
            }
            else {
                Err(OakError::custom_error(format!("Invalid method reference: {}", method_ref)))
            }
        }
        else {
            Err(OakError::custom_error(format!("Invalid method reference: {}", method_ref)))
        }
    }

    /// Parses field reference
    fn parse_field_reference(&self, field_ref: &str) -> Result<(String, String, String)> {
        // Format: java/lang/System.out:"Ljava/io/PrintStream;"
        if let Some(dot_pos) = field_ref.find('.') {
            let class_name = field_ref[..dot_pos].to_string();
            let rest = &field_ref[dot_pos + 1..];

            if let Some(colon_pos) = rest.find(':') {
                let field_name = rest[..colon_pos].to_string();
                let descriptor = rest[colon_pos + 1..].trim_matches('"').to_string();
                Ok((class_name, field_name, descriptor))
            }
            else {
                Err(OakError::custom_error(format!("Invalid field reference: {}", field_ref)))
            }
        }
        else {
            Err(OakError::custom_error(format!("Invalid field reference: {}", field_ref)))
        }
    }

    /// Parses string literal
    fn parse_string_literal(&self, literal: &str) -> Result<String> {
        if literal.starts_with('"') && literal.ends_with('"') {
            Ok(literal[1..literal.len() - 1].to_string())
        }
        else {
            Ok(literal.to_string())
        }
    }

    /// Adds string constant to constant pool
    fn add_string_constant(&mut self, value: String) -> u16 {
        let utf8_entry = JvmConstantPoolEntry::Utf8 { value: value.clone() };
        let _utf8_index = self.constant_pool.add_entry(utf8_entry);

        let string_entry = JvmConstantPoolEntry::String { value };
        let string_index = self.constant_pool.add_entry(string_entry);

        let symbol = format!("string_{}", string_index);
        self.constant_pool.add_symbol(symbol, string_index);
        string_index
    }

    /// Adds method reference to constant pool
    fn add_method_reference(&mut self, class_name: &str, method_name: &str, descriptor: &str) {
        let class_entry = JvmConstantPoolEntry::Class { name: class_name.to_string() };
        let _class_index = self.constant_pool.add_entry(class_entry);

        let method_entry = JvmConstantPoolEntry::Methodref {
            class_name: class_name.to_string(),
            name: method_name.to_string(),
            descriptor: descriptor.to_string(),
        };
        let method_index = self.constant_pool.add_entry(method_entry);

        let symbol = format!("method_{}_{}", class_name.replace('/', "_"), method_name);
        self.constant_pool.add_symbol(symbol, method_index);
    }

    /// Adds field reference to constant pool
    fn add_field_reference(&mut self, class_name: &str, field_name: &str, descriptor: &str) {
        let class_entry = JvmConstantPoolEntry::Class { name: class_name.to_string() }
        let _class_index = self.constant_pool.add_entry(class_entry);

        let field_entry = JvmConstantPoolEntry::Fieldref {
            class_name: class_name.to_string(),
            name: field_name.to_string(),
            descriptor: descriptor.to_string(),
        }
        let field_index = self.constant_pool.add_entry(field_entry);

        let symbol = format!("field_{}_{}", class_name.replace('/', "_"), field_name);
        self.constant_pool.add_symbol(symbol, field_index)
    }
}

impl Default for JasmToJvmConverter {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function: Converts JASM AST to JVM program.
pub fn convert_jasm_to_jvm(ast: JasmRoot) -> OakDiagnostics<JvmProgram> {
    let mut converter = JasmToJvmConverter::new();
    converter.convert(ast)
}

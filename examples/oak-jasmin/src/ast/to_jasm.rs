use crate::{formats::jasm::ast::*, program::*};
use oak_core::{OakDiagnostics, OakError};
type Result<T> = std::result::Result<T, OakError>;

pub struct JvmToJasmConverter {
    constant_pool: JvmConstantPool,
}

impl JvmToJasmConverter {
    pub fn new() -> Self {
        Self { constant_pool: JvmConstantPool::new() }
    }

    pub fn convert(&mut self, program: JvmProgram) -> OakDiagnostics<JasmRoot> {
        let constant_pool = program.constant_pool.clone();
        self.constant_pool = constant_pool;
        match self.convert_class(program) {
            Ok(jasm_class) => OakDiagnostics::success(JasmRoot { class: jasm_class }),
            Err(error) => OakDiagnostics::failure(error),
        }
    }

    fn convert_class(&mut self, program: JvmProgram) -> Result<JasmClass> {
        let mut jasm_class = JasmClass {
            modifiers: program.access_flags.to_modifiers(),
            name: program.name,
            version: Some(format!("{}:{}", program.version.major, program.version.minor)),
            methods: Vec::new(),
            fields: Vec::new(),
            source_file: program.source_file,
        };

        for jvm_field in program.fields {
            let jasm_field = self.convert_field(jvm_field)?;
            jasm_class.fields.push(jasm_field);
        }

        for jvm_method in program.methods {
            let jasm_method = self.convert_method(jvm_method)?;
            jasm_class.methods.push(jasm_method);
        }

        Ok(jasm_class)
    }

    fn convert_method(&mut self, method: JvmMethod) -> Result<JasmMethod> {
        let modifiers = method.access_flags.to_modifiers();
        let name_and_descriptor = format!("{}:{}", method.name, method.descriptor);
        let stack_size = Some(method.max_stack as u32);
        let locals_count = Some(method.max_locals as u32);

        let mut instructions = Vec::new();
        for jvm_instruction in method.instructions {
            let jasm_instruction = self.convert_instruction(jvm_instruction)?;
            instructions.push(jasm_instruction);
        }

        Ok(JasmMethod { modifiers, name_and_descriptor, stack_size, locals_count, instructions })
    }

    fn convert_field(&mut self, field: JvmField) -> Result<JasmField> {
        let modifiers = field.access_flags.to_modifiers();
        let name_and_descriptor = format!("{}:{}", field.name, field.descriptor);
        Ok(JasmField { modifiers, name_and_descriptor })
    }

    fn convert_instruction(&mut self, instruction: JvmInstruction) -> Result<JasmInstruction> {
        match instruction {
            // 常量加载指令
            JvmInstruction::Nop => Ok(JasmInstruction::Simple("nop".to_string())),
            JvmInstruction::AconstNull => Ok(JasmInstruction::Simple("aconst_null".to_string())),
            JvmInstruction::IconstM1 => Ok(JasmInstruction::Simple("iconst_m1".to_string())),
            JvmInstruction::Iconst0 => Ok(JasmInstruction::Simple("iconst_0".to_string())),
            JvmInstruction::Iconst1 => Ok(JasmInstruction::Simple("iconst_1".to_string())),
            JvmInstruction::Iconst2 => Ok(JasmInstruction::Simple("iconst_2".to_string())),
            JvmInstruction::Iconst3 => Ok(JasmInstruction::Simple("iconst_3".to_string())),
            JvmInstruction::Iconst4 => Ok(JasmInstruction::Simple("iconst_4".to_string())),
            JvmInstruction::Iconst5 => Ok(JasmInstruction::Simple("iconst_5".to_string())),
            JvmInstruction::Lconst0 => Ok(JasmInstruction::Simple("lconst_0".to_string())),
            JvmInstruction::Lconst1 => Ok(JasmInstruction::Simple("lconst_1".to_string())),
            JvmInstruction::Fconst0 => Ok(JasmInstruction::Simple("fconst_0".to_string())),
            JvmInstruction::Fconst1 => Ok(JasmInstruction::Simple("fconst_1".to_string())),
            JvmInstruction::Fconst2 => Ok(JasmInstruction::Simple("fconst_2".to_string())),
            JvmInstruction::Dconst0 => Ok(JasmInstruction::Simple("dconst_0".to_string())),
            JvmInstruction::Dconst1 => Ok(JasmInstruction::Simple("dconst_1".to_string())),
            JvmInstruction::Bipush { value } => {
                Ok(JasmInstruction::WithArgument { instruction: "bipush".to_string(), argument: value.to_string() })
            }
            JvmInstruction::Sipush { value } => {
                Ok(JasmInstruction::WithArgument { instruction: "sipush".to_string(), argument: value.to_string() })
            }
            JvmInstruction::Ldc { symbol } => {
                Ok(JasmInstruction::WithArgument { instruction: "ldc".to_string(), argument: format!("String \"{}\"", symbol) })
            }

            // 局部变量操
            JvmInstruction::Iload0 => Ok(JasmInstruction::Simple("iload_0".to_string())),
            JvmInstruction::Iload1 => Ok(JasmInstruction::Simple("iload_1".to_string())),
            JvmInstruction::Iload2 => Ok(JasmInstruction::Simple("iload_2".to_string())),
            JvmInstruction::Iload3 => Ok(JasmInstruction::Simple("iload_3".to_string())),
            JvmInstruction::Iload { index } => {
                Ok(JasmInstruction::WithArgument { instruction: "iload".to_string(), argument: index.to_string() })
            }
            JvmInstruction::Aload0 => Ok(JasmInstruction::Simple("aload_0".to_string())),
            JvmInstruction::Aload1 => Ok(JasmInstruction::Simple("aload_1".to_string())),
            JvmInstruction::Aload2 => Ok(JasmInstruction::Simple("aload_2".to_string())),
            JvmInstruction::Aload3 => Ok(JasmInstruction::Simple("aload_3".to_string())),
            JvmInstruction::Aload { index } => {
                Ok(JasmInstruction::WithArgument { instruction: "aload".to_string(), argument: index.to_string() })
            }
            JvmInstruction::Istore0 => Ok(JasmInstruction::Simple("istore_0".to_string())),
            JvmInstruction::Istore1 => Ok(JasmInstruction::Simple("istore_1".to_string())),
            JvmInstruction::Istore2 => Ok(JasmInstruction::Simple("istore_2".to_string())),
            JvmInstruction::Istore3 => Ok(JasmInstruction::Simple("istore_3".to_string())),
            JvmInstruction::Istore { index } => {
                Ok(JasmInstruction::WithArgument { instruction: "istore".to_string(), argument: index.to_string() })
            }
            JvmInstruction::Astore0 => Ok(JasmInstruction::Simple("astore_0".to_string())),
            JvmInstruction::Astore1 => Ok(JasmInstruction::Simple("astore_1".to_string())),
            JvmInstruction::Astore2 => Ok(JasmInstruction::Simple("astore_2".to_string())),
            JvmInstruction::Astore3 => Ok(JasmInstruction::Simple("astore_3".to_string())),
            JvmInstruction::Astore { index } => {
                Ok(JasmInstruction::WithArgument { instruction: "astore".to_string(), argument: index.to_string() })
            }

            // 算术运算
            JvmInstruction::Iadd => Ok(JasmInstruction::Simple("iadd".to_string())),
            JvmInstruction::Isub => Ok(JasmInstruction::Simple("isub".to_string())),
            JvmInstruction::Imul => Ok(JasmInstruction::Simple("imul".to_string())),
            JvmInstruction::Idiv => Ok(JasmInstruction::Simple("idiv".to_string())),
            JvmInstruction::Irem => Ok(JasmInstruction::Simple("irem".to_string())),
            JvmInstruction::Ineg => Ok(JasmInstruction::Simple("ineg".to_string())),

            // 栈操
            JvmInstruction::Pop => Ok(JasmInstruction::Simple("pop".to_string())),
            JvmInstruction::Pop2 => Ok(JasmInstruction::Simple("pop2".to_string())),
            JvmInstruction::Dup => Ok(JasmInstruction::Simple("dup".to_string())),
            JvmInstruction::Swap => Ok(JasmInstruction::Simple("swap".to_string())),

            // 返回指令
            JvmInstruction::Ireturn => Ok(JasmInstruction::Simple("ireturn".to_string())),
            JvmInstruction::Lreturn => Ok(JasmInstruction::Simple("lreturn".to_string())),
            JvmInstruction::Freturn => Ok(JasmInstruction::Simple("freturn".to_string())),
            JvmInstruction::Dreturn => Ok(JasmInstruction::Simple("dreturn".to_string())),
            JvmInstruction::Areturn => Ok(JasmInstruction::Simple("areturn".to_string())),
            JvmInstruction::Return => Ok(JasmInstruction::Simple("return".to_string())),

            // 对象操作
            JvmInstruction::New { class_name } => {
                Ok(JasmInstruction::WithArgument { instruction: "new".to_string(), argument: class_name })
            }
            JvmInstruction::Newarray { atype } => {
                Ok(JasmInstruction::WithArgument { instruction: "newarray".to_string(), argument: atype.to_string() })
            }
            JvmInstruction::Anewarray { class_name } => {
                Ok(JasmInstruction::WithArgument { instruction: "anewarray".to_string(), argument: class_name })
            }
            JvmInstruction::Arraylength => Ok(JasmInstruction::Simple("arraylength".to_string())),
            JvmInstruction::Athrow => Ok(JasmInstruction::Simple("athrow".to_string())),
            JvmInstruction::Checkcast { class_name } => {
                Ok(JasmInstruction::WithArgument { instruction: "checkcast".to_string(), argument: class_name })
            }
            JvmInstruction::Instanceof { class_name } => {
                Ok(JasmInstruction::WithArgument { instruction: "instanceof".to_string(), argument: class_name })
            }
            JvmInstruction::Monitorenter => Ok(JasmInstruction::Simple("monitorenter".to_string())),
            JvmInstruction::Monitorexit => Ok(JasmInstruction::Simple("monitorexit".to_string())),

            // 字段访问
            JvmInstruction::Getstatic { class_name, field_name, descriptor } => {
                let field_ref = format!("{}.{}:\"{}\"", class_name, field_name, descriptor);
                Ok(JasmInstruction::FieldAccess { instruction: "getstatic".to_string(), field_ref })
            }
            JvmInstruction::Putstatic { class_name, field_name, descriptor } => {
                let field_ref = format!("{}.{}:\"{}\"", class_name, field_name, descriptor);
                Ok(JasmInstruction::FieldAccess { instruction: "putstatic".to_string(), field_ref })
            }
            JvmInstruction::Getfield { class_name, field_name, descriptor } => {
                let field_ref = format!("{}.{}:\"{}\"", class_name, field_name, descriptor);
                Ok(JasmInstruction::FieldAccess { instruction: "getfield".to_string(), field_ref })
            }
            JvmInstruction::Putfield { class_name, field_name, descriptor } => {
                let field_ref = format!("{}.{}:\"{}\"", class_name, field_name, descriptor);
                Ok(JasmInstruction::FieldAccess { instruction: "putfield".to_string(), field_ref })
            }

            // 方法调用
            JvmInstruction::Invokevirtual { class_name, method_name, descriptor } => {
                let method_ref = format!("{}.\"{}\":\"{}\"", class_name, method_name, descriptor);
                Ok(JasmInstruction::MethodCall { instruction: "invokevirtual".to_string(), method_ref })
            }
            JvmInstruction::Invokespecial { class_name, method_name, descriptor } => {
                let method_ref = format!("{}.\"{}\":\"{}\"", class_name, method_name, descriptor);
                Ok(JasmInstruction::MethodCall { instruction: "invokespecial".to_string(), method_ref })
            }
            JvmInstruction::Invokestatic { class_name, method_name, descriptor } => {
                let method_ref = format!("{}.\"{}\":\"{}\"", class_name, method_name, descriptor);
                Ok(JasmInstruction::MethodCall { instruction: "invokestatic".to_string(), method_ref })
            }
            JvmInstruction::Invokeinterface { class_name, method_name, descriptor } => {
                let method_ref = format!("{}.\"{}\":\"{}\"", class_name, method_name, descriptor);
                Ok(JasmInstruction::MethodCall { instruction: "invokeinterface".to_string(), method_ref })
            }
            JvmInstruction::Invokedynamic { class_name, method_name, descriptor } => {
                let method_ref = format!("{}.\"{}\":\"{}\"", class_name, method_name, descriptor);
                Ok(JasmInstruction::MethodCall { instruction: "invokedynamic".to_string(), method_ref })
            }

            _ => Err(OakError::custom_error(format!("Unsupported JVM instruction for JASM conversion: {:?}", instruction))),
        }
    }

    fn resolve_constant_pool_symbol(&self, symbol: &str) -> Result<String> {
        if let Some(index) = self.constant_pool.symbol_table.get(symbol) {
            if let Some(entry) = self.constant_pool.entries.get(*index as usize) {
                match entry {
                    JvmConstantPoolEntry::Utf8 { value } => Ok(format!("\"{}\"", value)),
                    JvmConstantPoolEntry::Integer { value } => Ok(value.to_string()),
                    JvmConstantPoolEntry::Float { value } => Ok(value.to_string()),
                    JvmConstantPoolEntry::Long { value } => Ok(value.to_string()),
                    JvmConstantPoolEntry::Double { value } => Ok(value.to_string()),
                    JvmConstantPoolEntry::Class { name } => Ok(name.clone()),
                    JvmConstantPoolEntry::String { value } => Ok(format!("String \"{}\"", value)),
                    JvmConstantPoolEntry::Fieldref { class_name, name, descriptor } => {
                        Ok(format!("{}.{}:\"{}\"", class_name, name, descriptor))
                    }
                    JvmConstantPoolEntry::Methodref { class_name, name, descriptor } => {
                        Ok(format!("{}.\"{}\":\"{}\"", class_name, name, descriptor))
                    }
                    JvmConstantPoolEntry::InterfaceMethodref { class_name, name, descriptor } => {
                        Ok(format!("{}.\"{}\":\"{}\"", class_name, name, descriptor))
                    }
                    _ => Err(OakError::custom_error(format!(
                        "Unsupported constant pool entry type for JASM conversion: {:?}",
                        entry
                    ))),
                }
            }
            else {
                Err(OakError::custom_error(format!("Constant pool entry not found for index: {}", index)))
            }
        }
        else {
            Err(OakError::custom_error(format!("Symbol not found in constant pool: {}", symbol)))
        }
    }
}

impl Default for JvmToJasmConverter {
    fn default() -> Self {
        Self::new()
    }
}

pub fn convert_jvm_to_jasm(program: JvmProgram) -> OakDiagnostics<JasmRoot> {
    let mut converter = JvmToJasmConverter::new();
    converter.convert(program)
}

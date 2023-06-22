# Java Abstract Syntax Tree (AST) Module

This module defines the Abstract Syntax Tree (AST) structure for the [Java programming language](https://www.oracle.com/java/). It provides a strongly-typed and comprehensive representation of Java source code, strictly adhering to the Java Language Specification (JLS).

## Purpose

The Java AST is the central data structure used by the compiler, static analysis tools, and IDE features to represent the semantic and structural information of a Java program. It captures the full range of Java constructs, from package declarations to complex expression trees.

## AST Node Types

### Core Structure
- **`JavaRoot`**: The root node representing a complete Java compilation unit.
- **`PackageDeclaration`**: Represents the `package` statement.
- **`ImportDeclaration`**: Represents `import` and `import static` statements.

### Type Declarations
- **`ClassDeclaration`**: Class definition including modifiers, type parameters, superclass, interfaces, and body.
- **`InterfaceDeclaration`**: Interface definition.
- **`EnumDeclaration`**: Enum definition with its constants and members.
- **`RecordDeclaration`**: Java 14+ record definition.
- **`AnnotationDeclaration`**: Annotation type definition.

### Members and Elements
- **`MethodDeclaration`**: Method definition with its signature and body.
- **`FieldDeclaration`**: Field definition with modifiers and initializers.
- **`ConstructorDeclaration`**: Class constructor definition.
- **`EnumConstant`**: Individual constant in an enum.
- **`Annotation`**: Represents an annotation usage (e.g., `@Override`).

### Statements and Expressions
- **`Statement`**: Represents various Java statements (local variables, blocks, control flow, etc.).
- **`Expression`**: The base type for all Java expressions (literals, assignments, calls, lambda expressions, etc.).
- **`Block`**: A sequence of statements enclosed in braces.
- **`TryStatement`**: Try-catch-finally construct, including try-with-resources.

### Types and Parameters
- **`Type`**: Representation of Java types (primitive, reference, array, wildcard, etc.).
- **`Parameter`**: Method or constructor parameter.
- **`TypeParameter`**: Generic type parameter definition.

## Usage Example

```rust
use oak_java::ast::*;

fn main() {
    // Manually constructing a simple AST for a Java class
    let java_ast = JavaRoot {
        package: Some(PackageDeclaration { name: "com.example".to_string() }),
        imports: vec![],
        types: vec![
            TypeDeclaration::Class(ClassDeclaration {
                modifiers: vec![Modifier::Public],
                name: "Main".to_string(),
                members: vec![
                    ClassMember::Method(MethodDeclaration {
                        modifiers: vec![Modifier::Public, Modifier::Static],
                        return_type: Type::Void,
                        name: "main".to_string(),
                        params: vec![
                            Parameter {
                                ty: Type::Array(Box::new(Type::Reference("String".to_string()))),
                                name: "args".to_string(),
                            }
                        ],
                        body: Some(Block {
                            statements: vec![
                                Statement::Expression(Expression::MethodCall(MethodCall {
                                    target: Some(Box::new(Expression::Identifier("System.out".to_string()))),
                                    name: "println".to_string(),
                                    arguments: vec![Expression::StringLiteral("Hello, Java!".to_string())],
                                }))
                            ],
                        }),
                    })
                ],
            })
        ],
    };
}
```

## Design Principles

1. **JLS Fidelity**: Accurately reflects the structural rules of the Java Language Specification.
2. **Type Safety**: Uses Rust's rich type system to ensure AST validity and integrity.
3. **Rich Metadata**: Each node includes span information for precise source mapping.
4. **Extensibility**: Designed to be easily extended with semantic information during compilation phases.

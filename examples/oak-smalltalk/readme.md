# `oak-smalltalk`

This crate provides a comprehensive parser for the Smalltalk language, built using the `oaks` parsing framework. It includes a lexer and language definition to facilitate parsing Smalltalk code with support for modern Smalltalk syntax and features.

## Features

- **Complete Smalltalk Syntax**: Supports all major Smalltalk language constructs
- **Object-Oriented Parsing**: Handles classes, methods, inheritance, and message passing
- **Literal Support**: Numbers, strings, characters, symbols, arrays, booleans, and nil
- **Message Passing**: Unary, binary, and keyword message syntax
- **Block Closures**: Full support for Smalltalk blocks and closures
- **Comments**: Double-quoted comment syntax
- **Collections**: Arrays, ordered collections, sets, and dictionaries
- **Control Structures**: Conditional statements, loops, and exception handling

## Usage

To use the `oak-smalltalk` parser, you typically need to interact with `SmalltalkLanguage` and `SmalltalkLexer`.

### `SmalltalkLanguage`

The `SmalltalkLanguage` struct defines the grammar and rules for Smalltalk. It implements the `Language` trait from the `oaks` framework.

```rust
use oak_smalltalk::SmalltalkLanguage;

let language = SmalltalkLanguage::default();
```

### `SmalltalkLexer`

The `SmalltalkLexer` is responsible for tokenizing the input Smalltalk code based on the `SmalltalkLanguage` definition.

```rust
use oak_smalltalk::{SmalltalkLanguage, SmalltalkLexer};
use oak_core::lexer::{Lexer, LexInput};

// Initialize the language
let language = Box::leak(Box::new(SmalltalkLanguage::default()));

// Create a lexer instance
let lexer = SmalltalkLexer::new(language);

// Prepare the input source code
let source_code = r#"
"Class definition example"
Object subclass: #Person
    instanceVariableNames: 'name age'
    classVariableNames: ''
    poolDictionaries: ''
    category: 'Examples-Classes'!

!Person methodsFor: 'accessing'!

name
    "Return the person's name"
    ^name!

name: aName
    "Set the person's name"
    name := aName!

age
    "Return the person's age"
    ^age!

age: anAge
    "Set the person's age"
    age := anAge!
! !
"#;

let input = LexInput::new(source_code, 0, None);

// Lex the input
let lex_output = lexer.lex(input);

// You can now process the lex_output which contains the tokens
println!("Lexed tokens: {:?}", lex_output.tokens);
```

## Smalltalk Language Features

### Object Definition
```smalltalk
"Class definition"
Object subclass: #MyClass
    instanceVariableNames: 'var1 var2'
    classVariableNames: 'ClassVar'
    poolDictionaries: ''
    category: 'MyCategory'!
```

### Method Definition
```smalltalk
!MyClass methodsFor: 'category'!

methodName: parameter
    "Method comment"
    | localVar1 localVar2 |
    localVar1 := parameter.
    ^localVar1 * 2!
! !
```

### Literals
```smalltalk
"Numbers"
42          "integer"
3.14159     "float"
1.23e-4     "scientific notation"
16rFF       "hexadecimal"
8r755       "octal"

"Strings and Characters"
'Hello World'    "string"
$a               "character"

"Symbols"
#selector        "symbol"
#'multi word'   "symbol with spaces"

"Arrays"
#(1 2 3 4 5)                    "literal array"
#('hello' 'world' #symbol)      "mixed array"
```

### Message Passing
```smalltalk
"Unary messages"
object method
array size
dictionary keys

"Binary messages"
1 + 2
3 * 4
5 < 10

"Keyword messages"
dictionary at: 'key' put: 'value'
array copyFrom: 1 to: 5
string replaceFrom: 1 to: 3 with: 'new'
```

### Blocks and Closures
```smalltalk
"Simple block"
[42] value

"Block with parameter"
[:x | x * x] value: 5

"Block with multiple parameters"
[:x :y | x + y] value: 3 value: 4

"Block with local variables"
[
    | temp |
    temp := 10.
    temp * 2
] value
```

### Control Structures
```smalltalk
"Conditionals"
x > 0 ifTrue: ['positive'] ifFalse: ['non-positive']

"Loops"
1 to: 10 do: [:i | Transcript show: i printString; cr]

"Collection iteration"
#(1 2 3 4 5) do: [:each | Transcript show: each printString; cr]
#(1 2 3 4 5) select: [:each | each even]
#(1 2 3 4 5) collect: [:each | each * 2]
```

## Supported Syntax Elements

### Tokens
- **Literals**: Numbers, strings, characters, symbols, arrays, booleans, nil
- **Identifiers**: Variable names, method names, class names
- **Operators**: Arithmetic, comparison, assignment
- **Delimiters**: Parentheses, brackets, braces, dots, semicolons
- **Comments**: Double-quoted text
- **Whitespace**: Spaces, tabs, newlines

### Language Constructs
- Class definitions and inheritance
- Method definitions with categories
- Instance and class variables
- Message passing (unary, binary, keyword)
- Block closures with parameters
- Control structures (conditionals, loops)
- Exception handling
- Collection operations

## Integration with Oaks Framework

This parser integrates seamlessly with the Oaks framework, providing:

- **Incremental Parsing**: Support for partial and streaming parsing
- **Error Recovery**: Robust error handling with precise location information
- **Syntax Highlighting**: Compatible with `oak-highlight` for multi-language syntax highlighting
- **AST Generation**: Can be extended to generate typed AST structures
- **Pretty Printing**: Compatible with `oak-pretty-print` for code formatting

## Examples

For comprehensive examples of Smalltalk syntax and usage patterns, see the test files in the `tests/` directory, particularly `tests/lexer/basic.st` which contains extensive examples of Smalltalk language features.

## Development Status

The Smalltalk parser is actively developed and supports core Smalltalk syntax. Future enhancements may include:

- Complete AST implementation
- Advanced error recovery
- Semantic analysis
- Integration with Smalltalk runtime systems
- Support for dialect-specific features (Pharo, Squeak, GNU Smalltalk)

This example demonstrates the power and flexibility of the Oaks framework for parsing complex object-oriented languages like Smalltalk.
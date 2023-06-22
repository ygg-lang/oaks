# Valkyrie Code Formatter

The Valkyrie Code Formatter provides functionality to format and pretty-print Valkyrie source code according to established style guidelines and conventions.

## Overview

The formatter is designed to:
- Maintain consistent code style across Valkyrie projects
- Improve code readability and maintainability
- Support automatic formatting of Valkyrie language constructs
- Provide configurable formatting options

## Key Components

### ValkyrieFormatter

The main formatter struct that handles the formatting operations:

- **format()** - Formats complete Valkyrie source code
- **format_namespace()** - Formats namespace declarations
- **format_function()** - Formats function declarations
- **format_micro()** - Formats micro definitions
- **indent_lines()** - Utility for proper indentation

## Supported Valkyrie Features

The formatter supports formatting for:

1. **Namespace Declarations**
   - Proper indentation of namespace bodies
   - Consistent brace placement

2. **Function Declarations**
   - Parameter list formatting
   - Function body indentation
   - Return type formatting

3. **Micro Definitions**
   - Expression formatting
   - Proper spacing around operators

4. **Statements and Expressions**
   - Let statements
   - Expression statements
   - Block expressions

## Usage

```rust
use oak_valkyrie::formatter::ValkyrieFormatter;

let formatter = ValkyrieFormatter::new();
let source = "namespace Test{fn main(){let x=42;}}";
let formatted = formatter.format(source);
```

## Future Enhancements

- Configuration options for different formatting styles
- Integration with AST for semantic-aware formatting
- Support for comments and documentation formatting
- Customizable indentation and spacing rules
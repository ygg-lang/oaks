# Dejavu Code Formatter

The Dejavu Code Formatter provides functionality to format and pretty-print Dejavu source code according to established style guidelines and conventions.

## Overview

The formatter is designed to:
- Maintain consistent code style across Dejavu projects
- Improve code readability and maintainability
- Support automatic formatting of Dejavu language constructs
- Provide configurable formatting options

## Key Components

### DejavuFormatter

The main formatter struct that handles the formatting operations:

- **format()** - Formats complete Dejavu source code
- **format_namespace()** - Formats namespace declarations
- **format_micro()** - Formats micro function declarations
- **indent_lines()** - Utility for proper indentation

## Supported Dejavu Features

The formatter supports formatting for:

1. **Namespace Declarations**
   - Proper indentation of namespace bodies
   - Consistent brace placement

2. **Micro Function Declarations**
   - Parameter list formatting
   - Function body indentation
   - Return type formatting

3. **Statements and Expressions**
   - Let statements
   - Expression statements
   - Block expressions

## Usage

```rust
use oak_dejavu::formatter::DejavuFormatter;

let formatter = DejavuFormatter::new();
let source = "namespace Test{micro main(){let x=42;}}";
let formatted = formatter.format(source);
```

## Future Enhancements

- Configuration options for different formatting styles
- Integration with AST for semantic-aware formatting
- Support for comments and documentation formatting
- Customizable indentation and spacing rules

# Oak TeX Parser

[![Crates.io](https://img.shields.io/crates/v/oak-tex.svg)](https://crates.io/crates/oak-tex)
[![Documentation](https://docs.rs/oak-tex/badge.svg)](https://docs.rs/oak-tex)

A high-performance TeX/LaTeX parser for Rust, built with the Oak parser combinator framework. Parse TeX documents with comprehensive AST generation and error handling.

## Overview

Oak TeX provides robust parsing capabilities for TeX and LaTeX documents, supporting commands, environments, math mode, and all major TeX constructs. Built on the Oak parser combinator framework, it delivers excellent performance and detailed error messages.

## Features

- ✅ **Complete TeX Support**: Parse commands, environments, math mode, and macros
- ✅ **LaTeX Compatibility**: Support for LaTeX document structure and packages
- ✅ **Modern Rust API**: Type-safe parsing with comprehensive error handling
- ✅ **High Performance**: Built on the efficient Oak parser combinator framework
- ✅ **Rich AST**: Detailed Abstract Syntax Tree with source location tracking
- ✅ **Extensible**: Easy to extend for custom TeX dialects
- ✅ **Well Tested**: Comprehensive test suite with real-world examples

## Quick Start

Add Oak TeX to your `Cargo.toml`:

```toml
[dependencies]
oak = "0.1.0"
oak-tex = "0.1.0"
```

## Parsing Examples

### Basic LaTeX Document Parsing

```rust
use oak::{Parser, Language};
use oak_tex::TeXLanguage;

fn main() {
    let source = r#"
        \documentclass{article}
        \usepackage{amsmath}
        \usepackage{graphicx}
        
        \title{Introduction to Mathematics}
        \author{Jane Doe}
        \date{\today}
        
        \begin{document}
        
        \maketitle
        
        \section{Quadratic Equations}
        
        The quadratic equation $ax^2 + bx + c = 0$ has solutions given by:
        
        \begin{equation}
            x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}
        \end{equation}
        
        \section{Examples}
        
        Consider the equation $x^2 - 5x + 6 = 0$. Here, $a = 1$, $b = -5$, and $c = 6$.
        
        \begin{align}
            x &= \frac{-(-5) \pm \sqrt{(-5)^2 - 4(1)(6)}}{2(1)} \\
            &= \frac{5 \pm \sqrt{25 - 24}}{2} \\
            &= \frac{5 \pm \sqrt{1}}{2} \\
            &= \frac{5 \pm 1}{2}
        \end{align}
        
        Therefore, the solutions are $x = 3$ and $x = 2$.
        
        \end{document}
    "#;
    
    let mut parser = Parser::<TeXLanguage>::new();
    match parser.parse(&source) {
        Ok(ast) => {
            println!("Parsed AST: {:#?}", ast);
        }
        Err(error) => {
            eprintln!("Parse error: {}", error);
        }
    }
}
```

### Advanced Document with Custom Commands

```rust
use oak::{Parser, Language};
use oak_tex::TeXLanguage;

fn main() {
    let source = r#"
        \documentclass[12pt,a4paper]{article}
        \usepackage[utf8]{inputenc}
        \usepackage[T1]{fontenc}
        \usepackage{amsmath,amsfonts,amssymb}
        \usepackage{graphicx}
        \usepackage{hyperref}
        \usepackage{tikz}
        
        \newcommand{\R}{\mathbb{R}}
        \newcommand{\N}{\mathbb{N}}
        \newcommand{\Z}{\mathbb{Z}}
        \newcommand{\Q}{\mathbb{Q}}
        \newcommand{\C}{\mathbb{C}}
        
        \newtheorem{theorem}{Theorem}[section]
        \newtheorem{lemma}[theorem]{Lemma}
        \newtheorem{corollary}[theorem]{Corollary}
        \newtheorem{definition}[theorem]{Definition}
        
        \title{Advanced Mathematical Concepts}
        \author{Dr. John Smith}
        \date{\today}
        
        \begin{document}
        
        \maketitle
        
        \tableofcontents
        
        \section{Introduction}
        
        This document presents fundamental concepts in mathematical analysis.
        
        \section{Real Numbers}
        
        \begin{definition}
        The set of real numbers, denoted by \R, is the complete ordered field.
        \end{definition}
        
        \begin{theorem}[Completeness of \R]
        Every non-empty subset of \R that is bounded above has a least upper bound.
        \end{theorem}
        
        \section{Complex Numbers}
        
        \begin{definition}
        The set of complex numbers is defined as \C = \{a + bi : a, b \in \R\} where $i^2 = -1$.
        \end{definition}
        
        \begin{theorem}
        The field \C is algebraically closed.
        \end{theorem}
        
        \end{document}
    "#;
    
    let mut parser = Parser::<TeXLanguage>::new();
    match parser.parse(&source) {
        Ok(ast) => {
            println!("Advanced document parsed successfully!");
        }
        Err(error) => {
            eprintln!("Parse error: {}", error);
        }
    }
}
```

## Advanced Features

### Math Mode Parsing

Oak TeX supports parsing complex mathematical expressions:

```rust
let source = r#"
    Inline math: $E = mc^2$ and $a^2 + b^2 = c^2$
    
    Display math:
    \[
        \int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}
    \]
    
    Equation environment:
    \begin{equation}
        \nabla \times \mathbf{E} = -\frac{\partial \mathbf{B}}{\partial t}
    \end{equation}
    
    Align environment:
    \begin{align}
        a_1 &= b_1 + c_1 \\
        a_2 &= b_2 + c_2 + d_2 \\
        a_3 &= b_3 + c_3 + d_3 + e_3
    \end{align}
"#;
```

### Tables and Arrays

Parse tables and arrays:

```rust
let source = r#"
    \begin{tabular}{|l|c|r|}
        \hline
        Name & Age & City \\
        \hline
        Alice & 25 & New York \\
        Bob & 30 & London \\
        Carol & 28 & Paris \\
        \hline
    \end{tabular}
    
    \[
        \begin{pmatrix}
            1 & 2 & 3 \\
            4 & 5 & 6 \\
            7 & 8 & 9
        \end{pmatrix}
        \begin{bmatrix}
            a & b \\
            c & d
        \end{bmatrix}
    \]
"#;
```

### Packages and Extensions

Handle package imports and custom commands:

```rust
let source = r#"
    \documentclass{beamer}
    \usepackage{tikz}
    \usetikzlibrary{shapes,arrows}
    \usepackage{algorithm}
    \usepackage{algorithmic}
    
    \newcommand{\BigO}[1]{\ensuremath{\mathcal{O}(#1)}}
    \DeclareMathOperator{\erf}{erf}
    
    \begin{document}
    
    \begin{frame}{Algorithm Complexity}
        \begin{algorithm}[H]
        \caption{Bubble Sort}
        \begin{algorithmic}[1]
        \FOR{$i = 1$ to $n-1$}
            \FOR{$j = 1$ to $n-i$}
                \IF{$A[j] > A[j+1]$}
                    \STATE swap $A[j]$ and $A[j+1]$
                \ENDIF
            \ENDFOR
        \ENDFOR
        \end{algorithmic}
        \end{algorithm}
        \end{frame}
    
    \end{document}
"#;
```

## AST Structure

The parser generates a rich AST with the following main node types:

- `TeXFile` - Root node containing the entire document
- `DocumentClass` - Document class declaration
- `Package` - Package imports
- `Command` - TeX commands like \section{}
- `Environment` - Environments like \begin{equation}...\end{equation}
- `MathMode` - Mathematical expressions in $...$ or \[...\]
- `Text` - Regular text content
- `Comment` - TeX comments starting with %
- `Group` - Braced groups { ... }

## Performance

Oak TeX is designed for high performance:

- **Zero-copy parsing** where possible
- **Streaming support** for large documents
- **Efficient memory usage** with minimal allocations
- **Fast error recovery** for better developer experience

## Integration

Oak TeX integrates seamlessly with the Oak ecosystem:

```rust
use oak::{Parser, Language};
use oak_tex::TeXLanguage;

// Use with other Oak parsers
let mut parser = Parser::<TeXLanguage>::new();
let result = parser.parse(tex_source);
```

## Examples

More examples can be found in the [examples directory](https://github.com/axodotdev/oak/tree/main/examples/oak-tex/examples):

- [Basic LaTeX document](examples/basic.rs)
- [Mathematical expressions](examples/math.rs)
- [Tables and arrays](examples/tables.rs)
- [Custom commands](examples/macros.rs)
- [Error handling](examples/error_handling.rs)

## Contributing

We welcome contributions! Please see our [Contributing Guide](https://github.com/axodotdev/oak/blob/main/CONTRIBUTING.md) for details.
// Comprehensive Typst Lexer Test

#set page(
    paper: "a4",
    margin: (x: 2cm, y: 2cm),
    numbering: "1",
    header: [
        #set text(8pt)
        #smallcaps("Typst Documentation")
        #h(1fr)
        Page #counter(page).display()
    ]
)

#set text(
    font: "New Computer Modern",
    size: 11pt,
    lang: "en"
)

#let title = "Typst Document"

= Introduction
This is a comprehensive test of Typst syntax.
We can use *bold*, _italic_, and `monospaced` text.
Escaped symbols: \# \$ \%.

== Math Mode
Inline math: $E = mc^2$.
Display math:
$
    sum_(i=0)^n i = (n(n+1)) / 2
$

Complex Math:
$
    cal(A) := { x in RR | x "is natural" }
    mat(1, 2; 3, 4)
    vec(1, 2)
    cases(
        1 "if" x > 0,
        0 "otherwise"
    )
    integral_0^oo e^(-x) dx = 1
$

== Lists and Enumerations
- Unordered item 1
- Unordered item 2
    - Nested item
        - Deeply nested

+ Ordered item 1
+ Ordered item 2
    + Nested ordered

/ Term 1: Definition 1
/ Term 2: Definition 2 (long definition that
  spans multiple lines)

== Functions and Scripting
#let add(x, y) = {
    x + y
}

The sum of 1 and 2 is #add(1, 2).

#let values = (1, 2, 3, 4, 5)
#for v in values [
    - Value: #v
]

#let dict = (name: "Typst", version: 0.1)
#dict.name is version #dict.version

#if values.len() > 0 [
    The list is not empty.
] else [
    The list is empty.
]

#show heading: it => [
    #set text(fill: blue)
    #block(below: 1em)[
        #it
    ]
]

== Figures and Images
#figure(
    image("test.png", width: 80%),
    caption: [A test image],
)

== References
See @intro for more details.
See @math-section.

== Code Blocks
```rust
fn main() {
    println!("Hello from Rust code block!");
}
```

```python
def hello():
    print("Hello from Python")
```

== Comments
// Single line comment
/*
    Multi-line comment
    spanning lines
    /* Nested comment */
*/

== Labels
<intro>
<math-section>

== String Operations
#let name = "World"
#("Hello " + name)

== Raw Text
`raw text`
```
block raw text
```

== Shapes
#rect(width: 100%, height: 20pt, fill: blue)
#circle(radius: 10pt)
#line(length: 100%)

== Imports
#import "utils.typ": *
#include "chapter1.typ"

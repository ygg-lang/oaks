// Typst Test Document - Comprehensive Syntax Coverage
// This document tests various Typst syntax elements for lexer testing

#set document(title: "Typst Test Document", author: "Test Author")
#set page(numbering: "1", number-align: center)
#set text(font: "Linux Libertine", size: 11pt)
#set heading(numbering: "1.1")

// Title and metadata
#align(center)[
  #text(size: 20pt, weight: "bold")[Typst Comprehensive Test Document]
  
  #v(0.5em)
  
  #text(size: 14pt)[Testing Various Syntax Elements]
  
  #v(1em)
  
  #text(size: 12pt)[Author: Test Author]
  
  #text(size: 12pt)[Date: #datetime.today().display()]
]

#pagebreak()

// Table of contents
#outline()

#pagebreak()

= Introduction

This document serves as a comprehensive test for Typst syntax elements, covering everything from basic text formatting to advanced features like functions, loops, and mathematical expressions.

== Basic Text Formatting

Here we test various text formatting options:

- *Bold text* using asterisks
- _Italic text_ using underscores  
- `Monospace text` using backticks
- ~Strikethrough text~ using tildes
- #underline[Underlined text] using function
- #overline[Overlined text] using function
- #smallcaps[Small caps text] using function
- #super[Superscript] and #sub[subscript] text

=== Text Styling with Functions

#text(fill: red)[Red text]
#text(fill: blue, weight: "bold")[Bold blue text]
#text(size: 14pt, style: "italic")[Large italic text]
#text(font: "Courier New")[Monospace font]

== Headings and Structure

= Level 1 Heading
== Level 2 Heading  
=== Level 3 Heading
==== Level 4 Heading
===== Level 5 Heading
====== Level 6 Heading

== Lists

=== Unordered Lists

- First item
- Second item
  - Nested item
  - Another nested item
    - Deeply nested item
- Third item

=== Ordered Lists

1. First numbered item
2. Second numbered item
   1. Nested numbered item
   2. Another nested item
3. Third numbered item

=== Custom Lists

#list(
  [Custom list item 1],
  [Custom list item 2],
  [Custom list item 3]
)

#enum(
  [Enumerated item 1],
  [Enumerated item 2], 
  [Enumerated item 3]
)

== Links and References

Visit #link("https://typst.app")[Typst's website] for more information.

Internal reference to @introduction.

#label("introduction")

== Code Blocks

=== Inline Code

Use the `print()` function to output text.

=== Code Blocks

```python
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)

# Calculate first 10 Fibonacci numbers
for i in range(10):
    print(f"F({i}) = {fibonacci(i)}")
```

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);
}
```

```javascript
const greet = (name) => {
    return `Hello, ${name}!`;
};

console.log(greet("World"));
```

== Mathematics

=== Inline Math

The quadratic formula is $x = (-b ± sqrt(b^2 - 4a c)) / (2a)$.

The area of a circle is $A = π r^2$.

=== Display Math

$ sum_(i=1)^n i = (n(n+1))/2 $

$ integral_(-∞)^∞ e^(-x^2) dif x = sqrt(π) $

$ lim_(x → ∞) (1 + 1/x)^x = e $

=== Complex Mathematical Expressions

$ mat(
  a, b;
  c, d
) vec(x, y) = vec(a x + b y, c x + d y) $

$ cases(
  x + y = 5,
  2x - y = 1
) $

$ sum_(k=0)^∞ x^k/k! = e^x $

$ product_(i=1)^n a_i = a_1 · a_2 · ... · a_n $

== Tables

=== Basic Table

#table(
  columns: 3,
  [Name], [Age], [City],
  [Alice], [25], [New York],
  [Bob], [30], [London],
  [Charlie], [35], [Tokyo]
)

=== Styled Table

#table(
  columns: (1fr, auto, auto),
  stroke: 0.5pt,
  fill: (x, y) => if calc.odd(y) { gray.lighten(80%) },
  [*Product*], [*Price*], [*Stock*],
  [Laptop], [$999], [15],
  [Mouse], [$25], [50],
  [Keyboard], [$75], [30],
  [Monitor], [$299], [8]
)

== Figures and Images

#figure(
  rect(width: 100pt, height: 60pt, fill: blue.lighten(80%)),
  caption: [A simple blue rectangle]
) <fig-rect>

As shown in @fig-rect, we can create simple shapes.

== Variables and Functions

#let name = "Typst"
#let version = "0.11"
#let pi = 3.14159

The current version of #name is #version.

The value of π is approximately #pi.

=== Custom Functions

#let greet(name) = [Hello, #name!]

#greet("World")
#greet("Typst")

#let double(x) = x * 2
#let square(x) = x * x

Double of 5 is #double(5).
Square of 7 is #square(7).

=== Functions with Multiple Parameters

#let format-name(first, last, title: none) = {
  if title != none {
    [#title #first #last]
  } else {
    [#first #last]
  }
}

#format-name("John", "Doe")
#format-name("Jane", "Smith", title: "Dr.")

== Conditionals

#let score = 85

#if score >= 90 [
  Excellent work!
] else if score >= 80 [
  Good job!
] else if score >= 70 [
  Not bad.
] else [
  Needs improvement.
]

== Loops

=== For Loops

#for i in range(5) [
  Item #(i + 1): #lorem(5)
  
]

#for (index, item) in ("a", "b", "c").enumerate() [
  #index: #item
  
]

=== While Loops

#let count = 0
#while count < 3 [
  Count: #count
  
  #(count += 1)
]

== Arrays and Dictionaries

=== Arrays

#let fruits = ("apple", "banana", "cherry")
#let numbers = (1, 2, 3, 4, 5)

First fruit: #fruits.at(0)
Last number: #numbers.at(-1)
Array length: #fruits.len()

=== Dictionaries

#let person = (
  name: "Alice",
  age: 30,
  city: "New York"
)

Name: #person.name
Age: #person.age
City: #person.city

== String Operations

#let text = "Hello, World!"

Original: #text
Uppercase: #upper(text)
Lowercase: #lower(text)
Length: #text.len()

== Layout and Spacing

=== Columns

#columns(2)[
  This text will be displayed in two columns. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
  
  #colbreak()
  
  This is the second column. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
]

=== Grid Layout

#grid(
  columns: (1fr, 1fr),
  gutter: 1em,
  [Left column content],
  [Right column content],
  [Bottom left],
  [Bottom right]
)

=== Alignment

#align(center)[Centered text]

#align(right)[Right-aligned text]

#align(left)[Left-aligned text]

== Boxes and Shapes

#rect(width: 100pt, height: 50pt, fill: red.lighten(80%), stroke: 1pt)

#circle(radius: 25pt, fill: green.lighten(80%))

#ellipse(width: 80pt, height: 40pt, fill: blue.lighten(80%))

#polygon(
  fill: yellow.lighten(80%),
  stroke: 1pt,
  (0pt, 0pt),
  (30pt, 0pt),
  (15pt, 25pt)
)

== Page Layout

#set page(margin: (x: 2cm, y: 2.5cm))

=== Page Breaks

Content before page break.

#pagebreak()

Content after page break.

== Bibliography and Citations

// Note: This would typically reference an external bibliography file
// #bibliography("references.bib")

== Advanced Features

=== Custom Styling

#let highlight(body) = rect(
  fill: yellow.lighten(80%),
  inset: 5pt,
  radius: 3pt,
  body
)

#highlight[This text is highlighted with a custom function.]

=== State Management

#let counter = state("my-counter", 0)

#counter.update(5)
Current counter value: #counter.display()

#counter.update(n => n + 3)
Updated counter value: #counter.display()

=== Measurements and Units

Width: #measure([Hello World]).width
Height: #measure([Hello World]).height

== Error Handling

#let safe-divide(a, b) = {
  if b == 0 {
    [Error: Division by zero]
  } else {
    str(a / b)
  }
}

10 ÷ 2 = #safe-divide(10, 2)
10 ÷ 0 = #safe-divide(10, 0)

== Comments

// This is a single-line comment

/* This is a
   multi-line comment
   that spans several lines */

== Raw Text and Escaping

Raw text: `#let x = 5`

Escaped characters: \# \* \_ \` \~ \\

== Conclusion

This document demonstrates the comprehensive syntax coverage of Typst, including:

- Text formatting and styling
- Mathematical expressions
- Code blocks and syntax highlighting
- Tables and figures
- Variables and functions
- Control flow (conditionals and loops)
- Data structures (arrays and dictionaries)
- Layout and spacing
- Custom styling and functions
- State management
- And much more!

Typst provides a powerful and flexible typesetting system that combines the ease of markup languages with the power of programming languages.

#align(center)[
  #text(style: "italic")[End of Document]
]
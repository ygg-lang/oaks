⍝ Comprehensive APL Lexer Test
⍝ Comments start with a lamp symbol

⍝ --- Scalar Arithmetic ---
2 + 3
5 - 2
3 × 4
10 ÷ 2
2 * 3      ⍝ Exponentiation
3 ⌈ 4      ⍝ Maximum
3 ⌊ 4      ⍝ Minimum
| ¯5       ⍝ Absolute value (magnitude)
! 5        ⍝ Factorial

⍝ --- Arrays (Vectors) ---
1 2 3 4 5
2.5 3.14 ¯1.5
'Hello' 'World'

⍝ --- Array Operations ---
⍳ 10       ⍝ Iota (Index generator)
⍴ 3 3      ⍝ Reshape (3x3 matrix)
, 3 3 ⍴ ⍳9 ⍝ Ravel (flatten)
⌽ 1 2 3    ⍝ Reverse
⊖ 3 3 ⍴ ⍳9 ⍝ Reverse first axis
+⌿ 3 3 ⍴ ⍳9 ⍝ Sum reduction along first axis
+/ 1 2 3   ⍝ Sum reduction
×/ 1 2 3   ⍝ Product reduction

⍝ --- Operators (Adverbs/Conjunctions) ---
+.×        ⍝ Inner product (Matrix multiplication)
∘.×        ⍝ Outer product
/          ⍝ Compression / Replicate
\          ⍝ Expansion / Scan
¨          ⍝ Each
⍣          ⍝ Power operator (Iterate)
.          ⍝ Dot product / Inner product
∘          ⍝ Jot (Outer product / Compose)

⍝ --- Logic & Comparison ---
1 = 1
1 ≠ 2
2 < 3
3 > 2
2 ≤ 3
3 ≥ 2
1 ∧ 1      ⍝ And
1 ∨ 0      ⍝ Or
~ 1        ⍝ Not
1 ⍲ 0      ⍝ Nand
1 ⍱ 0      ⍝ Nor

⍝ --- Mathematical Functions ---
○ 1        ⍝ Pi times
1 ○ 0.5    ⍝ Circular functions (Sin, Cos, etc.)
⍟ 10       ⍝ Natural Log
10 ⍟ 100   ⍝ Log base 10
√ 9        ⍝ Square root (generic power)

⍝ --- Structural Functions ---
⍴          ⍝ Shape
≢          ⍝ Tally (Count)
⍋          ⍝ Grade up (Indices for sorting)
⍒          ⍝ Grade down
⍉          ⍝ Transpose
↑          ⍝ Take
↓          ⍝ Drop
⊂          ⍝ Enclose
⊃          ⍝ Disclose / First
∊          ⍝ Membership
⍷          ⍝ Find
∪          ⍝ Unique
∩          ⍝ Intersection
~          ⍝ Without / Not

⍝ --- Variables & Assignment ---
X ← 10
Y ← 20
Z ← X + Y
Vector ← 1 2 3 4 5
Matrix ← 3 3 ⍴ ⍳9

⍝ --- User Defined Functions (Dfns) ---
Mean ← { (+/⍵) ÷ ≢⍵ }
Mean 1 2 3 4 5

Hypotenuse ← { √ (⍺*2) + (⍵*2) }
3 Hypotenuse 4

⍝ --- Tradfns (Traditional Functions) ---
∇ R ← Add A B
  R ← A + B
∇

⍝ --- Special Symbols ---
⍬          ⍝ Zilde (Empty numeric vector)
⍺          ⍝ Left argument (in dfns)
⍵          ⍝ Right argument (in dfns)
⍞          ⍝ Character input/output
⎕          ⍝ Evaluated input/output
⍙          ⍝ Delta (often used in names)
∆          ⍝ Delta
⍇          ⍝ File read (quad-variant)
⍈          ⍝ File write
⍐          ⍝ Format

⍝ --- Complex Numbers ---
1J2        ⍝ 1 + 2i
3.5J¯1.2   ⍝ 3.5 - 1.2i

⍝ --- System Variables ---
⎕IO ← 0    ⍝ Index Origin
⎕PP ← 10   ⍝ Print Precision
⎕WA        ⍝ Workspace Available

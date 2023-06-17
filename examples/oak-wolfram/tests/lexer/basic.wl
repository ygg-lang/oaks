(* Wolfram Language test file *)
f[x_] := x^2 + 2*x + 1

result = f[5]

If[result > 30, 
  Print["Result is large: ", result],
  Print["Result is small: ", result]
]

list = {1, 2, 3, 4, 5}
squared = Map[#^2 &, list]

Plot[Sin[x], {x, 0, 2*Pi}]
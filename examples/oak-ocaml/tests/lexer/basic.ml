(* Basic OCaml test file *)

let rec factorial n =
  if n <= 1 then 1
  else n * factorial (n - 1)

let main () =
  let result = factorial 5 in
  Printf.printf "Factorial of 5 is %d\n" result

type 'a tree =
  | Leaf of 'a
  | Node of 'a tree * 'a * 'a tree

let rec insert x = function
  | Leaf y -> if x <= y then Node (Leaf x, y, Leaf y) else Node (Leaf y, y, Leaf x)
  | Node (l, v, r) -> if x <= v then Node (insert x l, v, r) else Node (l, v, insert x r)

module StringSet = Set.Make(String)

class counter =
  object
    val mutable count = 0
    method get = count
    method incr = count <- count + 1
  end
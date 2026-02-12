(* Comprehensive OCaml Lexer Test *)

(* Basic Types and Literals *)
let int_val = 42
let float_val = 3.14
let string_val = "Hello, world!"
let char_val = 'a'
let bool_val = true

(* Lists and Arrays *)
let list = [1; 2; 3; 4]
let array = [| 1; 2; 3; 4 |]

(* Functions *)
let rec factorial n =
    if n = 0 then 1 else n * factorial (n - 1)

let add x y = x + y
let result = add 5 10

(* Pattern Matching *)
let describe_list l =
    match l with
    | [] -> "Empty"
    | [x] -> "Singleton"
    | x :: xs -> "Multiple elements"

(* Records *)
type person = {
    name : string;
    age : int;
    mutable active : bool;
}

let alice = { name = "Alice"; age = 30; active = true }
let name = alice.name
let () = alice.active <- false

(* Variants and Algebraic Data Types *)
type shape =
    | Circle of float
    | Rectangle of float * float
    | Point

let area s =
    match s with
    | Circle r -> 3.14159 *. r *. r
    | Rectangle (w, h) -> w *. h
    | Point -> 0.0

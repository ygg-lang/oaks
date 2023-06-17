// F# test file for lexer testing
// Functional programming language for .NET

// Basic values and bindings
let x = 42
let name = "F#"
let pi = 3.14159
let isValid = true
let letter = 'A'

// Type annotations
let integer : int = 100
let floating : float = 3.14
let text : string = "Hello World"
let boolean : bool = false
let character : char = 'Z'

// Lists
let numbers = [1; 2; 3; 4; 5]
let emptyList = []
let mixedList = [1; 2; 3; "four"; "five"]
let nestedList = [[1; 2]; [3; 4]; [5; 6]]

// List operations
let firstItem = List.head numbers
let restOfList = List.tail numbers
let doubled = List.map (fun x -> x * 2) numbers
let evens = List.filter (fun x -> x % 2 = 0) numbers
let sum = List.sum numbers
let max = List.max numbers

// Arrays
let arr = [|1; 2; 3; 4; 5|]
let matrix = [| [|1; 2|]; [|3; 4|] |]
let mutableArray = [|1; 2; 3|]

// Sequences
let seq1 = seq { 1 .. 10 }
let seq2 = seq { for i in 1 .. 10 do yield i * i }
let infiniteSeq = Seq.initInfinite (fun i -> i * 2)

// Tuples
let pair = (1, "one")
let triple = (1, 2, 3)
let deconstructed = let (a, b) = pair in a + b

// Records
type Person = {
    Name: string
    Age: int
    Email: string option
}

let person1 = { Name = "Alice"; Age = 30; Email = Some "alice@example.com" }
let person2 = { person1 with Age = 31 }

// Discriminated unions
type Shape =
    | Circle of radius: float
    | Rectangle of width: float * height: float
    | Triangle of base: float * height: float

let circle = Circle 5.0
let rectangle = Rectangle(10.0, 20.0)
let triangle = Triangle(6.0, 8.0)

// Pattern matching
let describeShape shape =
    match shape with
    | Circle r -> sprintf "Circle with radius %.2f" r
    | Rectangle(w, h) -> sprintf "Rectangle %.2f x %.2f" w h
    | Triangle(b, h) -> sprintf "Triangle base %.2f height %.2f" b h

// Functions
let add x y = x + y
let multiply x y = x * y
let square x = x * x
let isEven x = x % 2 = 0

// Recursive functions
let rec factorial n =
    if n <= 1 then 1
    else n * factorial (n - 1)

let rec fibonacci n =
    match n with
    | 0 | 1 -> n
    | _ -> fibonacci (n - 1) + fibonacci (n - 2)

// Higher-order functions
let applyTwice f x = f (f x)
let compose f g x = f (g x)
let pipeline x f = f x

// Pipe operator
let result = 5 |> square |> add 10 |> multiply 2

// Function composition
let squarePlusOne = square >> add 1

// Option type
let safeDivide x y =
    if y = 0 then None
    else Some (x / y)

let processOption opt =
    match opt with
    | Some value -> printfn "Value: %d" value
    | None -> printfn "No value"

// Result type
type Result<'T, 'E> =
    | Ok of 'T
    | Error of 'E

let divide x y =
    if y = 0 then Error "Division by zero"
    else Ok (x / y)

// Async workflows
let asyncWorkflow = async {
    printfn "Starting async work"
    do! Async.Sleep 1000
    printfn "Async work completed"
    return 42
}

// Computation expressions
type MaybeBuilder() =
    member _.Bind(opt, binder) = Option.bind binder opt
    member _.Return(value) = Some value

let maybe = MaybeBuilder()

let maybeWorkflow = maybe {
    let! x = Some 10
    let! y = Some 20
    return x + y
}

// Units of measure
[<Measure>] type meter
[<Measure>] type second
[<Measure>] type kg

let distance = 100.0<meter>
let time = 10.0<second>
let speed = distance / time  // 10.0<meter/second>

// Object expressions
let obj =
    { new System.IDisposable with
        member _.Dispose() = printfn "Disposed" }

// Type extensions
type System.String with
    member this.IsPalindrome() =
        let reversed = System.String(this.ToCharArray() |> Array.rev)
        this = reversed

// Active patterns
let (|Even|Odd|) n = if n % 2 = 0 then Even else Odd

let describeNumber n =
    match n with
    | Even -> sprintf "%d is even" n
    | Odd -> sprintf "%d is odd" n

// Quotations
let expr = <@ 1 + 2 * 3 @>
let lambdaExpr = <@ fun x -> x + 1 @>

// Attributes
[<System.Obsolete("Use newFunction instead")>]
let oldFunction x = x + 1

[<System.Serializable>]
type SerializableType = { Value: int }

// Modules and namespaces
module MathUtils =
    let private helper x = x * 2
    let publicFunction x = helper x + 1
    
    module Nested =
        let nestedFunction x = x + 100

// Exception handling
try
    failwith "Something went wrong"
with
| :? System.Exception as ex -> printfn "Error: %s" ex.Message

// Mutable variables and reference cells
let mutable counter = 0
counter <- counter + 1

let refCell = ref 10
refCell := !refCell + 5

// Events
let event = Event<int>()
event.Publish.Add(fun x -> printfn "Event triggered with %d" x)
event.Trigger(42)

// Lazy evaluation
let lazyValue = lazy (printfn "Computing expensive value"; 42)
let actualValue = lazyValue.Value  // Computation happens here

// Print formatting
printfn "Integer: %d, Float: %.2f, String: %s" 42 3.14 "hello"
printfn "List: %A" numbers
printfn "Record: %A" person1

// Type providers (simplified example)
type CsvProvider = CsvProvider<"data.csv">
let data = CsvProvider.Load("data.csv")

// Computation expressions with custom builders
type LoggingBuilder() =
    member _.Bind(x, f) = 
        printfn "Binding: %A" x
        f x
    member _.Return(x) = 
        printfn "Returning: %A" x
        x

let logging = LoggingBuilder()

let loggedWorkflow = logging {
    let! x = 10
    let! y = 20
    return x + y
}
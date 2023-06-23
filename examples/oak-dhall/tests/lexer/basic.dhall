-- Dhall test file for lexer testing
-- This file contains various Dhall syntax elements

-- Basic function definition
let identity : ∀(a : Type) → a → a = λ(a : Type) → λ(x : a) → x

-- Natural numbers and arithmetic
let add : Natural → Natural → Natural = λ(x : Natural) → λ(y : Natural) → x + y
let multiply : Natural → Natural → Natural = λ(x : Natural) → λ(y : Natural) → x * y

-- Text manipulation
let greet : Text → Text = λ(name : Text) → "Hello, ${name}!"

-- Lists
let numbers : List Natural = [1, 2, 3, 4, 5]
let words : List Text = ["hello", "world", "dhall"]
let mixed : List (Natural, Text) = [{fst = 1, snd = "one"}, {fst = 2, snd = "two"}]

-- Records
let person : { name : Text, age : Natural, email : Text } = {
  name = "Alice",
  age = 30,
  email = "alice↯example.com"
}

-- Record operations
let updatedPerson = person // { age = 31 }
let name : Text = person.name

-- Union types
let status : < Active : {} | Inactive : Natural | Pending : Text > = < Active = {=} | Active : {} | Inactive : Natural | Pending : Text >

-- Optional values
let maybeNumber : Optional Natural = Some 42
let noNumber : Optional Natural = None Natural

-- Function composition
let compose : ∀(a : Type) → ∀(b : Type) → ∀(c : Type) → (b → c) → (a → b) → a → c =
  λ(a : Type) → λ(b : Type) → λ(c : Type) → λ(f : b → c) → λ(g : a → b) → λ(x : a) → f (g x)

-- Boolean operations
let and : Bool → Bool → Bool = λ(x : Bool) → λ(y : Bool) → x && y
let or : Bool → Bool → Bool = λ(x : Bool) → λ(y : Bool) → x || y
let not : Bool → Bool = λ(x : Bool) → !(x)

-- Conditional expressions
let max : Natural → Natural → Natural = λ(x : Natural) → λ(y : Natural) →
  if Natural/isZero (x - y) then y else x

-- Type annotations
let annotatedValue : Text = "This has a type annotation"
let complexType : { field1 : Natural, field2 : List Text } = {
  field1 = 42,
  field2 = ["a", "b", "c"]
}

-- Imports (commented out for testing)
-- let types = ./types.dhall
-- let functions = https://example.com/functions.dhall sha256:1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef

-- Assertions
let _ : Bool = assert : Natural/even 2 ≡ True
let _ : Bool = assert : List/length Natural [1, 2, 3] ≡ 3

-- Let expressions with multiple bindings
let x = 5
let y = 10
let sum = x + y
let product = x * y

-- Nested records
let company : {
  name : Text,
  employees : List { name : Text, role : Text },
  address : { street : Text, city : Text, zipCode : Natural }
} = {
  name = "Tech Corp",
  employees = [
    { name = "Bob", role = "Developer" },
    { name = "Alice", role = "Designer" }
  ],
  address = {
    street = "123 Main St",
    city = "San Francisco",
    zipCode = 94105
  }
}

-- Recursive merge
let defaults = { debug = False, port = 8080 }
let config = defaults // { port = 3000 }

-- Function types with multiple parameters
let multiParam : Natural → Text → Bool → { num : Natural, text : Text, flag : Bool } =
  λ(n : Natural) → λ(t : Text) → λ(b : Bool) → { num = n, text = t, flag = b }

-- List operations
let doubled : List Natural = List/map Natural Natural (λ(x : Natural) → x * 2) [1, 2, 3]
let filtered : List Natural = List/filter Natural (λ(x : Natural) → Natural/greaterThan x 1) [1, 2, 3, 4]

-- Text interpolation
let template : Text → Natural → Text = λ(name : Text) → λ(age : Natural) →
  "Hello ${name}, you are ${Natural/show age} years old!"

-- Double-quoted strings with escape sequences
let escaped : Text = "This string has \"quotes\" and \n newlines"
let unicode : Text = "Unicode: \u03BB x \u2192 x + 1"

-- Multi-line strings
let multiline : Text = ''
  This is a multi-line string
  that preserves indentation
  and line breaks
  ''

-- Environment variables (commented out)
-- let home = env:HOME as Text
-- let path = env:PATH as Text

-- Complex type signatures
let complexFunction : ∀(a : Type) → ∀(b : Type) → (a → b) → List a → List b =
  λ(a : Type) → λ(b : Type) → λ(f : a → b) → λ(list : List a) → List/map a b f list

-- Using merge with types
let userPreferences : { theme : Text, language : Text } = { theme = "dark", language = "en" }
let defaultSettings : { theme : Text, language : Text, notifications : Bool } =
  userPreferences // { notifications = True }

-- Empty record and empty list
let emptyRecord : {} = {=}
let emptyList : List Text = [] : List Text

-- Natural number literals and operations
let largeNumber : Natural = 1000000
let hexNumber : Natural = 0xFF
let binaryNumber : Natural = 0b1010

-- Type-level computation (advanced)
let List/concatMap : ∀(a : Type) → ∀(b : Type) → (a → List b) → List a → List b =
  λ(a : Type) → λ(b : Type) → λ(f : a → List b) → λ(list : List a) →
    List/fold (List b) (List b) [] (λ(acc : List b) → λ(elem : a) → acc # f elem) list
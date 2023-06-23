# Nim Test File - Comprehensive Syntax Coverage
# This file tests various Nim syntax elements for lexer testing

import std/[strutils, sequtils, tables, sets, json, os, times, math, random, 
           algorithm, sugar, options, asyncdispatch, httpclient, strformat]

# Constants and compile-time evaluation
const
  PI = 3.14159265359
  MAX_SIZE = 1000
  VERSION = "1.0.0"
  DEBUG = true

# Type definitions
type
  # Enum types
  Color = enum
    Red, Green, Blue, Yellow, Purple, Orange
  
  Direction = enum
    North = "N"
    South = "S" 
    East = "E"
    West = "W"
  
  LogLevel = enum
    Trace = 0
    Debug = 1
    Info = 2
    Warn = 3
    Error = 4
    Fatal = 5
  
  # Object types
  Point = object
    x, y: float
  
  Point3D = object
    x, y, z: float
  
  Person = object
    name: string
    age: int
    email: string
    active: bool
  
  Employee = object of Person
    id: int
    department: string
    salary: float
    manager: ref Employee
  
  # Generic types
  Stack[T] = object
    items: seq[T]
    top: int
  
  Result[T, E] = object
    case success: bool
    of true:
      value: T
    of false:
      error: E
  
  # Variant types (sum types)
  Shape = object
    case kind: ShapeKind
    of Circle:
      radius: float
    of Rectangle:
      width, height: float
    of Triangle:
      a, b, c: float
  
  ShapeKind = enum
    Circle, Rectangle, Triangle
  
  # Reference types
  Node = ref object
    data: int
    next: Node
  
  Tree = ref object
    value: int
    left, right: Tree
  
  # Distinct types
  UserId = distinct int
  Email = distinct string
  Password = distinct string

# Global variables
var
  globalCounter: int = 0
  userDatabase: Table[UserId, Person]
  activeConnections: HashSet[string]
  configData: JsonNode

# Let bindings (immutable)
let
  appName = "Nim Test Application"
  startTime = now()
  defaultConfig = %*{
    "host": "localhost",
    "port": 8080,
    "debug": true
  }

# Procedures (functions)
proc add(a, b: int): int =
  ## Adds two integers
  result = a + b

proc subtract(a, b: int): int =
  a - b

proc multiply(a, b: int): int {.inline.} =
  a * b

proc divide(a, b: float): float =
  if b == 0.0:
    raise newException(DivByZeroDefect, "Division by zero")
  a / b

# Generic procedures
proc swap[T](a, b: var T) =
  let temp = a
  a = b
  b = temp

proc find[T](items: seq[T], predicate: proc(item: T): bool): Option[T] =
  for item in items:
    if predicate(item):
      return some(item)
  none(T)

proc map[T, U](items: seq[T], transform: proc(item: T): U): seq[U] =
  result = newSeq[U](items.len)
  for i, item in items:
    result[i] = transform(item)

# Procedures with various parameter types
proc processData(data: string; separator: char = ','; 
                 ignoreEmpty: bool = true): seq[string] =
  result = data.split(separator)
  if ignoreEmpty:
    result = result.filterIt(it.len > 0)

proc variadicSum(numbers: varargs[int]): int =
  for num in numbers:
    result += num

proc openArrayExample(arr: openArray[int]): int =
  for item in arr:
    result += item

# Method definitions
method area(shape: Shape): float {.base.} =
  case shape.kind
  of Circle:
    PI * shape.radius * shape.radius
  of Rectangle:
    shape.width * shape.height
  of Triangle:
    # Heron's formula
    let s = (shape.a + shape.b + shape.c) / 2
    sqrt(s * (s - shape.a) * (s - shape.b) * (s - shape.c))

method perimeter(shape: Shape): float {.base.} =
  case shape.kind
  of Circle:
    2 * PI * shape.radius
  of Rectangle:
    2 * (shape.width + shape.height)
  of Triangle:
    shape.a + shape.b + shape.c

# Operator overloading
proc `+`(a, b: Point): Point =
  Point(x: a.x + b.x, y: a.y + b.y)

proc `-`(a, b: Point): Point =
  Point(x: a.x - b.x, y: a.y - b.y)

proc `*`(p: Point, scalar: float): Point =
  Point(x: p.x * scalar, y: p.y * scalar)

proc `$`(p: Point): string =
  fmt"Point(x: {p.x}, y: {p.y})"

# Stack implementation
proc newStack[T](): Stack[T] =
  Stack[T](items: ↯[], top: -1)

proc push[T](stack: var Stack[T], item: T) =
  stack.items.add(item)
  inc stack.top

proc pop[T](stack: var Stack[T]): T =
  if stack.top < 0:
    raise newException(IndexDefect, "Stack is empty")
  result = stack.items[stack.top]
  dec stack.top
  stack.items.setLen(stack.top + 1)

proc isEmpty[T](stack: Stack[T]): bool =
  stack.top < 0

proc peek[T](stack: Stack[T]): T =
  if stack.top < 0:
    raise newException(IndexDefect, "Stack is empty")
  stack.items[stack.top]

# Tree operations
proc newTree(value: int): Tree =
  Tree(value: value, left: nil, right: nil)

proc insert(tree: var Tree, value: int) =
  if tree == nil:
    tree = newTree(value)
  elif value < tree.value:
    insert(tree.left, value)
  else:
    insert(tree.right, value)

proc search(tree: Tree, value: int): bool =
  if tree == nil:
    false
  elif value == tree.value:
    true
  elif value < tree.value:
    search(tree.left, value)
  else:
    search(tree.right, value)

proc inorderTraversal(tree: Tree): seq[int] =
  if tree != nil:
    result.add(inorderTraversal(tree.left))
    result.add(tree.value)
    result.add(inorderTraversal(tree.right))

# Iterators
iterator countdown(start, stop: int): int =
  var current = start
  while current >= stop:
    yield current
    dec current

iterator pairs[T](items: seq[T]): tuple[index: int, value: T] =
  for i in 0..<items.len:
    yield (i, items[i])

iterator fibonacci(max: int): int =
  var a, b = 0, 1
  while a <= max:
    yield a
    let temp = a + b
    a = b
    b = temp

# Templates
template benchmark(name: string, code: untyped): untyped =
  let start = cpuTime()
  code
  let duration = cpuTime() - start
  echo fmt"Benchmark '{name}': {duration:.6f} seconds"

template withFile(filename: string, mode: FileMode, body: untyped): untyped =
  let file = open(filename, mode)
  try:
    body
  finally:
    file.close()

template debug(msg: string): untyped =
  when DEBUG:
    echo fmt"[DEBUG] {msg}"

# Macros
import macros

macro createGetter(typeName: untyped, fieldName: untyped): untyped =
  let procName = ident("get" & $fieldName)
  result = quote do:
    proc `procName`(obj: `typeName`): auto =
      obj.`fieldName`

macro createSetter(typeName: untyped, fieldName: untyped): untyped =
  let procName = ident("set" & $fieldName)
  result = quote do:
    proc `procName`(obj: var `typeName`, value: auto) =
      obj.`fieldName` = value

# Apply macros
createGetter(Person, name)
createSetter(Person, name)

# Async procedures
proc fetchData(url: string): Future[string] {.async.} =
  let client = newAsyncHttpClient()
  try:
    let response = await client.get(url)
    result = await response.body
  finally:
    client.close()

proc processAsync(data: seq[string]): Future[seq[string]] {.async.} =
  result = newSeq[string]()
  for item in data:
    await sleepAsync(10) # Simulate async work
    result.add(item.toUpper())

# Exception handling
proc safeDivide(a, b: float): Result[float, string] =
  try:
    if b == 0.0:
      Result[float, string](success: false, error: "Division by zero")
    else:
      Result[float, string](success: true, value: a / b)
  except:
    Result[float, string](success: false, error: getCurrentExceptionMsg())

proc readFileContent(filename: string): string =
  try:
    result = readFile(filename)
  except IOError:
    echo fmt"Error reading file: {filename}"
    result = ""
  except:
    echo fmt"Unexpected error: {getCurrentExceptionMsg()}"
    result = ""

# String operations and formatting
proc stringOperations() =
  let text = "Hello, Nim World!"
  
  echo fmt"Original: {text}"
  echo fmt"Length: {text.len}"
  echo fmt"Uppercase: {text.toUpper()}"
  echo fmt"Lowercase: {text.toLower()}"
  echo fmt"Reversed: {text.reversed()}"
  echo fmt"Contains 'Nim': {text.contains("Nim")}"
  echo fmt"Starts with 'Hello': {text.startsWith("Hello")}"
  echo fmt"Ends with '!': {text.endsWith("!")}"
  
  let words = text.split(' ')
  echo fmt"Words: {words}"
  echo fmt"Joined: {words.join(" | ")}"
  
  # String interpolation
  let name = "Alice"
  let age = 30
  echo fmt"Name: {name}, Age: {age}"
  echo &"Name: {name}, Age: {age}"

# Collection operations
proc collectionOperations() =
  # Sequences
  var numbers = ↯[1, 2, 3, 4, 5]
  numbers.add(6)
  numbers.insert(0, 0)
  echo fmt"Numbers: {numbers}"
  
  let evens = numbers.filterIt(it mod 2 == 0)
  echo fmt"Even numbers: {evens}"
  
  let doubled = numbers.mapIt(it * 2)
  echo fmt"Doubled: {doubled}"
  
  let sum = numbers.foldl(a + b)
  echo fmt"Sum: {sum}"
  
  # Arrays
  var matrix: array[3, array[3, int]]
  for i in 0..2:
    for j in 0..2:
      matrix[i][j] = i * 3 + j
  
  # Tables (hash maps)
  var scores = initTable[string, int]()
  scores["Alice"] = 95
  scores["Bob"] = 87
  scores["Charlie"] = 92
  
  for name, score in scores:
    echo fmt"{name}: {score}"
  
  # Sets
  var fruits = initHashSet[string]()
  fruits.incl("apple")
  fruits.incl("banana")
  fruits.incl("orange")
  
  echo fmt"Fruits: {fruits}"
  echo fmt"Has apple: {"apple" in fruits}"

# Pattern matching with case statements
proc processValue(value: int): string =
  case value
  of 0:
    "zero"
  of 1..10:
    "small"
  of 11..100:
    "medium"
  of 101..1000:
    "large"
  else:
    "very large"

proc processShape(shape: Shape): string =
  case shape.kind
  of Circle:
    fmt"Circle with radius {shape.radius}"
  of Rectangle:
    fmt"Rectangle {shape.width}x{shape.height}"
  of Triangle:
    fmt"Triangle with sides {shape.a}, {shape.b}, {shape.c}"

# Control flow
proc controlFlowExamples() =
  # If statements
  let x = 42
  if x > 0:
    echo "Positive"
  elif x < 0:
    echo "Negative"
  else:
    echo "Zero"
  
  # When statements (compile-time if)
  when DEBUG:
    echo "Debug mode enabled"
  else:
    echo "Release mode"
  
  # For loops
  for i in 1..5:
    echo fmt"Count: {i}"
  
  for i in countdown(5, 1):
    echo fmt"Countdown: {i}"
  
  for item in ↯["a", "b", "c"]:
    echo fmt"Item: {item}"
  
  for i, item in ↯["x", "y", "z"]:
    echo fmt"Index {i}: {item}"
  
  # While loops
  var count = 0
  while count < 3:
    echo fmt"While count: {count}"
    inc count
  
  # Block statements with labels
  block outer:
    for i in 1..3:
      for j in 1..3:
        if i * j > 4:
          break outer
        echo fmt"i: {i}, j: {j}"

# Object-oriented features
proc objectOrientedExamples() =
  # Create objects
  var person = Person(name: "John Doe", age: 35, email: "john↯example.com", active: true)
  echo fmt"Person: {person.name}, Age: {person.age}"
  
  var employee = Employee(
    name: "Jane Smith",
    age: 28,
    email: "jane↯company.com",
    active: true,
    id: 1001,
    department: "Engineering",
    salary: 75000.0
  )
  
  # Method calls
  let circle = Shape(kind: Circle, radius: 5.0)
  let rectangle = Shape(kind: Rectangle, width: 4.0, height: 6.0)
  
  echo fmt"Circle area: {circle.area()}"
  echo fmt"Rectangle area: {rectangle.area()}"
  echo fmt"Circle perimeter: {circle.perimeter()}"
  echo fmt"Rectangle perimeter: {rectangle.perimeter()}"

# Functional programming features
proc functionalExamples() =
  let numbers = ↯[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
  
  # Higher-order functions
  let evens = numbers.filter(proc(x: int): bool = x mod 2 == 0)
  let squares = numbers.map(proc(x: int): int = x * x)
  let sum = numbers.foldl(a + b)
  
  echo fmt"Evens: {evens}"
  echo fmt"Squares: {squares}"
  echo fmt"Sum: {sum}"
  
  # Lambda expressions (sugar syntax)
  let doubled = numbers.map(x => x * 2)
  let filtered = numbers.filter(x => x > 5)
  
  echo fmt"Doubled: {doubled}"
  echo fmt"Filtered: {filtered}"
  
  # Partial application
  proc add(a, b: int): int = a + b
  let addFive = (x: int) => add(5, x)
  echo fmt"Add five to 3: {addFive(3)}"

# Memory management examples
proc memoryManagement() =
  # Automatic memory management
  var data = newSeq[int](1000)
  for i in 0..<data.len:
    data[i] = i
  
  # Manual memory management (when needed)
  let ptr = alloc(sizeof(int) * 100)
  defer: dealloc(ptr)
  
  # Reference counting
  var node1 = Node(data: 1, next: nil)
  var node2 = Node(data: 2, next: node1)
  var node3 = Node(data: 3, next: node2)

# Concurrency examples
proc concurrencyExamples() =
  # Async/await
  proc asyncExample() {.async.} =
    echo "Starting async operation"
    await sleepAsync(1000)
    echo "Async operation completed"
  
  # Spawn (parallel execution)
  proc computeSum(start, stop: int): int =
    for i in start..stop:
      result += i
  
  let future1 = spawn computeSum(1, 1000)
  let future2 = spawn computeSum(1001, 2000)
  
  let result1 = ^future1
  let result2 = ^future2
  let total = result1 + result2
  
  echo fmt"Parallel computation result: {total}"

# File I/O operations
proc fileOperations() =
  let filename = "test_output.txt"
  
  # Write to file
  withFile(filename, fmWrite):
    file.writeLine("Hello, Nim!")
    file.writeLine("This is a test file.")
    file.writeLine(fmt"Generated at: {now()}")
  
  # Read from file
  if fileExists(filename):
    let content = readFile(filename)
    echo fmt"File content:\n{content}"
    
    # Clean up
    removeFile(filename)

# JSON operations
proc jsonOperations() =
  # Create JSON
  let data = %*{
    "name": "Nim Language",
    "version": "1.6.0",
    "features": ["fast", "expressive", "elegant"],
    "stats": {
      "lines_of_code": 100000,
      "contributors": 500
    }
  }
  
  echo fmt"JSON: {data.pretty()}"
  
  # Parse JSON
  let jsonStr = """{"x": 10, "y": 20}"""
  let parsed = parseJson(jsonStr)
  echo fmt"Parsed x: {parsed["x"].getInt()}"
  echo fmt"Parsed y: {parsed["y"].getInt()}"

# Main execution
proc main() =
  echo fmt"=== Nim Comprehensive Test ==="
  echo fmt"Application: {appName}"
  echo fmt"Started at: {startTime}"
  
  # Test various features
  stringOperations()
  collectionOperations()
  objectOrientedExamples()
  functionalExamples()
  
  # Test mathematical operations
  benchmark "Math operations":
    var result = 0.0
    for i in 1..10000:
      result += sin(float(i)) * cos(float(i))
    echo fmt"Math result: {result}"
  
  # Test data structures
  var stack = newStack[int]()
  for i in 1..5:
    stack.push(i)
  
  echo "Stack contents:"
  while not stack.isEmpty():
    echo fmt"  {stack.pop()}"
  
  # Test tree
  var tree: Tree = nil
  for value in [5, 3, 7, 1, 9, 4, 6]:
    tree.insert(value)
  
  echo fmt"Tree traversal: {tree.inorderTraversal()}"
  echo fmt"Search for 4: {tree.search(4)}"
  echo fmt"Search for 8: {tree.search(8)}"
  
  # Test iterators
  echo "Fibonacci sequence:"
  for fib in fibonacci(100):
    echo fmt"  {fib}"
  
  # Test file operations
  fileOperations()
  
  # Test JSON operations
  jsonOperations()
  
  echo "=== Test completed ==="

# Run the main procedure
when isMainModule:
  main()
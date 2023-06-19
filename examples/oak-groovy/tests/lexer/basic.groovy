// Comprehensive Groovy syntax test file for lexer testing

// 1. Basic Types and Variables
def intValue = 123
def floatValue = 123.45f
def doubleValue = 123.45d
def longValue = 123L
def booleanValue = true
def charValue = 'c'
def stringValue = "Hello, Groovy!"
def multiLineString = '''This is a
multi-line string.'''
def gString = "The value is ${intValue}"
def nullValue = null

// 2. Operators
def a = 10, b = 5
def sum = a + b
def difference = a - b
def product = a * b
def division = a / b
def remainder = a % b
def power = a ** b
def increment = a++
def decrement = --b
def equal = (a == b)
def notEqual = (a != b)
def greaterThan = (a > b)
def lessThan = (a < b)
def greaterThanOrEqual = (a >= b)
def lessThanOrEqual = (a <= b)
def logicalAnd = (true && false)
def logicalOr = (true || false)
def logicalNot = !true
def bitwiseAnd = (a & b)
def bitwiseOr = (a | b)
def bitwiseXor = (a ^ b)
def leftShift = (a << 1)
def rightShift = (a >> 1)
def unsignedRightShift = (a >>> 1)
def spaceship = (a <=> b) // Groovy 3+
def elvis = (nullValue ?: "default")
def safeNavigation = nullValue?.length()
def methodPointer = this.&greet
def spreadOperator = [1, 2, *[3, 4]]
def regexMatch = "abc" =~ /a/
def regexFind = "abc" ==~ /a/

// 3. Control Structures
// If-else
if (a > b) {
    println "a is greater"
} else if (a < b) {
    println "b is greater"
} else {
    println "a and b are equal"
}

// Switch
def day = "Monday"
switch (day) {
    case "Monday":
        println "Start of week"
        break
    case "Friday":
        println "End of week"
        break
    default:
        println "Mid-week"
}

// For loop
for (i in 1..3) {
    println "Loop $i"
}

// While loop
def count = 0
while (count < 3) {
    println "While loop $count"
    count++
}

// Do-while loop
def doCount = 0
do {
    println "Do-while loop $doCount"
    doCount++
} while (doCount < 3)

// For-each with closure
[1, 2, 3].each { item ->
    println "Item: $item"
}

// 4. Collections
def list = [1, 2, 3, "four", true]
def map = [key1: "value1", key2: 123]
def set = [1, 2, 2, 3] as Set
def range = 1..5

println "List: $list"
println "Map: $map"
println "Set: $set"
println "Range: $range"

// 5. Classes and Objects
class MyClass {
    String name
    int id
    static final String TYPE = "Test"

    MyClass(String name, int id) {
        this.name = name
        this.id = id
    }

    String getInfo() {
        return "Name: $name, ID: $id, Type: ${TYPE}"
    }

    void doSomething() {
        println "Doing something..."
    }
}

def myObject = new MyClass("TestObject", 1)
println myObject.getInfo()
myObject.doSomething()

// 6. Closures
def closure = { param1, param2 ->
    println "Closure executed with $param1 and $param2"
    param1 + param2
}
println "Closure result: ${closure(10, 20)}"

def implicitItClosure = { it * 2 }
println "Implicit it closure: ${implicitItClosure(5)}"

// 7. Metaprogramming (simple example)
MyClass.metaClass.newMethod = { -> "New method called!" }
println myObject.newMethod()

// 8. Exception Handling
try {
    def result = 10 / 0
} catch (ArithmeticException e) {
    println "Caught exception: ${e.message}"
} finally {
    println "Finally block executed"
}

// 9. Annotations
@Grab('org.apache.commons:commons-lang3:3.12.0')
import org.apache.commons.lang3.StringUtils

// 10. GStrings (already covered, but more examples)
def item = "apple"
def price = 1.50
println "The ${item} costs \$${price}."

// 11. Regular Expressions
def email = "test@example.com"
def emailPattern = ~/\\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,4}\\b/
if (email =~ emailPattern) {
    println "Valid email"
} else {
    println "Invalid email"
}

// 12. Elvis Operator (more complex)
def config = [:]
def value = config.setting ?: "default_setting"
println "Setting value: $value"

// 13. Safe Navigation Operator (more complex)
def user = [address: [street: "Main St"]]
def streetName = user?.address?.street
println "Street name: $streetName"

def invalidUser = [:]
def invalidStreetName = invalidUser?.address?.street
println "Invalid street name: $invalidStreetName"

// 14. Spread Operator (more complex)
def list1 = [1, 2]
def list2 = [3, 4]
def combinedList = [*list1, *list2, 5]
println "Combined list: $combinedList"

// 15. Method Pointers
class Calculator {
    def add(x, y) { x + y }
}
def calc = new Calculator()
def addMethod = calc.&add
println "Method pointer result: ${addMethod(5, 3)}"

// 16. Traits (Groovy 2.3+)
trait Greeter {
    String greeting() { "Hello from trait!" }
}

class TraitUser implements Greeter {}

def traitUser = new TraitUser()
println traitUser.greeting()

// 17. Enums
enum Status {
    ACTIVE, INACTIVE, PENDING
}
println "Status: ${Status.ACTIVE}"

// 18. Scripting features (outside of class)
def scriptVar = "I'm a script variable"
println scriptVar

// 19. Closures as method arguments
def processList(list, processor) {
    list.collect(processor)
}
def processed = processList([1, 2, 3], { it * 10 })
println "Processed list: $processed"

// 20. Named arguments
def printDetails(Map details) {
    println "Details: Name=${details.name}, Age=${details.age}"
}
printDetails(name: "Alice", age: 30)

// 21. Currying
def multiply = { a, b -> a * b }
def multiplyByTwo = multiply.curry(2)
println "Curried multiply: ${multiplyByTwo(5)}"

// 22. Memoization
def fib
fib = { n ->
    if (n < 2) return n
    fib(n - 1) + fib(n - 2)
}.memoize()
println "Fib(10): ${fib(10)}"

// 23. Elvis assignment operator (Groovy 2.5+)
def configValue = null
configValue ?= "default"
println "Config value: $configValue"

// 24. Identity operator (is)
def obj1 = new Object()
def obj2 = obj1
println "Identity check: ${obj1.is(obj2)}"

// 25. Type checking (instanceof)
def checkType = "hello"
if (checkType instanceof String) {
    println "It's a String"
}

// 26. Power assertion (Groovy 1.6+)
// assert 1 == 2 // This would fail and show detailed info

// 27. Spread-dot operator (Groovy 2.0+)
def cars = [[make: 'Ford', model: 'Focus'], [make: 'Honda', model: 'Civic']]
def models = cars*.model
println "Car models: $models"

// 28. Star-dot operator (Groovy 2.0+)
def numbersList = [[1, 2], [3, 4]]
def flattened = numbersList*.flatten()
println "Flattened list: $flattened"

// 29. Safe index operator (Groovy 2.0+)
def listWithNull = [1, null, 3]
def element = listWithNull?[1]
println "Safe indexed element: $element"

// 30. Closure composition
def addOne = { it + 1 }
def timesTwo = { it * 2 }
def addOneThenTimesTwo = addOne >> timesTwo
println "Composed closure: ${addOneThenTimesTwo(3)}" // (3 + 1) * 2 = 8

// 31. Custom operators (example, not actual operator definition)
// Groovy allows operator overloading, but defining new operators is not direct syntax.
// This is more about method calls that look like operators.

// 32. Type annotations for static compilation (Groovy 2.0+)
import groovy.transform.TypeChecked

@TypeChecked
class TypedClass {
    String typedMethod(String param) {
        return "Typed: $param"
    }
}
def typedObj = new TypedClass()
println typedObj.typedMethod("hello")

// 33. Default parameters in methods
def greetUser(name, greeting = "Hello") {
    println "$greeting, $name!"
}
greetUser("World")
greetUser("Alice", "Hi")

// 34. Named arguments for constructors
class Point {
    int x, y
    Point(int x, int y) { this.x = x; this.y = y }
}
def p = new Point(x: 10, y: 20)
println "Point: x=${p.x}, y=${p.y}"

// 35. Category classes (for dynamic method injection)
// use(StringCategories) { "hello".reverse() }

// 36. Mixins (for injecting behavior)
// @Mixin(MyMixin) class MyClassWithMixin {}

// 37. AST Transformations (e.g., @Canonical, @Immutable)
import groovy.transform.Canonical

@Canonical
class User {
    String firstName
    String lastName
}
def user1 = new User(firstName: "John", lastName: "Doe")
def user2 = new User("John", "Doe")
println "User equals: ${user1 == user2}"

// 38. Builder pattern (using @Builder)
import groovy.transform.builder.Builder

@Builder
class Product {
    String name
    double price
    int quantity
}
def product = Product.builder().name("Laptop").price(1200.0).quantity(1).build()
println "Product: $product.name, $product.price, $product.quantity"

// 39. File I/O (simple example)
def file = new File("test.txt")
file.write("Hello from Groovy file!")
println "File content: ${file.text}"
file.delete()

// 40. Concurrency (simple example with GParallelizer)
// @GParallelizer def parallelMethod() { ... }

// 41. Date and Time (using TimeCategory)
import groovy.time.TimeCategory
use(TimeCategory) {
    def now = new Date()
    def tomorrow = now + 1.day
    println "Tomorrow: $tomorrow"
}

// 42. XML Processing (using XmlSlurper)
// def xml = new XmlSlurper().parseText('<root><item>value</item></root>')
// println xml.item.text()

// 43. JSON Processing (using JsonSlurper)
// def json = new JsonSlurper().parseText('{\"name\": \"Groovy\"}')
// println json.name

// 44. Command execution
// def output = "ls -l".execute().text
// println "Command output: $output"

// 45. Elvis operator with assignment (Groovy 2.5+)
def myVar = null
myVar ?= "default value"
println "My var: $myVar"

// 46. Method reference (Groovy 2.3+)
def listToProcess = [1, 2, 3]
def doubledList = listToProcess.collect(this.&doubleValueMethod)
println "Doubled list: $doubledList"

def doubleValueMethod(num) {
    num * 2
}

// 47. Spread map operator (Groovy 2.4+)
def mapA = [a: 1, b: 2]
def mapB = [c: 3, d: 4]
def combinedMap = [*: mapA, *: mapB, e: 5]
println "Combined map: $combinedMap"

// 48. Closure with delegate
class DelegateTarget {
    String prefix = ">>"
    def message = "Hello"
    String getFullMessage() { "$prefix $message" }
}

def target = new DelegateTarget()
def myClosure = {
    "$prefix $message from closure"
}
myClosure.delegate = target
myClosure.resolveStrategy = Closure.DELEGATE_FIRST
println "Closure with delegate: ${myClosure()}"

// 49. Type inference with 'var' (Groovy 3.0+)
var inferredString = "inferred"
var inferredInt = 123
println "Inferred string: $inferredString, inferred int: $inferredInt"

// 50. Record-like classes (Groovy 4.0+)
// record PersonRecord(String name, int age) {}
// def personRecord = new PersonRecord("Jane", 28)
// println "Person record: $personRecord.name"

// 51. Sealed types (Groovy 4.0+)
// sealed interface Shape permits Circle, Square {}
// final class Circle implements Shape {}
// final class Square implements Shape {}

// 52. Pattern matching (Groovy 4.0+)
// def obj = [a: 1]
// def result = switch (obj) {
//     case [a: var x] -> "Map with a: $x"
//     default -> "Other"
// }
// println "Pattern matching result: $result"

// 53. Enhanced switch expression (Groovy 4.0+)
// def num = 2
// def description = switch (num) {
//     case 1 -> "One"
//     case 2 -> "Two"
//     default -> "Other"
// }
// println "Enhanced switch: $description"

// 54. Try-with-resources (Groovy 2.0+)
// new StringReader("test").withReader { reader ->
//     println "Read: ${reader.readLine()}"
// }

// 55. Using statement (Groovy 2.0+)
// use(SomeCategory) {
//     // methods from SomeCategory are available
// }

// 56. Optional type (Groovy 2.5+)
// def optionalValue = Optional.of("present")
// optionalValue.ifPresent { v -> println "Optional value: $v" }

// 57. Safe dereference for static members (Groovy 2.5+)
// def className = null
// def staticField = className?.staticField // Does not throw NPE

// 58. Multiple assignment / destructuring
def (x, y) = [10, 20]
println "Destructuring: x=$x, y=$y"

def (name, age) = ["Alice", 30]
println "Destructuring: name=$name, age=$age"

// 59. Type conversion (as operator)
def numStr = "123"
def numInt = numStr as Integer
println "Type conversion: $numInt"

// 60. Identity comparison (is)
def listA = [1, 2, 3]
def listB = listA
def listC = [1, 2, 3]
println "listA is listB: ${listA.is(listB)}"
println "listA is listC: ${listA.is(listC)}"

// 61. Ternary operator
def max = (a > b) ? a : b
println "Max: $max"

// 62. Assertions
assert 1 == 1 : "One should equal one"

// 63. Import statements
import java.util.Date
import static java.lang.Math.PI

println "Current date: ${new Date()}"
println "PI: $PI"

// 64. Package declaration
package com.example.test

// 65. Interface definition
interface MyInterface {
    void myMethod()
}

// 66. Abstract class
abstract class AbstractClass {
    abstract void abstractMethod()
    void concreteMethod() { println "Concrete method" }
}

// 67. Enum with methods
enum Color {
    RED {
        String getDescription() { "The color red" }
    },
    GREEN {
        String getDescription() { "The color green" }
    }
    abstract String getDescription()
}
println "Red description: ${Color.RED.getDescription()}"

// 68. Annotation definition
@interface MyAnnotation {
    String value() default "default"
    int count()
}

// 69. Usage of custom annotation
@MyAnnotation(value = "custom", count = 1)
class AnnotatedClass {
    @MyAnnotation(count = 2)
    def annotatedField
}

// 70. Closure with parameters and return type
def sumClosure: (int, int) -> int = { a, b -> a + b }
println "Sum closure: ${sumClosure(5, 7)}"

// 71. Trailing closures
def withConfig(Map config, Closure body) {
    body.delegate = config
    body.resolveStrategy = Closure.DELEGATE_FIRST
    body()
}

withConfig(timeout: 1000, retries: 3) {
    println "Timeout: $timeout"
    println "Retries: $retries"
}

// 72. Elvis operator with method call
def obj = null
def result = obj?.someMethod() ?: "default result"
println "Elvis with method call: $result"

// 73. Safe navigation with list access
def data = [items: [1, 2, 3]]
def firstItem = data?.items?[0]
println "Safe navigation with list: $firstItem"

// 74. Diamond operator for generics (Groovy 2.0+)
List<String> names = new ArrayList<>()
names.add("Alice")
println "Generic list: $names"

// 75. Type parameters for classes and methods
class Box<T> {
    T value
    Box(T value) { this.value = value }
    T get() { value }
}
def intBox = new Box<Integer>(10)
println "Box value: ${intBox.get()}"

// 76. Varargs
def sumAll(int... nums) {
    nums.sum()
}
println "Sum all: ${sumAll(1, 2, 3, 4)}"

// 77. Static imports for fields
import static java.lang.System.out

out.println("Static import for out")

// 78. Static imports for methods
import static java.lang.Math.max

println "Max of 10 and 20: ${max(10, 20)}"

// 79. Multiple assignment with map
def mapData = [name: "Bob", age: 25]
def (personName, personAge) = mapData.values()
println "Map destructuring: $personName, $personAge"

// 80. Closure coercion to SAM type (Single Abstract Method)
Runnable myRunnable = { println "Running from closure" }
myRunnable.run()

// 81. Spread operator with maps (Groovy 2.4+)
def baseConfig = [host: "localhost", port: 8080]
def devConfig = [env: "development", *: baseConfig]
println "Dev config: $devConfig"

// 82. Identity comparison for numbers (== vs .is())
def intA = 127
def intB = 127
def intC = 128
def intD = 128

println "intA == intB: ${intA == intB}"
println "intA.is(intB): ${intA.is(intB)}" // For small integers, often true due to caching

println "intC == intD: ${intC == intD}"
println "intC.is(intD): ${intC.is(intD)}" // For larger integers, often false

// 83. Using 'def' for dynamic method definition
def myDynamicMethod = { arg -> "Dynamic method with $arg" }
this.metaClass.dynamicMethod = myDynamicMethod
println dynamicMethod("test")

// 84. Property accessors (getters/setters generated automatically)
class ProductBean {
    String name
    double price
}
def productBean = new ProductBean(name: "Book", price: 25.0)
println "Product bean name: ${productBean.name}"
productBean.price = 30.0
println "Product bean price: ${productBean.price}"

// 85. Operator overloading (example with plus operator)
class Vector {
    int x, y
    Vector(int x, int y) { this.x = x; this.y = y }
    Vector plus(Vector other) { new Vector(x + other.x, y + other.y) }
    String toString() { "($x, $y)" }
}
def v1 = new Vector(1, 2)
def v2 = new Vector(3, 4)
def v3 = v1 + v2
println "Vector sum: $v3"

// 86. Closure with owner and delegate
class Outer {
    def outerVar = "Outer"
    def getClosure() {
        def innerVar = "Inner"
        return {
            println "Owner: ${owner.outerVar}"
            println "Delegate: ${delegate.outerVar}"
            println "Local: $innerVar"
        }
    }
}
def outer = new Outer()
def c = outer.getClosure()
c.delegate = outer // Set delegate to owner for this example
c()

// 87. Using 'with' statement
class Config {
    String url
    int timeout
}
def config = new Config()
config.with {
    url = "http://example.com"
    timeout = 5000
}
println "Config URL: ${config.url}, Timeout: ${config.timeout}"

// 88. Invoking closures with 'call'
def myCallable = { a, b -> a * b }
println "Callable result: ${myCallable.call(6, 7)}"

// 89. Using 'as' for interface implementation (SAM coercion)
interface GreeterInterface {
    String sayHello(String name)
}
GreeterInterface greeter = { name -> "Hello, $name!" } as GreeterInterface
println greeter.sayHello("World")

// 90. Multiple assignment with properties
class Coordinates {
    int lat, lon
}
def coords = new Coordinates(lat: 34, lon: -118)
def (latitude, longitude) = coords
println "Coordinates: $latitude, $longitude"

// 91. Using 'yield' in closures (for generators)
def generateNumbers = { max ->
    for (i in 1..max) {
        yield i
    }
}
def generator = generateNumbers(3)
generator.each { num -> println "Generated: $num" }

// 92. Safe method call operator (Groovy 2.0+)
def maybeNull = null
maybeNull?.doSomething() // No NPE

// 93. Ternary operator with closures
def check = true
def action = check ? { println "True action" } : { println "False action" }
action()

// 94. Using 'synchronized' block
def lock = new Object()
synchronized (lock) {
    println "Inside synchronized block"
}

// 95. Using 'final' keyword
final int FINAL_VAR = 100
final class FinalClass {}
final def finalClosure = { -> "Final closure" }

// 96. Using 'static' block
class StaticBlockExample {
    static {
        println "Static block executed"
    }
    static String STATIC_FIELD = "Static Field"
}
println StaticBlockExample.STATIC_FIELD

// 97. Using 'enum' as a switch expression (Groovy 4.0+)
// def status = Status.ACTIVE
// def message = switch (status) {
//     case ACTIVE -> "Active status"
//     case INACTIVE -> "Inactive status"
//     default -> "Unknown status"
// }
// println "Status message: $message"

// 98. Using 'var' for loop variables (Groovy 3.0+)
for (var i = 0; i < 3; i++) {
    println "Var loop: $i"
}

// 99. Using 'var' with closures
var myVarClosure = { a, b -> a + b }
println "Var closure: ${myVarClosure(1, 2)}"

// 100. Using 'var' with collections
var myList = [1, 2, 3]
println "Var list: $myList"
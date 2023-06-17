// Scala test file for lexer testing

package com.example.test

import scala.collection.mutable
import scala.concurrent.{Future, ExecutionContext}
import scala.util.{Try, Success, Failure, Random}
import java.time.LocalDateTime
import java.util.UUID

// Implicit execution context for futures
implicit val ec: ExecutionContext = ExecutionContext.global

// Object with main method
object ScalaTestApp extends App {
  println("=== Scala Test Application ===")
  
  // Call various test methods
  testBasicTypes()
  testCollections()
  testFunctions()
  testClasses()
  testTraits()
  testCaseClasses()
  testPatternMatching()
  testForComprehensions()
  testFutures()
  testImplicits()
  testHigherOrderFunctions()
  
  println("=== End of Test Application ===")
}

// Basic data types and variables
def testBasicTypes(): Unit = {
  println("\n=== Basic Types Test ===")
  
  // Immutable variables (val)
  val intVal: Int = 42
  val longVal: Long = 42L
  val floatVal: Float = 3.14f
  val doubleVal: Double = 3.14159
  val boolVal: Boolean = true
  val charVal: Char = 'A'
  val stringVal: String = "Hello, Scala!"
  
  // Mutable variables (var)
  var mutableInt: Int = 10
  mutableInt += 5
  
  // Type inference
  val inferredInt = 100
  val inferredString = "Inferred type"
  
  // Null and Option
  val nullableString: String = null
  val optionString: Option[String] = Some("Optional value")
  val noneOption: Option[String] = None
  
  // Unit type (equivalent to void)
  val unitVal: Unit = ()
  
  println(s"Int: $intVal, String: $stringVal, Mutable: $mutableInt")
  println(s"Option: ${optionString.getOrElse("default")}")
}

// Collections
def testCollections(): Unit = {
  println("\n=== Collections Test ===")
  
  // Lists (immutable)
  val list1 = List(1, 2, 3, 4, 5)
  val list2 = 0 :: list1  // Prepend
  val list3 = list1 :+ 6  // Append
  
  // Arrays (mutable)
  val array = Array(1, 2, 3, 4, 5)
  array(0) = 10
  
  // Vectors (immutable, efficient random access)
  val vector = Vector(1, 2, 3, 4, 5)
  
  // Sets
  val set1 = Set(1, 2, 3, 4, 5)
  val set2 = Set(4, 5, 6, 7, 8)
  val intersection = set1 intersect set2
  val union = set1 union set2
  
  // Maps
  val map1 = Map("a" -> 1, "b" -> 2, "c" -> 3)
  val map2 = Map(("x", 10), ("y", 20), ("z", 30))
  
  // Mutable collections
  val mutableList = mutable.ListBuffer(1, 2, 3)
  mutableList += 4
  mutableList ++= List(5, 6)
  
  val mutableMap = mutable.Map("key1" -> "value1")
  mutableMap("key2") = "value2"
  
  println(s"List: $list1")
  println(s"Set intersection: $intersection")
  println(s"Map value for 'a': ${map1.get("a")}")
  println(s"Mutable list: $mutableList")
}

// Functions
def testFunctions(): Unit = {
  println("\n=== Functions Test ===")
  
  // Simple function
  def add(x: Int, y: Int): Int = x + y
  
  // Function with default parameters
  def greet(name: String, greeting: String = "Hello"): String = 
    s"$greeting, $name!"
  
  // Function with variable arguments
  def sum(numbers: Int*): Int = numbers.sum
  
  // Higher-order function
  def applyOperation(x: Int, y: Int, op: (Int, Int) => Int): Int = op(x, y)
  
  // Anonymous functions (lambdas)
  val multiply = (x: Int, y: Int) => x * y
  val square = (x: Int) => x * x
  
  // Partially applied functions
  val addFive = add(5, _: Int)
  
  // Curried functions
  def curriedAdd(x: Int)(y: Int): Int = x + y
  val addTen = curriedAdd(10) _
  
  // Recursive function
  def factorial(n: Int): Int = {
    if (n <= 1) 1
    else n * factorial(n - 1)
  }
  
  // Tail recursive function
  def fibonacciTail(n: Int): Long = {
    @annotation.tailrec
    def fibHelper(n: Int, a: Long, b: Long): Long = {
      if (n == 0) a
      else fibHelper(n - 1, b, a + b)
    }
    fibHelper(n, 0, 1)
  }
  
  println(s"Add: ${add(3, 4)}")
  println(s"Greet: ${greet("Alice")}")
  println(s"Sum: ${sum(1, 2, 3, 4, 5)}")
  println(s"Apply operation: ${applyOperation(6, 7, multiply)}")
  println(s"Add five: ${addFive(10)}")
  println(s"Factorial: ${factorial(5)}")
  println(s"Fibonacci: ${fibonacciTail(10)}")
}

// Classes and objects
class Person(val name: String, var age: Int) {
  // Secondary constructor
  def this(name: String) = this(name, 0)
  
  // Method
  def greet(): String = s"Hello, I'm $name and I'm $age years old"
  
  // Method with parameters
  def haveBirthday(): Unit = {
    age += 1
    println(s"$name is now $age years old")
  }
  
  // Override toString
  override def toString: String = s"Person($name, $age)"
}

// Companion object
object Person {
  def apply(name: String, age: Int): Person = new Person(name, age)
  
  def apply(name: String): Person = new Person(name)
  
  val defaultAge = 0
}

// Inheritance
class Employee(name: String, age: Int, val employeeId: String, var salary: Double) 
  extends Person(name, age) {
  
  def this(name: String, employeeId: String) = this(name, 0, employeeId, 0.0)
  
  def work(): String = s"$name is working (ID: $employeeId)"
  
  def raiseSalary(percentage: Double): Unit = {
    salary *= (1 + percentage / 100)
  }
  
  override def toString: String = s"Employee($name, $age, $employeeId, $salary)"
}

def testClasses(): Unit = {
  println("\n=== Classes Test ===")
  
  val person1 = new Person("John", 30)
  val person2 = Person("Jane", 25)  // Using companion object apply
  val person3 = Person("Bob")       // Using secondary constructor
  
  val employee = new Employee("Alice", 28, "EMP001", 50000.0)
  
  println(person1.greet())
  person1.haveBirthday()
  
  println(employee.work())
  employee.raiseSalary(10.0)
  println(s"Employee after raise: $employee")
}

// Traits (similar to interfaces)
trait Drawable {
  def draw(): String
}

trait Movable {
  def move(x: Int, y: Int): String
}

trait Resizable {
  def resize(factor: Double): String
}

// Abstract class
abstract class Shape(val name: String) extends Drawable {
  def area(): Double
  def perimeter(): Double
  
  def describe(): String = s"This is a $name"
}

// Concrete class implementing traits
class Circle(val radius: Double) extends Shape("Circle") with Movable with Resizable {
  def area(): Double = math.Pi * radius * radius
  def perimeter(): Double = 2 * math.Pi * radius
  
  def draw(): String = s"Drawing a circle with radius $radius"
  def move(x: Int, y: Int): String = s"Moving circle to ($x, $y)"
  def resize(factor: Double): String = {
    val newRadius = radius * factor
    s"Resizing circle to radius $newRadius"
  }
}

class Rectangle(val width: Double, val height: Double) extends Shape("Rectangle") with Drawable {
  def area(): Double = width * height
  def perimeter(): Double = 2 * (width + height)
  
  def draw(): String = s"Drawing a rectangle ${width}x${height}"
}

def testTraits(): Unit = {
  println("\n=== Traits Test ===")
  
  val circle = new Circle(5.0)
  val rectangle = new Rectangle(4.0, 6.0)
  
  val shapes: List[Shape] = List(circle, rectangle)
  
  shapes.foreach { shape =>
    println(shape.describe())
    println(shape.draw())
    println(s"Area: ${shape.area()}, Perimeter: ${shape.perimeter()}")
  }
  
  println(circle.move(10, 20))
  println(circle.resize(1.5))
}

// Case classes
case class Point(x: Double, y: Double) {
  def distance(other: Point): Double = {
    math.sqrt(math.pow(x - other.x, 2) + math.pow(y - other.y, 2))
  }
  
  def +(other: Point): Point = Point(x + other.x, y + other.y)
  def -(other: Point): Point = Point(x - other.x, y - other.y)
  def *(scalar: Double): Point = Point(x * scalar, y * scalar)
}

case class User(id: UUID, name: String, email: String, age: Int, isActive: Boolean = true)

// Sealed traits for ADTs (Algebraic Data Types)
sealed trait Color
case object Red extends Color
case object Green extends Color
case object Blue extends Color
case class RGB(r: Int, g: Int, b: Int) extends Color

sealed trait Tree[+A]
case class Leaf[A](value: A) extends Tree[A]
case class Branch[A](left: Tree[A], right: Tree[A]) extends Tree[A]
case object Empty extends Tree[Nothing]

def testCaseClasses(): Unit = {
  println("\n=== Case Classes Test ===")
  
  val point1 = Point(3.0, 4.0)
  val point2 = Point(1.0, 2.0)
  val point3 = point1 + point2
  
  println(s"Point1: $point1")
  println(s"Point2: $point2")
  println(s"Sum: $point3")
  println(s"Distance: ${point1.distance(point2)}")
  
  val user1 = User(UUID.randomUUID(), "Alice", "alice@example.com", 25)
  val user2 = user1.copy(name = "Alice Smith")  // Copy with modification
  
  println(s"User1: $user1")
  println(s"User2: $user2")
  
  // Pattern matching with case classes
  val color: Color = RGB(255, 128, 0)
  val colorName = color match {
    case Red => "Red"
    case Green => "Green"
    case Blue => "Blue"
    case RGB(r, g, b) => s"RGB($r, $g, $b)"
  }
  println(s"Color: $colorName")
}

// Pattern matching
def testPatternMatching(): Unit = {
  println("\n=== Pattern Matching Test ===")
  
  // Basic pattern matching
  def describe(x: Any): String = x match {
    case 0 => "zero"
    case 1 => "one"
    case i: Int if i > 1 => s"positive integer: $i"
    case i: Int if i < 0 => s"negative integer: $i"
    case s: String => s"string: $s"
    case l: List[_] => s"list with ${l.length} elements"
    case _ => "something else"
  }
  
  // Pattern matching with case classes
  def processUser(user: User): String = user match {
    case User(_, name, _, age, true) if age >= 18 => s"Adult user: $name"
    case User(_, name, _, age, true) if age < 18 => s"Minor user: $name"
    case User(_, name, _, _, false) => s"Inactive user: $name"
  }
  
  // Pattern matching with collections
  def processList(list: List[Int]): String = list match {
    case Nil => "empty list"
    case head :: Nil => s"single element: $head"
    case head :: tail => s"head: $head, tail: $tail"
  }
  
  // Pattern matching with tuples
  def processCoordinate(coord: (Int, Int)): String = coord match {
    case (0, 0) => "origin"
    case (x, 0) => s"on x-axis at $x"
    case (0, y) => s"on y-axis at $y"
    case (x, y) => s"point at ($x, $y)"
  }
  
  println(describe(42))
  println(describe("Hello"))
  println(describe(List(1, 2, 3)))
  
  val user = User(UUID.randomUUID(), "Bob", "bob@example.com", 30)
  println(processUser(user))
  
  println(processList(List(1, 2, 3)))
  println(processCoordinate((5, 10)))
}

// For comprehensions
def testForComprehensions(): Unit = {
  println("\n=== For Comprehensions Test ===")
  
  // Simple for comprehension
  val numbers = for (i <- 1 to 10) yield i * i
  println(s"Squares: $numbers")
  
  // For comprehension with filter
  val evenSquares = for {
    i <- 1 to 10
    if i % 2 == 0
  } yield i * i
  println(s"Even squares: $evenSquares")
  
  // Multiple generators
  val combinations = for {
    x <- 1 to 3
    y <- 1 to 3
    if x != y
  } yield (x, y)
  println(s"Combinations: $combinations")
  
  // For comprehension with side effects
  for {
    i <- 1 to 5
    j <- 1 to 3
  } println(s"($i, $j)")
  
  // Nested for comprehensions
  val matrix = for {
    row <- 1 to 3
  } yield {
    for {
      col <- 1 to 3
    } yield row * col
  }
  println(s"Matrix: $matrix")
}

// Futures and asynchronous programming
def testFutures(): Unit = {
  println("\n=== Futures Test ===")
  
  // Simple future
  val future1 = Future {
    Thread.sleep(100)
    42
  }
  
  // Future with transformation
  val future2 = future1.map(_ * 2)
  
  // Future with flatMap
  val future3 = future1.flatMap(x => Future(x + 10))
  
  // Future with error handling
  val future4 = Future {
    if (Random.nextBoolean()) throw new RuntimeException("Random error")
    else "Success"
  }
  
  val recoveredFuture = future4.recover {
    case _: RuntimeException => "Recovered from error"
  }
  
  // Combining futures
  val combinedFuture = for {
    a <- future1
    b <- future2
    c <- future3
  } yield (a, b, c)
  
  // Using Try for synchronous error handling
  val tryResult = Try {
    10 / 0
  } match {
    case Success(value) => s"Success: $value"
    case Failure(exception) => s"Failure: ${exception.getMessage}"
  }
  
  println(s"Try result: $tryResult")
  
  // Note: In a real application, you would use Await.result or callbacks
  // Here we just demonstrate the syntax
}

// Implicit parameters and conversions
implicit class StringExtensions(s: String) {
  def isPalindrome: Boolean = s == s.reverse
  def wordCount: Int = s.split("\\s+").length
  def toTitleCase: String = s.split(" ").map(_.capitalize).mkString(" ")
}

implicit class IntExtensions(i: Int) {
  def times(f: => Unit): Unit = {
    for (_ <- 1 to i) f
  }
  
  def factorial: Long = {
    if (i <= 1) 1L
    else i * (i - 1).factorial
  }
}

// Implicit parameters
def greetWithContext(name: String)(implicit greeting: String): String = 
  s"$greeting, $name!"

def testImplicits(): Unit = {
  println("\n=== Implicits Test ===")
  
  // Implicit conversions
  val text = "hello world"
  println(s"'$text' is palindrome: ${text.isPalindrome}")
  println(s"Word count: ${text.wordCount}")
  println(s"Title case: ${text.toTitleCase}")
  
  // Implicit class for Int
  println(s"5 factorial: ${5.factorial}")
  
  3.times {
    print("Hello ")
  }
  println()
  
  // Implicit parameters
  implicit val defaultGreeting: String = "Hi"
  println(greetWithContext("Alice"))
}

// Higher-order functions and functional programming
def testHigherOrderFunctions(): Unit = {
  println("\n=== Higher-Order Functions Test ===")
  
  val numbers = List(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)
  
  // Map, filter, reduce
  val doubled = numbers.map(_ * 2)
  val evens = numbers.filter(_ % 2 == 0)
  val sum = numbers.reduce(_ + _)
  val product = numbers.fold(1)(_ * _)
  
  println(s"Original: $numbers")
  println(s"Doubled: $doubled")
  println(s"Evens: $evens")
  println(s"Sum: $sum")
  println(s"Product: $product")
  
  // GroupBy
  val grouped = numbers.groupBy(_ % 3)
  println(s"Grouped by mod 3: $grouped")
  
  // FlatMap
  val nested = List(List(1, 2), List(3, 4), List(5, 6))
  val flattened = nested.flatMap(identity)
  println(s"Flattened: $flattened")
  
  // Zip
  val letters = List("a", "b", "c", "d", "e")
  val zipped = numbers.zip(letters)
  println(s"Zipped: $zipped")
  
  // Partition
  val (odds, evens2) = numbers.partition(_ % 2 == 1)
  println(s"Odds: $odds, Evens: $evens2")
  
  // Find and exists
  val firstEven = numbers.find(_ % 2 == 0)
  val hasLargeNumber = numbers.exists(_ > 8)
  println(s"First even: $firstEven")
  println(s"Has number > 8: $hasLargeNumber")
  
  // Sorting
  val shuffled = Random.shuffle(numbers)
  val sorted = shuffled.sorted
  val sortedDesc = shuffled.sortWith(_ > _)
  println(s"Shuffled: $shuffled")
  println(s"Sorted: $sorted")
  println(s"Sorted desc: $sortedDesc")
}
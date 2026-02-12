package examples.lexer

import scala.collection.mutable.ListBuffer
import scala.concurrent.{Future, ExecutionContext}
import scala.util.{Try, Success, Failure}

/**
 * A comprehensive Scala lexer test.
 * Covering various language features.
 */
object BasicScala {
    val Pi = 3.14159
    var count = 0
    
    // Type alias
    type StringList = List[String]

    // Case class
    case class Person(name: String, age: Int)

    // Trait
    trait Greeter {
        def greet(name: String): Unit = {
            println(s"Hello, $name!")
        }
    }

    // Class implementing trait
    class Employee(name: String, age: Int, val role: String) extends Person(name, age) with Greeter {
        override def greet(name: String): Unit = {
            println(s"Greetings, $name. I am a $role.")
        }
    }

    def main(args: Array[String]): Unit = {
        // Variables
        val x: Int = 42
        val y = 100L
        val text = "Hello, Scala!"
        val multiline = """
            This is a
            multiline string
        """
        val symbol = 'symbol
        
        // Collections
        val list = List(1, 2, 3)
        val map = Map("a" -> 1, "b" -> 2)
        val set = Set(1, 2, 3)

        // Control Structures
        if (x > 10) {
            println("Greater than 10")
        } else {
            println("Less or equal")
        }

        for (i <- 1 to 5) {
            println(i)
        }

        while (count < 5) {
            count += 1
        }

        // Pattern Matching
        val result = x match {
            case 1 => "one"
            case 2 => "two"
            case _ => "other"
        }

        // Higher-order functions
        val doubled = list.map(_ * 2)
        val filtered = list.filter(_ > 1)

        // Try/Success/Failure
        val tried = Try {
            Integer.parseInt("123")
        } match {
            case Success(v) => v
            case Failure(e) => -1
        }

        // XML Literal (Scala 2 feature, but common)
        val xml = <elem attribute="value">Content</elem>

        // Implicit
        implicit val context: String = "Context"
        printContext
    }

    def printContext(implicit ctx: String): Unit = {
        println(ctx)
    }

    // Generic function
    def first[T](list: List[T]): Option[T] = {
        list.headOption
    }
}

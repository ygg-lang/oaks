/*
 * Comprehensive Kotlin Lexer Test
 * Package and Imports
 */
package com.example.oak.test

import java.util.Date
import kotlin.collections.*
import kotlin.text.Regex as KRegex

// Top-level constants and variables
const val PI = 3.14159
var globalCounter = 0
val immutableList = listOf("a", "b", "c")

// Type Aliases
typealias Name = String
typealias Handler = (Int) -> Unit

// Enumerations
enum class Direction(val angle: Int) {
    NORTH(0), SOUTH(180), WEST(270), EAST(90);
    
    fun description() = "Direction: $name ($angle)"
}

// Sealed Classes
sealed class Result
data class Success(val data: String) : Result()
data class Error(val exception: Exception) : Result()
object Loading : Result()

// Interfaces
interface Printable {
    fun printMe()
    val format: String
        get() = "Default"
}

// Classes with primary constructor
open class Person(val name: String, var age: Int) : Printable {
    // Secondary constructor
    constructor(name: String) : this(name, 0)
    
    // Properties with custom accessors
    var isAdult: Boolean
        get() = age >= 18
        set(value) {
            if (value) age = 18 else age = 0
        }
        
    // Initializer block
    init {
        println("Person initialized: $name")
    }
    
    override fun printMe() {
        println("Person: $name, Age: $age")
    }
    
    // Infix function
    infix fun likes(other: Person): Boolean {
        return true
    }
    
    // Companion Object
    companion object Factory {
        fun create(): Person = Person("Unknown")
    }
}

// Data Class
data class Point(val x: Int, val y: Int)

// Extension Functions
fun String.toSlug(): String = this.lowercase().replace(" ", "-")

// Higher-Order Functions and Lambdas
fun operate(a: Int, b: Int, operation: (Int, Int) -> Int): Int {
    return operation(a, b)
}

// Main Function
fun main(args: Array<String>) {
    // Variables
    val x: Int = 10
    var y = 20
    
    // String Templates
    println("Sum: ${x + y}")
    println("""
        Multi-line string
        with trimMargin
    """.trimIndent())
    
    // Control Flow
    val max = if (x > y) x else y
    
    when (x) {
        1 -> println("One")
        in 2..10 -> println("Between 2 and 10")
        else -> println("Other")
    }
    
    // Loops
    for (i in 1..5) println(i)
    for (i in 10 downTo 1 step 2) println(i)
    
    var index = 0
    while (index < 5) {
        index++
    }
    
    // Ranges
    val range = 1..10
    
    // Null Safety
    var nullable: String? = "Hello"
    nullable = null
    val length = nullable?.length ?: 0
    
    // Safe Cast
    val num = "123" as? Int
    
    // Collections
    val map = mapOf("key" to "value", "a" to 1)
    val list = mutableListOf<Int>()
    list.add(1)
    
    // Lambdas
    val sum = operate(10, 20) { a, b -> a + b }
    val product = operate(5, 5) { a, b -> 
        a * b 
    }
    
    // Destructuring Declaration
    val (px, py) = Point(10, 20)
    
    // Exception Handling
    try {
        throw IllegalArgumentException("Error")
    } catch (e: Exception) {
        println("Caught: ${e.message}")
    } finally {
        println("Finally")
    }
    
    // Annotations
    @Deprecated("Use newMethod instead")
    fun oldMethod() {}
    
    // Delegated Properties
    val lazyValue: String by lazy {
        println("Computed!")
        "Hello"
    }
}

// Generics
class Box<T>(t: T) {
    var value = t
}

fun <T> singletonList(item: T): List<T> {
    return listOf(item)
}

// Coroutines (Syntax only)
suspend fun doSomethingAsync() {
    // ...
}

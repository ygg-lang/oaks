// Test file for Kotlin features

// Data class
data class Person(val name: String, val age: Int)

// Sealed class
sealed class Result<T>
class Success<T>(val value: T) : Result<T>()
class Failure<T>(val error: String) : Result<T>()

// Extension function
fun String.isPalindrome(): Boolean {
    return this == this.reversed()
}

// Regular class
class Calculator {
    fun add(a: Int, b: Int): Int {
        return a + b
    }
}

fun main() {
    // Test data class
    val person = Person("John", 30)
    println("Person: person")
    
    // Test sealed class
    val success: Result<Int> = Success(42)
    val failure: Result<Int> = Failure("Error")
    
    // Test extension function
    val text = "level"
    println("Is \"\$text\" a palindrome? {text.isPalindrome()}")
    
    // Test regular class
    val calculator = Calculator()
    println("5 + 3 = {calculator.add(5, 3)}")
}

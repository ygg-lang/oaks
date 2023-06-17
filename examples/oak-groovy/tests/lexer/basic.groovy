// Basic Groovy syntax test file
class Person {
    String name
    int age
    
    Person(String name, int age) {
        this.name = name
        this.age = age
    }
    
    String greet() {
        return "Hello, I'm $name and I'm $age years old"
    }
}

// Dynamic typing
def person = new Person("Alice", 25)
println person.greet()

// Closures
def numbers = [1, 2, 3, 4, 5]
def squared = numbers.collect { it * it }
println "Squared: $squared"

// Maps and ranges
def map = [name: "Bob", age: 30, city: "New York"]
println "Name: ${map.name}"

// Range
(1..5).each { println "Number: $it" }

// String interpolation
def name = "Charlie"
def message = "Welcome, $name!"
println message

// Regular expressions
def pattern = ~/^[A-Za-z]+$/
def text = "HelloWorld"
println "Matches pattern: ${text ==~ pattern}"

// Method with default parameters
def calculateArea(double radius, double pi = 3.14159) {
    return pi * radius * radius
}

def area = calculateArea(5.0)
println "Area: $area"
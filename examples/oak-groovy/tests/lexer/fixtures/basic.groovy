class HelloWorld {
    static void main(String[] args) {
        println "Hello, World!"
        def name = "Groovy"
        def age = 25
        
        if (age > 18) {
            println "Adult: $name"
        }
        
        def numbers = [1, 2, 3, 4, 5]
        numbers.each { num ->
            println "Number: $num"
        }
    }
    
    def calculate(int x, int y) {
        return x + y
    }
}

class Person {
    String name
    int age
    
    Person(String name, int age) {
        this.name = name
        this.age = age
    }
    
    def greet() {
        return "Hello, I'm $name and I'm $age years old"
    }
}
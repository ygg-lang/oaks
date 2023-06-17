// Swift test file
import Foundation

// Variables and constants
let name = "Swift"
var count = 0

// Functions
func greet(person: String) -> String {
    return "Hello, \(person)!"
}

func add(_ a: Int, _ b: Int) -> Int {
    return a + b
}

// Classes
class Person {
    var name: String
    var age: Int
    
    init(name: String, age: Int) {
        self.name = name
        self.age = age
    }
    
    func introduce() {
        print("Hi, I'm \(name) and I'm \(age) years old.")
    }
}

// Structs
struct Point {
    var x: Double
    var y: Double
    
    func distance(to other: Point) -> Double {
        let dx = x - other.x
        let dy = y - other.y
        return sqrt(dx * dx + dy * dy)
    }
}

// Enums
enum Direction {
    case north, south, east, west
}

// Arrays and dictionaries
let numbers = [1, 2, 3, 4, 5]
let squares = numbers.map { $0 * $0 }

var scores = ["Alice": 95, "Bob": 87, "Charlie": 92]

// Control flow
for number in numbers {
    if number % 2 == 0 {
        print("\(number) is even")
    } else {
        print("\(number) is odd")
    }
}

// Optional handling
var optionalName: String? = "John"
if let name = optionalName {
    print("Name is \(name)")
}

// Guard statement
func processAge(_ age: Int?) {
    guard let validAge = age, validAge >= 0 else {
        print("Invalid age")
        return
    }
    print("Age is \(validAge)")
}
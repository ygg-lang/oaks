// Comprehensive Swift Lexer Test
import Foundation

// Variables and Constants
var myVariable = 42
let myConstant = 3.14159
let implicitInteger = 70
let implicitDouble = 70.0
let explicitDouble: Double = 70
let label = "The width is "
let width = 94
let widthLabel = label + String(width)

// String Interpolation
let apples = 3
let appleSummary = "I have \(apples) apples."

// Multiline Strings
let quotation = """
The White Rabbit put on his spectacles.  "Where shall I begin,
please your Majesty?" he asked.

"Begin at the beginning," the King said gravely, "and go on
till you come to the end: then stop."
"""

// Arrays and Dictionaries
var shoppingList = ["catfish", "water", "tulips"]
shoppingList[1] = "bottle of water"

var occupations = [
    "Malcolm": "Captain",
    "Kaylee": "Mechanic",
]
occupations["Jayne"] = "Public Relations"

// Control Flow
let scores = [75, 43, 103, 87, 12]
for score in scores {
    if score > 50 {
        print("Pass")
    } else {
        print("Fail")
    }
}

var optionalString: String? = "Hello"
if let name = optionalString {
    print("Hello, \(name)")
}

let nickName: String? = nil
let fullName: String = "John Appleseed"
let informalGreeting = "Hi \(nickName ?? fullName)"

// Switch
let vegetable = "red pepper"
switch vegetable {
case "celery":
    print("Add some raisins and make ants on a log.")
case "cucumber", "watercress":
    print("That would make a good tea sandwich.")
case let x where x.hasSuffix("pepper"):
    print("Is it a spicy \(x)?")
default:
    print("Everything tastes good in soup.")
}

// Functions and Closures
func greet(person: String, day: String) -> String {
    return "Hello \(person), today is \(day)."
}

func calculateStatistics(scores: [Int]) -> (min: Int, max: Int, sum: Int) {
    return (1, 100, 5050)
}

let numbers = [20, 19, 7, 12]
let mappedNumbers = numbers.map({ number in 3 * number })
let sortedNumbers = numbers.sorted { $0 > $1 }

// Classes
class Shape {
    var numberOfSides = 0
    func simpleDescription() -> String {
        return "A shape with \(numberOfSides) sides."
    }
}

class Square: Shape {
    var sideLength: Double
    
    init(sideLength: Double) {
        self.sideLength = sideLength
        super.init()
        numberOfSides = 4
    }
    
    var perimeter: Double {
        get {
            return 3.0 * sideLength
        }
        set {
            sideLength = newValue / 3.0
        }
    }
    
    override func simpleDescription() -> String {
        return "A square with sides of length \(sideLength)."
    }
}

// Enums and Structures
enum Rank: Int {
    case ace = 1
    case two, three, four, five, six, seven, eight, nine, ten
    case jack, queen, king
    
    func simpleDescription() -> String {
        switch self {
        case .ace:
            return "ace"
        case .jack:
            return "jack"
        case .queen:
            return "queen"
        case .king:
            return "king"
        default:
            return String(self.rawValue)
        }
    }
}

struct Card {
    var rank: Rank
    var suit: String
    
    func simpleDescription() -> String {
        return "The \(rank.simpleDescription()) of \(suit)"
    }
}

// Protocols and Extensions
protocol ExampleProtocol {
    var simpleDescription: String { get }
    mutating func adjust()
}

extension Int: ExampleProtocol {
    var simpleDescription: String {
        return "The number \(self)"
    }
    mutating func adjust() {
        self += 42
    }
}

// Error Handling
enum PrinterError: Error {
    case outOfPaper
    case noToner
    case onFire
}

func send(job: Int, toPrinter printerName: String) throws -> String {
    if printerName == "Never Has Toner" {
        throw PrinterError.noToner
    }
    return "Job sent"
}

do {
    let printerResponse = try send(job: 1040, toPrinter: "Bi Sheng")
    print(printerResponse)
} catch {
    print(error)
}

// Generics
func makeArray<Item>(repeating item: Item, numberOfTimes: Int) -> [Item] {
    var result = [Item]()
    for _ in 0..<numberOfTimes {
        result.append(item)
    }
    return result
}

// Defer
func cleanup() {
    defer { print("Cleaning up...") }
    print("Working...")
}

// Guard
func greet(person: [String: String]) {
    guard let name = person["name"] else {
        return
    }
    print("Hello \(name)!")
}

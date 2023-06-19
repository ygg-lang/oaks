// D language test file for lexer testing

module basic;

import std.stdio;
import std.string;
import std.conv;
import std.algorithm;

// Single-line comment

/*
 * Multi-line comment
 * covering several lines.
 */

/// DDoc comment for the main function.
void main()
{
    // Basic types and variable declarations
    int integerVar = 10;
    float floatVar = 10.5f;
    double doubleVar = 20.25;
    char charVar = 'A';
    string stringVar = "Hello, D!";
    bool boolVar = true;
    auto inferredVar = 123.456;

    const int constVar = 100;
    immutable string immutableVar = "Immutable string";

    enum Color { Red, Green, Blue };
    Color myColor = Color.Green;

    // Output
    writeln("Integer: ", integerVar);
    writeln("Float: ", floatVar);
    writeln("Double: ", doubleVar);
    writeln("Char: ", charVar);
    writeln("String: ", stringVar);
    writeln("Boolean: ", boolVar);
    writeln("Inferred: ", inferredVar);
    writeln("Const: ", constVar);
    writeln("Immutable: ", immutableVar);
    writeln("Enum Color: ", myColor);

    // Control flow
    if (integerVar > 5) {
        writeln("Integer is greater than 5.");
    } else if (integerVar == 5) {
        writeln("Integer is 5.");
    } else {
        writeln("Integer is less than 5.");
    }

    switch (myColor) {
        case Color.Red:
            writeln("Color is Red.");
            break;
        case Color.Green:
            writeln("Color is Green.");
            break;
        default:
            writeln("Color is Blue.");
            break;
    }

    for (int i = 0; i < 3; ++i) {
        writeln("For loop iteration: ", i);
    }

    int j = 0;
    while (j < 2) {
        writeln("While loop iteration: ", j);
        j++;
    }

    int k = 0;
    do {
        writeln("Do-while loop iteration: ", k);
        k++;
    } while (k < 1);

    int[] dynamicArray = [10, 20, 30];
    foreach (value; dynamicArray) {
        writeln("Foreach value: ", value);
    }

    // Functions
    int sum = add(integerVar, 20);
    writeln("Sum: ", sum);

    auto lambdaResult = (int a, int b) => a * b;
    writeln("Lambda result: ", lambdaResult(5, 6));

    // Classes and Structs
    auto myPoint = Point(1, 2);
    writeln("Point: (", myPoint.x, ", ", myPoint.y, ")");

    auto myCar = new Car("Toyota");
    myCar.start();

    // Arrays and Associative Arrays
    int[3] staticArray = [1, 2, 3];
    string[string] assocArray;
    assocArray["key1"] = "value1";
    assocArray["key2"] = "value2";
    writeln("Assoc array key1: ", assocArray["key1"]);

    // Pointers (use with caution in modern D)
    int* ptr = &integerVar;
    *ptr = 15;
    writeln("Modified integerVar via pointer: ", integerVar);

    // Templates
    writeln("Max of 10 and 20: ", max(10, 20));
    writeln("Max of 10.5 and 20.25: ", max(10.5, 20.25));

    // Mixins
    mixin("writeln(\"Hello from mixin!\");");

    // Contract Programming
    testContract(10);

    // Unit Test (will be run automatically by D compiler)
    unittest {
        assert(add(1, 2) == 3);
        assert(factorial(0) == 1);
        assert(factorial(3) == 6);
    }

    // Concurrency (simplified example)
    import std.concurrency;
    auto f = spawn((int x) { return x * 2; }, 5);
    writeln("Fiber result: ", receive!int);

    // Metaprogramming with __traits
    static if (__traits(hasMember, Car, "start")) {
        writeln("Car has a start method.");
    }
}

// Function definition
int add(int a, int b)
{
    return a + b;
}

// Struct definition
struct Point
{
    int x;
    int y;
}

// Class definition
class Car
{
    string model;

    this(string model) {
        this.model = model;
    }

    void start() {
        writeln(model, " started.");
    }
}

// Function with contract programming
void testContract(int value)
in {
    assert(value > 0, "Value must be positive");
}
out (result) {
    assert(result == value * 2, "Result should be double the input");
}
body {
    writeln("Value in contract: ", value);
    return value * 2;
}

// Recursive function for unittest
long factorial(int n)
{
    if (n == 0) return 1;
    return n * factorial(n - 1);
}

// Template function
T max(T)(T a, T b) {
    return a > b ? a : b;
}
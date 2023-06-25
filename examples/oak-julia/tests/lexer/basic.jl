# Comprehensive Julia Lexer Test

module BasicJulia

using LinearAlgebra
using Statistics

# Abstract Types and Structs
abstract type Animal end

struct Dog <: Animal
    name::String
    breed::String
    age::Int
end

mutable struct Cat <: Animal
    name::String
    lives::Int
end

# Functions
function speak(d::Dog)
    println("Woof! I am $(d.name).")
end

function speak(c::Cat)
    println("Meow! I have $(c.lives) lives left.")
end

# Macros
macro sayhello(name)
    return :( println("Hello, ", $name) )
end

# Constants and Variables
const PI_VAL = 3.14159
global_var = 100

function main()
    # Numbers
    x = 42
    y = 3.14
    z = 1.2e-5
    c = 1 + 2im  # Complex number
    r = 1 // 2   # Rational number

    # Strings and Characters
    s = "Hello, Julia!"
    char = 'α'
    raw_str = raw"Raw string literal \n"
    cmd = `echo hello`

    # Arrays and Tuples
    arr = [1, 2, 3, 4, 5]
    mat = [1 2; 3 4]
    tuple = (1, "two", 3.0)
    
    # Ranges
    range = 1:10
    step_range = 1:2:10

    # Dictionaries
    dict = Dict("a" => 1, "b" => 2)

    # Control Flow
    if x > 10
        println("Big")
    elseif x == 10
        println("Ten")
    else
        println("Small")
    end

    for i in 1:5
        println(i)
    end

    while x > 0
        x -= 1
    end

    # List Comprehension
    squares = [i^2 for i in 1:10]

    # Broadcasting
    v = [1, 2, 3]
    v_squared = v .^ 2

    # Error Handling
    try
        sqrt(-1)
    catch e
        println("Error: ", e)
    finally
        println("Cleanup")
    end

    # Symbols
    sym = :symbol_name

    # Function call with keyword arguments
    plot(x, y; color="red", width=2)
    
    # Call macro
    @sayhello("World")
end

# One-line function
f(x) = x^2 + 2x + 1

end # module BasicJulia

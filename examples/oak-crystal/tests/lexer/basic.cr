# Crystal test file for lexer testing

# Class definition
class Person
  property name : String
  property age : Int32
  
  def initialize(@name : String, @age : Int32)
  end
  
  def greet
    puts "Hello, I'm #{@name} and I'm #{@age} years old."
  end
end

# Module definition
module MathUtils
  def self.factorial(n : Int32) : Int64
    return 1 if n <= 1
    n * factorial(n - 1)
  end
  
  def self.fibonacci(n : Int32) : Int64
    return n if n <= 1
    fibonacci(n - 1) + fibonacci(n - 2)
  end
end

# Struct definition
struct Point
  getter x : Float64
  getter y : Float64
  
  def initialize(@x : Float64, @y : Float64)
  end
  
  def distance_to(other : Point) : Float64
    Math.sqrt((@x - other.x)**2 + (@y - other.y)**2)
  end
end

# Enum definition
enum Color
  Red
  Green
  Blue
  Yellow
end

# Method with block
[1, 2, 3, 4, 5].each do |number|
  puts "Number: #{number}"
end

# Hash and array literals
numbers = [1, 2, 3, 4, 5]
person_data = {"name" => "Alice", "age" => 30, "city" => "New York"}

# Proc definition
calculator = ->(x : Int32, y : Int32) { x + y }
result = calculator.call(10, 20)

# Macro definition
macro define_method(name, content)
  def {{name.id}}
    {{content}}
  end
end

define_method hello, puts "Hello from macro!"

# Case statement
case Color::Red
when Color::Red then puts "Red color"
when Color::Green then puts "Green color"
else puts "Other color"
end

# Exception handling
begin
  raise "Something went wrong"
rescue exception
  puts "Caught exception: #{exception.message}"
ensure
  puts "Cleanup code"
end

# Type alias
alias MyString = String
alias Number = Int32 | Float64

# Generic class
class Container(T)
  @value : T
  
  def initialize(@value : T)
  end
  
  def get : T
    @value
  end
end

# Usage
int_container = Container(Int32).new(42)
string_container = Container(String).new("Hello")
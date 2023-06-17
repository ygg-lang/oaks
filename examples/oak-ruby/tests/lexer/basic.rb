# Ruby Test File - Comprehensive Syntax Coverage
# This file tests various Ruby syntax elements for lexer testing

# Encoding declaration
# encoding: utf-8

# Shebang line
#!/usr/bin/env ruby

# Comments
# Single line comment

=begin
Multi-line comment
can span multiple lines
and supports documentation
=end

# Constants and variables
CONSTANT_VALUE = 42
$global_variable = "global"
@@class_variable = "class"
@instance_variable = "instance"
local_variable = "local"

# Numbers and literals
integer = 123
negative = -456
hex_number = 0xFF
octal_number = 0755
binary_number = 0b1010
float_number = 3.14159
scientific = 1.23e-4
rational = 3/4r
complex = 3+4i

# Strings and string interpolation
single_quoted = 'Hello, World!'
double_quoted = "Hello, #{local_variable}!"
heredoc = <<EOF
This is a heredoc string
that can span multiple lines
and preserve formatting
EOF

heredoc_indented = <<~TEXT
  This is an indented heredoc
  that removes leading whitespace
  based on the least indented line
TEXT

# String literals with different delimiters
percent_string = %q{This is a string}
percent_interpolated = %Q{This has #{local_variable} interpolation}
word_array = %w[one two three four]
symbol_array = %i[red green blue yellow]

# Regular expressions
simple_regex = /hello/
regex_with_flags = /hello/i
regex_with_interpolation = /hello #{local_variable}/
percent_regex = %r{hello/world}

# Symbols
symbol = :symbol
string_symbol = :"string symbol"
interpolated_symbol = :"interpolated #{local_variable}"

# Arrays
empty_array = []
number_array = [1, 2, 3, 4, 5]
mixed_array = [1, "two", :three, 4.0]
nested_array = [[1, 2], [3, 4], [5, 6]]

# Array operations
array = [1, 2, 3]
array << 4
array.push(5)
array.unshift(0)
first_element = array.first
last_element = array.last
array_length = array.length

# Hashes
empty_hash = {}
string_keys = {"name" => "John", "age" => 30}
symbol_keys = {name: "Jane", age: 25}
mixed_hash = {"string" => 1, :symbol => 2, 3 => "number"}

# Hash operations
hash = {a: 1, b: 2, c: 3}
hash[:d] = 4
hash.merge!({e: 5})
keys = hash.keys
values = hash.values

# Ranges
inclusive_range = 1..10
exclusive_range = 1...10
string_range = "a".."z"
reverse_range = 10.downto(1)

# Control structures - if/elsif/else
age = 25
if age < 18
  puts "Minor"
elsif age < 65
  puts "Adult"
else
  puts "Senior"
end

# Ternary operator
status = age >= 18 ? "adult" : "minor"

# Unless statement
unless age < 18
  puts "Not a minor"
end

# Case/when statement
grade = 'B'
case grade
when 'A'
  puts "Excellent"
when 'B', 'C'
  puts "Good"
when 'D'
  puts "Passing"
when 'F'
  puts "Failing"
else
  puts "Invalid grade"
end

# Case with ranges and regex
score = 85
case score
when 90..100
  puts "A grade"
when 80...90
  puts "B grade"
when 70...80
  puts "C grade"
when /^[0-6]\d$/
  puts "Failing"
else
  puts "Invalid score"
end

# Loops - while
counter = 0
while counter < 5
  puts "Counter: #{counter}"
  counter += 1
end

# Until loop
countdown = 5
until countdown == 0
  puts "Countdown: #{countdown}"
  countdown -= 1
end

# For loop
for i in 1..5
  puts "For loop: #{i}"
end

# Times loop
5.times do |i|
  puts "Times loop: #{i}"
end

# Each loop
[1, 2, 3, 4, 5].each do |num|
  puts "Each: #{num}"
end

# Each with index
%w[apple banana cherry].each_with_index do |fruit, index|
  puts "#{index}: #{fruit}"
end

# Loop control
(1..10).each do |i|
  next if i.even?
  break if i > 7
  puts "Odd number: #{i}"
end

# Method definitions
def simple_method
  puts "Hello from simple method"
end

def method_with_params(name, age = 18)
  puts "Name: #{name}, Age: #{age}"
end

def method_with_splat(*args)
  puts "Arguments: #{args.join(', ')}"
end

def method_with_keyword_args(name:, age: 18, **options)
  puts "Name: #{name}, Age: #{age}, Options: #{options}"
end

def method_with_block
  yield if block_given?
end

def method_with_block_param(&block)
  block.call if block
end

# Method with return value
def add(a, b)
  a + b  # implicit return
end

def subtract(a, b)
  return a - b  # explicit return
end

# Method with multiple return values
def divide_with_remainder(dividend, divisor)
  quotient = dividend / divisor
  remainder = dividend % divisor
  [quotient, remainder]
end

# Class definitions
class Person
  # Class variables
  @@population = 0
  
  # Constants
  SPECIES = "Homo sapiens"
  
  # Attribute accessors
  attr_reader :name
  attr_writer :age
  attr_accessor :email
  
  # Constructor
  def initialize(name, age = 0)
    @name = name
    @age = age
    @@population += 1
  end
  
  # Instance methods
  def greet
    "Hello, I'm #{@name} and I'm #{@age} years old."
  end
  
  def birthday
    @age += 1
  end
  
  # Class methods
  def self.population
    @@population
  end
  
  def self.create_anonymous
    new("Anonymous")
  end
  
  # Private methods
  private
  
  def secret_method
    "This is private"
  end
  
  # Protected methods
  protected
  
  def protected_method
    "This is protected"
  end
end

# Inheritance
class Employee < Person
  attr_accessor :salary, :department
  
  def initialize(name, age, salary, department)
    super(name, age)  # Call parent constructor
    @salary = salary
    @department = department
  end
  
  def greet
    super + " I work in #{@department}."
  end
  
  def annual_salary
    @salary * 12
  end
end

# Modules
module Greetings
  def say_hello
    puts "Hello!"
  end
  
  def say_goodbye
    puts "Goodbye!"
  end
  
  module_function :say_hello
end

module Comparable
  def <=>(other)
    # Implementation for comparison
  end
end

# Module inclusion
class Student < Person
  include Greetings
  include Comparable
  
  attr_accessor :grade
  
  def initialize(name, age, grade)
    super(name, age)
    @grade = grade
  end
end

# Singleton methods
person = Person.new("John", 30)
def person.special_ability
  "I have a special ability!"
end

# Class extension
class Person
  def full_info
    "#{greet} My email is #{@email}."
  end
end

# Blocks and Procs
# Block examples
[1, 2, 3, 4, 5].select { |n| n.even? }
[1, 2, 3, 4, 5].map { |n| n * 2 }
[1, 2, 3, 4, 5].reduce(0) { |sum, n| sum + n }

# Multi-line blocks
numbers = [1, 2, 3, 4, 5]
squared = numbers.map do |n|
  result = n * n
  puts "#{n} squared is #{result}"
  result
end

# Proc objects
square_proc = Proc.new { |x| x * x }
cube_proc = proc { |x| x * x * x }
double_lambda = lambda { |x| x * 2 }
triple_lambda = ->(x) { x * 3 }

# Using procs
puts square_proc.call(5)
puts cube_proc[4]
puts double_lambda.(3)
puts triple_lambda.call(2)

# Exception handling
begin
  # Code that might raise an exception
  result = 10 / 0
rescue ZeroDivisionError => e
  puts "Cannot divide by zero: #{e.message}"
rescue StandardError => e
  puts "An error occurred: #{e.message}"
else
  puts "No exception occurred"
ensure
  puts "This always executes"
end

# Custom exceptions
class CustomError < StandardError
  def initialize(message = "A custom error occurred")
    super(message)
  end
end

# Raising exceptions
def risky_method(value)
  raise CustomError, "Value cannot be negative" if value < 0
  raise ArgumentError, "Value must be a number" unless value.is_a?(Numeric)
  value * 2
end

# File operations
begin
  File.open("test.txt", "w") do |file|
    file.write("Hello, World!")
  end
  
  content = File.read("test.txt")
  puts content
rescue IOError => e
  puts "File operation failed: #{e.message}"
end

# Metaprogramming
class DynamicClass
  # Define methods dynamically
  %w[red green blue].each do |color|
    define_method("#{color}?") do
      @color == color
    end
    
    define_method("make_#{color}") do
      @color = color
    end
  end
  
  # Method missing
  def method_missing(method_name, *args, &block)
    if method_name.to_s.start_with?("find_by_")
      attribute = method_name.to_s.sub("find_by_", "")
      puts "Finding by #{attribute} with value #{args.first}"
    else
      super
    end
  end
  
  def respond_to_missing?(method_name, include_private = false)
    method_name.to_s.start_with?("find_by_") || super
  end
end

# Class variables and instance variables
class Counter
  @@total_count = 0
  
  def initialize
    @count = 0
    @@total_count += 1
  end
  
  def increment
    @count += 1
  end
  
  def self.total_instances
    @@total_count
  end
  
  def count
    @count
  end
end

# Eigenclass (singleton class)
obj = Object.new
class << obj
  def singleton_method
    "I'm a singleton method"
  end
end

# Constants and namespacing
module MyModule
  CONSTANT = "module constant"
  
  class MyClass
    CONSTANT = "class constant"
    
    def show_constants
      puts CONSTANT
      puts MyModule::CONSTANT
      puts ::CONSTANT if defined?(::CONSTANT)
    end
  end
end

# Operators and operator overloading
class Vector
  attr_reader :x, :y
  
  def initialize(x, y)
    @x, @y = x, y
  end
  
  def +(other)
    Vector.new(@x + other.x, @y + other.y)
  end
  
  def -(other)
    Vector.new(@x - other.x, @y - other.y)
  end
  
  def *(scalar)
    Vector.new(@x * scalar, @y * scalar)
  end
  
  def ==(other)
    @x == other.x && @y == other.y
  end
  
  def to_s
    "(#{@x}, #{@y})"
  end
end

# Enumerable module
class TodoList
  include Enumerable
  
  def initialize
    @items = []
  end
  
  def add(item)
    @items << item
  end
  
  def each
    @items.each { |item| yield(item) }
  end
end

# Struct
Person = Struct.new(:name, :age) do
  def adult?
    age >= 18
  end
end

# OpenStruct
require 'ostruct'
person = OpenStruct.new(name: "John", age: 30)

# Regular expressions with named captures
text = "John Doe, age 30"
if match = text.match(/(?<name>\w+ \w+), age (?<age>\d+)/)
  puts "Name: #{match[:name]}, Age: #{match[:age]}"
end

# String methods and manipulation
string = "  Hello, World!  "
puts string.strip
puts string.upcase
puts string.downcase
puts string.capitalize
puts string.reverse
puts string.length
puts string.include?("World")
puts string.gsub("World", "Ruby")
puts string.split(",")

# Array methods
array = [1, 2, 3, 4, 5]
puts array.first(3)
puts array.last(2)
puts array.sample
puts array.shuffle
puts array.sort
puts array.reverse
puts array.uniq
puts array.compact  # removes nil values

# Hash methods
hash = {a: 1, b: 2, c: 3}
puts hash.keys
puts hash.values
puts hash.has_key?(:a)
puts hash.has_value?(2)
puts hash.invert

# Numeric methods
number = 42
puts number.even?
puts number.odd?
puts number.abs
puts number.round
puts number.ceil
puts number.floor

# Time and Date
require 'time'
require 'date'

now = Time.now
puts now.strftime("%Y-%m-%d %H:%M:%S")
puts now.year
puts now.month
puts now.day

today = Date.today
puts today.strftime("%A, %B %d, %Y")

# Threads
thread = Thread.new do
  5.times do |i|
    puts "Thread: #{i}"
    sleep(0.1)
  end
end

thread.join

# Fibers
fiber = Fiber.new do
  puts "Fiber started"
  Fiber.yield "First yield"
  puts "Fiber resumed"
  Fiber.yield "Second yield"
  puts "Fiber finished"
  "Final value"
end

puts fiber.resume
puts fiber.resume
puts fiber.resume

# Constants and freeze
FROZEN_ARRAY = [1, 2, 3].freeze
FROZEN_HASH = {a: 1, b: 2}.freeze

# Method visibility
class VisibilityExample
  def public_method
    "This is public"
  end
  
  private
  
  def private_method
    "This is private"
  end
  
  protected
  
  def protected_method
    "This is protected"
  end
  
  public
  
  def another_public_method
    "This is also public"
  end
end

# Refinements
module StringExtensions
  refine String do
    def palindrome?
      self == self.reverse
    end
  end
end

class PalindromeChecker
  using StringExtensions
  
  def check(string)
    string.palindrome?
  end
end

# Keyword arguments with required keywords
def method_with_required_keywords(name:, age:, city: "Unknown")
  puts "Name: #{name}, Age: #{age}, City: #{city}"
end

# Pattern matching (Ruby 2.7+)
case [1, 2, 3]
in [1, 2, 3]
  puts "Exact match"
in [1, *, 3]
  puts "First and last match"
in [*, 3]
  puts "Last element is 3"
else
  puts "No match"
end

# Endless method definitions (Ruby 3.0+)
def square(x) = x * x
def greet(name) = "Hello, #{name}!"

# Multiple assignment
a, b, c = 1, 2, 3
x, *rest = [1, 2, 3, 4, 5]
first, *middle, last = [1, 2, 3, 4, 5]

# Parallel assignment
a, b = b, a  # swap values

# Conditional assignment
@name ||= "Default Name"
@count &&= @count + 1

# Safe navigation operator
user = nil
puts user&.name&.upcase

# Frozen string literals
# frozen_string_literal: true

# Method chaining
result = [1, 2, 3, 4, 5]
  .select(&:even?)
  .map(&:to_s)
  .join(", ")

puts result

# Symbol to proc
numbers = [1, 2, 3, 4, 5]
strings = numbers.map(&:to_s)
doubled = numbers.map(&2.method(:*))

# Splat operators
def method_with_args(a, b, c)
  puts "#{a}, #{b}, #{c}"
end

args = [1, 2, 3]
method_with_args(*args)

# Double splat for hashes
def method_with_kwargs(name:, age:, **options)
  puts "Name: #{name}, Age: #{age}, Options: #{options}"
end

hash = {name: "John", age: 30, city: "NYC", country: "USA"}
method_with_kwargs(**hash)

# END block
END {
  puts "This runs at the end of the program"
}

# BEGIN block
BEGIN {
  puts "This runs at the beginning of the program"
}

puts "Main program execution"
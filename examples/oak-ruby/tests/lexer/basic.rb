#!/usr/bin/env ruby
# Comprehensive Ruby Lexer Test

=begin
    Multi-line comment
    Testing various Ruby features
=end

require 'date'

# Constants and Global Variables
PI = 3.14159
$global_var = "I am global"

# Module definition
module MathHelper
    def self.square(x)
        x * x
    end
end

# Class definition with inheritance
class Animal
    attr_accessor :name
    
    def initialize(name)
        @name = name
    end
    
    def speak
        raise NotImplementedError, "Subclasses must implement speak"
    end
end

class Dog < Animal
    def speak
        "Woof!"
    end
    
    def self.info
        puts "I am a dog class"
    end
end

# Variables and Types
integer = 42
float = 3.14
string_dq = "Double quoted with interpolation: #{integer}"
string_sq = 'Single quoted string'
symbol = :my_symbol
array = [1, 2, "three", :four]
hash = { :name => "Alice", "age" => 30, city: "New York" }
range = 1..10
regex = /^[a-z0-9]+$/i

# Control Structures
if integer > 10
    puts "Greater than 10"
elsif integer == 10
    puts "Equal to 10"
elsif integer < 0
    puts "Negative"
else
    puts "Less than 10"
end

unless integer < 0
    puts "Not negative"
end

case integer
when 1..10
    puts "Small"
when 42
    puts "The Answer"
else
    puts "Other"
end

# Loops and Iterators
5.times do |i|
    puts "Iteration #{i}"
end

array.each { |element| puts element }

while integer > 40
    integer -= 1
end

# Methods and Blocks
def greet(name, greeting = "Hello", *args, &block)
    puts "#{greeting}, #{name}"
    yield if block_given?
end

greet("Bob") { puts "Block executed" }

# Exception Handling
begin
    result = 10 / 0
rescue ZeroDivisionError => e
    puts "Error: #{e.message}"
ensure
    puts "Cleanup"
end

# Procs and Lambdas
my_proc = Proc.new { |x| puts x }
my_lambda = ->(x) { puts x * 2 }

my_proc.call(5)
my_lambda.call(5)

# Metaprogramming
define_method(:dynamic_method) do
    puts "Dynamic!"
end

__END__
Data after the script

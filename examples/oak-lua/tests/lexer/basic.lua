-- Oak Lua Test Suite
-- Basic Lua syntax test file for Oak lexer

-- Variables and basic data types
local name = "Oak Lua"
local version = 5.4
local is_active = true
local data = nil
local count = 42
local pi = 3.14159
local hex_value = 0xFF
local scientific = 1.23e-4

-- Comments
-- This is a single line comment
local x = 10 -- inline comment

--[[
    This is a multi-line comment
    that spans multiple lines
    and can contain various content
]]

local y = 20 --[[inline multi-line comment]]

-- Functions
function greet(name)
    return "Hello, " .. name .. "!"
end

local function add(a, b)
    return a + b
end

local multiply = function(x, y)
    return x * y
end

-- Control structures
if count > 0 then
    print("Count is positive")
elseif count < 0 then
    print("Count is negative")
else
    print("Count is zero")
end

-- Loops
for i = 1, 10 do
    print("Iteration: " .. i)
end

local i = 1
while i <= 5 do
    print("While loop: " .. i)
    i = i + 1
end

repeat
    print("Repeat-until loop")
    i = i - 1
until i <= 0

-- Tables (Lua's primary data structure)
local person = {
    name = "John Doe",
    age = 30,
    city = "New York",
    hobbies = {"reading", "gaming", "hiking"}
}

local array = {10, 20, 30, 40, 50}
local mixed = {1, "two", 3.0, true, nil, {nested = "table"}}

-- Table operations
local matrix = {
    {1, 2, 3},
    {4, 5, 6},
    {7, 8, 9}
}

-- String operations
local message = "Hello World"
local upper = string.upper(message)
local lower = string.lower(message)
local length = #message
local substring = string.sub(message, 1, 5)

-- Operators
local sum = 10 + 20
local difference = 30 - 15
local product = 5 * 6
local quotient = 20 / 4
local modulo = 17 % 5
local power = 2 ^ 8

-- Comparison operators
local equal = (5 == 5)
local not_equal = (5 ~= 6)
local less_than = (3 < 7)
local greater_than = (8 > 2)
local less_equal = (4 <= 4)
local greater_equal = (9 >= 9)

-- Logical operators
local and_result = true and false
local or_result = true or false
local not_result = not true

-- String concatenation
local first_name = "John"
local last_name = "Doe"
local full_name = first_name .. " " .. last_name

-- Variable scoping
local global_var = "I'm global"

do
    local local_var = "I'm local"
    print(local_var) -- Accessible here
end
-- local_var not accessible here

-- Metatables and metamethods
local mt = {
    __add = function(a, b)
        return a.value + b.value
    end,
    __tostring = function(t)
        return "CustomObject: " .. tostring(t.value)
    end
}

local obj1 = {value = 10}
local obj2 = {value = 20}
setmetatable(obj1, mt)
setmetatable(obj2, mt)

local result = obj1 + obj2
print(result) -- Calls __add metamethod

-- Error handling
local success, error_msg = pcall(function()
    error("This is an intentional error")
end)

if not success then
    print("Error caught: " .. error_msg)
end

-- Coroutines
local co = coroutine.create(function(a, b)
    print("Coroutine started with:", a, b)
    local yield_value = coroutine.yield(a + b)
    print("Coroutine resumed with:", yield_value)
    return a * b
end)

print("Coroutine status:", coroutine.status(co))
local ok, result = coroutine.resume(co, 10, 20)
print("First resume result:", result)

-- Module pattern
local M = {}

function M.add(a, b)
    return a + b
end

function M.multiply(a, b)
    return a * b
end

return M

-- Advanced features
local unpack = table.unpack or unpack

local function varargs(...)
    local args = {...}
    print("Received", #args, "arguments")
    for i, arg in ipairs(args) do
        print(i, arg)
    end
end

varargs(1, 2, 3, "four", 5.0)

-- Tail call optimization
local function factorial(n, acc)
    acc = acc or 1
    if n <= 1 then
        return acc
    end
    return factorial(n - 1, n * acc) -- Tail call
end

print("5! =", factorial(5))
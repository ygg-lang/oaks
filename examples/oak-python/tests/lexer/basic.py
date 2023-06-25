#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Comprehensive Python Lexer Test File
Covers Python 3.10+ syntax features
"""

import math
import sys
from typing import List, Optional, Union, Dict, Any
from dataclasses import dataclass
import asyncio

# Variables and Primitive Types
integer_val: int = 42
float_val: float = 3.14159
scientific_notation = 1.2e-3
hex_val = 0xDEADBEEF
oct_val = 0o755
bin_val = 0b101010
complex_val = 1 + 2j
string_val: str = "Hello, World!"
f_string = f"Value: {integer_val}"
raw_string = r"Raw\nString"
bytes_val = b"bytes"
boolean_val: bool = True
none_val: None = None

# Collections
my_list: List[int] = [1, 2, 3, 4, 5]
my_tuple = (1, "two", 3.0)
my_set = {1, 2, 3}
my_dict: Dict[str, int] = {"a": 1, "b": 2}

# Operators
a = 10
b = 3
sum_val = a + b
diff = a - b
prod = a * b
div = a / b
floor_div = a // b
mod = a % b
power = a ** b

# Bitwise
bitwise_and = a & b
bitwise_or = a | b
bitwise_xor = a ^ b
bitwise_not = ~a
left_shift = a << 1
right_shift = a >> 1

# Comparison & Logical
is_equal = (a == b)
is_not_equal = (a != b)
is_greater = (a > b)
logic = (a > 5) and (b < 10) or not False

# Walrus operator
if (n := len(my_list)) > 10:
    print(f"List is too long ({n} elements)")

# Control Flow
x = 10
if x < 0:
    print("Negative")
elif x == 0:
    print("Zero")
else:
    print("Positive")

for i in range(5):
    if i == 2:
        continue
    if i == 4:
        break
    print(i)
else:
    print("Loop finished")

while x > 0:
    x -= 1

# Match Statement (Python 3.10+)
command = "quit"
match command:
    case "start":
        print("Starting")
    case "stop" | "quit":
        print("Stopping")
    case _:
        print("Unknown command")

# Functions and Decorators
def my_decorator(func):
    def wrapper():
        print("Before")
        func()
        print("After")
    return wrapper

@my_decorator
def say_hello(name: str = "World", *args, **kwargs) -> str:
    """Docstring for say_hello"""
    return f"Hello, {name}"

lambda_func = lambda x, y: x + y

# Classes
class Animal:
    species = "Generic"

    def __init__(self, name: str):
        self._name = name
    
    def speak(self):
        raise NotImplementedError

class Dog(Animal):
    def speak(self):
        return "Woof!"
    
    @property
    def name(self):
        return self._name

# Exception Handling
try:
    result = 10 / 0
except ZeroDivisionError as e:
    print(f"Error: {e}")
except (TypeError, ValueError):
    print("Type or Value error")
else:
    print("Success")
finally:
    print("Cleanup")

# Context Manager
with open("test.txt", "w") as f:
    f.write("Hello")

# Comprehensions
squares = [x**2 for x in range(10) if x % 2 == 0]
square_map = {x: x**2 for x in range(5)}
unique_squares = {x**2 for x in range(10)}
gen_exp = (x**2 for x in range(10))

# Async/Await
async def fetch_data():
    await asyncio.sleep(1)
    return "Data"

async def main():
    data = await fetch_data()
    print(data)

if __name__ == "__main__":
    asyncio.run(main())

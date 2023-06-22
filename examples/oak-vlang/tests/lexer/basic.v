// V language test file
module main

import os

fn main() {
    println('Hello, V!')
    
    // Variables
    name := 'World'
    age := 25
    
    // Arrays
    numbers := [1, 2, 3, 4, 5]
    
    // Functions
    result := add(10, 20)
    println('Result: $result')
    
    // Structs
    user := User{
        name: 'Alice'
        age: 30
    }
    
    println('User: $user.name, Age: $user.age')
}

fn add(a int, b int) int {
    return a + b
}

struct User {
    name string
    age  int
}
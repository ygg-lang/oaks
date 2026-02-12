// Comprehensive V Language Test
module main

import os
import time

// Constants
const (
    pi    = 3.14159
    world = 'World'
)

// Enums
enum Color {
    red
    green
    blue
}

// Structs with attributes
[heap]
struct User {
pub mut:
    name string [required]
    age  int
    role string = 'user'
    tags []string
}

// Interface
interface Speaker {
    speak() string
}

// Methods
fn (u User) speak() string {
    return 'Hello, my name is $u.name'
}

// Generics
fn compare<T>(a T, b T) int {
    if a < b {
        return -1
    }
    if a > b {
        return 1
    }
    return 0
}

// Main function
fn main() {
    println('Hello, V!')
    
    // Variables
    mut x := 10
    x += 5
    
    // Arrays
    nums := [1, 2, 3, 4]
    filtered := nums.filter(it > 2)
    
    // Maps
    mut m := map[string]int{}
    m['one'] = 1
    
    // Control Flow
    if x > 10 {
        println('Greater than 10')
    } else {
        println('Less or equal')
    }
    
    // Match
    os_name := 'linux'
    match os_name {
        'linux' { println('Linux') }
        'windows' { println('Windows') }
        else { println('Other') }
    }
    
    // Loops
    for i in 0..5 {
        println(i)
    }
    
    names := ['a', 'b', 'c']
    for i, name in names {
        println('$i: $name')
    }
    
    // Error Handling
    res := might_fail() or {
        eprintln(err)
        return
    }
    
    // Concurrency
    ch := chan int{}
    go fn(c chan int) {
        c <- 100
    }(ch)
    
    val := <-ch
    println('Got $val')
    
    // SQL (V feature)
    // sql db {
    //     select from User where age > 20
    // }
    
    // C interop
    // C.printf(c'Hello from C\n')
}

fn might_fail() !int {
    return error('Something went wrong')
}

// Attributes
[inline]
fn fast_add(a int, b int) int {
    return a + b
}

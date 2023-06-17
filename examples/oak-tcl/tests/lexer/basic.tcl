#!/usr/bin/env tclsh
# TCL test file for lexer testing

# Basic variables and data types
set name "John Doe"
set age 30
set height 5.9
set is_student true
set empty_var ""

# Lists
set fruits {apple banana cherry date elderberry}
set numbers {1 2 3 4 5}
set mixed_list {hello 42 3.14 world}
set nested_list {{a b c} {1 2 3} {x y z}}

# Arrays (associative arrays)
array set person {
    name "Alice Smith"
    age 25
    city "New York"
    occupation "Engineer"
}

array set colors {
    red "#FF0000"
    green "#00FF00"
    blue "#0000FF"
    yellow "#FFFF00"
}

# Dictionary (Tcl 8.5+)
set config [dict create \
    host "localhost" \
    port 8080 \
    timeout 30 \
    ssl_enabled true \
    max_connections 100]

# Procedures (functions)
proc greet {name} {
    return "Hello, $name!"
}

proc add {a b} {
    return [expr {$a + $b}]
}

proc multiply {a b} {
    return [expr {$a * $b}]
}

proc factorial {n} {
    if {$n <= 1} {
        return 1
    } else {
        return [expr {$n * [factorial [expr {$n - 1}]]}]
    }
}

proc fibonacci {n} {
    if {$n <= 1} {
        return $n
    } else {
        return [expr {[fibonacci [expr {$n - 1}]] + [fibonacci [expr {$n - 2}]]}]
    }
}

proc sum_list {lst} {
    set total 0
    foreach item $lst {
        set total [expr {$total + $item}]
    }
    return $total
}

proc find_max {lst} {
    set max [lindex $lst 0]
    foreach item $lst {
        if {$item > $max} {
            set max $item
        }
    }
    return $max
}

proc is_prime {n} {
    if {$n < 2} {
        return 0
    }
    if {$n == 2} {
        return 1
    }
    if {$n % 2 == 0} {
        return 0
    }
    for {set i 3} {$i * $i <= $n} {incr i 2} {
        if {$n % $i == 0} {
            return 0
        }
    }
    return 1
}

proc reverse_string {str} {
    set result ""
    for {set i [expr {[string length $str] - 1}]} {$i >= 0} {incr i -1} {
        append result [string index $str $i]
    }
    return $result
}

proc count_words {text} {
    return [llength [split $text]]
}

# Procedure with variable arguments
proc print_all {args} {
    foreach arg $args {
        puts $arg
    }
}

# Procedure with optional arguments
proc connect_db {host {port 5432} {database "mydb"}} {
    puts "Connecting to $host:$port/$database"
}

# Procedure with upvar (reference parameters)
proc increment {varname} {
    upvar $varname var
    incr var
}

# Procedure with global variables
set global_counter 0

proc increment_global {} {
    global global_counter
    incr global_counter
}

# Control structures
# If-else statements
set score 85
if {$score >= 90} {
    set grade "A"
} elseif {$score >= 80} {
    set grade "B"
} elseif {$score >= 70} {
    set grade "C"
} elseif {$score >= 60} {
    set grade "D"
} else {
    set grade "F"
}

# Switch statement
set day "Monday"
switch $day {
    "Monday" {
        set mood "Tired"
    }
    "Tuesday" {
        set mood "Getting better"
    }
    "Wednesday" {
        set mood "Hump day"
    }
    "Thursday" {
        set mood "Almost there"
    }
    "Friday" {
        set mood "TGIF!"
    }
    "Saturday" -
    "Sunday" {
        set mood "Weekend!"
    }
    default {
        set mood "Unknown day"
    }
}

# For loops
puts "Counting from 1 to 10:"
for {set i 1} {$i <= 10} {incr i} {
    puts $i
}

puts "Counting down from 10 to 1:"
for {set i 10} {$i >= 1} {incr i -1} {
    puts $i
}

# While loop
set count 0
while {$count < 5} {
    puts "Count: $count"
    incr count
}

# Foreach loop
puts "Fruits:"
foreach fruit $fruits {
    puts "- $fruit"
}

puts "Numbers and their squares:"
foreach num $numbers {
    puts "$num squared is [expr {$num * $num}]"
}

# Foreach with multiple variables
set pairs {{name Alice} {age 25} {city "New York"}}
foreach {key value} $pairs {
    puts "$key: $value"
}

# Array iteration
puts "Person information:"
foreach {key value} [array get person] {
    puts "$key: $value"
}

# String operations
set text "Hello, World!"
puts "Original: $text"
puts "Length: [string length $text]"
puts "Uppercase: [string toupper $text]"
puts "Lowercase: [string tolower $text]"
puts "First 5 chars: [string range $text 0 4]"
puts "Last 6 chars: [string range $text end-5 end]"
puts "Index of 'World': [string first "World" $text]"
puts "Replace 'World' with 'TCL': [string map {"World" "TCL"} $text]"

# Regular expressions
set email "user@example.com"
if {[regexp {^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$} $email]} {
    puts "$email is a valid email address"
} else {
    puts "$email is not a valid email address"
}

# Extract parts using regexp
set phone "123-456-7890"
if {[regexp {(\d{3})-(\d{3})-(\d{4})} $phone match area exchange number]} {
    puts "Area code: $area"
    puts "Exchange: $exchange"
    puts "Number: $number"
}

# List operations
set original_list {1 2 3 4 5}
puts "Original list: $original_list"
puts "Length: [llength $original_list]"
puts "First element: [lindex $original_list 0]"
puts "Last element: [lindex $original_list end]"
puts "Range 1-3: [lrange $original_list 1 3]"

# Append to list
lappend original_list 6 7 8
puts "After append: $original_list"

# Insert into list
set original_list [linsert $original_list 0 0]
puts "After insert at beginning: $original_list"

# Replace in list
set original_list [lreplace $original_list 2 2 "THREE"]
puts "After replace: $original_list"

# Sort list
set unsorted {banana apple cherry date}
set sorted [lsort $unsorted]
puts "Unsorted: $unsorted"
puts "Sorted: $sorted"

# Sort numerically
set numbers_unsorted {10 2 30 4 5}
set numbers_sorted [lsort -integer $numbers_unsorted]
puts "Numbers unsorted: $numbers_unsorted"
puts "Numbers sorted: $numbers_sorted"

# Dictionary operations (Tcl 8.5+)
puts "Configuration:"
dict for {key value} $config {
    puts "$key: $value"
}

# Add to dictionary
dict set config debug_mode true
puts "After adding debug_mode: [dict get $config debug_mode]"

# Check if key exists
if {[dict exists $config ssl_enabled]} {
    puts "SSL is [expr {[dict get $config ssl_enabled] ? "enabled" : "disabled"}]"
}

# File operations
set filename "test.txt"

# Write to file
set file [open $filename w]
puts $file "This is a test file"
puts $file "Line 2"
puts $file "Line 3"
close $file

# Read from file
if {[file exists $filename]} {
    set file [open $filename r]
    set content [read $file]
    close $file
    puts "File content:"
    puts $content
}

# Read file line by line
if {[file exists $filename]} {
    set file [open $filename r]
    set line_number 1
    while {[gets $file line] >= 0} {
        puts "Line $line_number: $line"
        incr line_number
    }
    close $file
}

# File information
if {[file exists $filename]} {
    puts "File size: [file size $filename] bytes"
    puts "File type: [file type $filename]"
    puts "File readable: [file readable $filename]"
    puts "File writable: [file writable $filename]"
    puts "File executable: [file executable $filename]"
}

# Delete file
if {[file exists $filename]} {
    file delete $filename
    puts "File deleted"
}

# Error handling
proc safe_divide {a b} {
    if {[catch {expr {$a / $b}} result]} {
        return "Error: Division by zero or invalid input"
    } else {
        return $result
    }
}

puts "10 / 2 = [safe_divide 10 2]"
puts "10 / 0 = [safe_divide 10 0]"

# Try-catch equivalent
if {[catch {
    set result [expr {10 / 0}]
    puts "Result: $result"
} error_msg]} {
    puts "Caught error: $error_msg"
}

# Namespace example
namespace eval ::math {
    variable pi 3.14159
    
    proc circle_area {radius} {
        variable pi
        return [expr {$pi * $radius * $radius}]
    }
    
    proc circle_circumference {radius} {
        variable pi
        return [expr {2 * $pi * $radius}]
    }
}

puts "Circle area (r=5): [::math::circle_area 5]"
puts "Circle circumference (r=5): [::math::circle_circumference 5]"

# Object-oriented programming with TclOO (Tcl 8.6+)
if {[info commands oo::class] ne ""} {
    oo::class create Person {
        variable name age
        
        constructor {person_name person_age} {
            set name $person_name
            set age $person_age
        }
        
        method get_name {} {
            return $name
        }
        
        method get_age {} {
            return $age
        }
        
        method set_age {new_age} {
            set age $new_age
        }
        
        method introduce {} {
            return "Hi, I'm $name and I'm $age years old."
        }
        
        method birthday {} {
            incr age
            return "Happy birthday! I'm now $age years old."
        }
    }
    
    # Create instances
    set person1 [Person new "Alice" 25]
    set person2 [Person new "Bob" 30]
    
    puts [$person1 introduce]
    puts [$person2 introduce]
    
    puts [$person1 birthday]
    puts "Alice's new age: [$person1 get_age]"
    
    # Cleanup
    $person1 destroy
    $person2 destroy
}

# Advanced list processing
proc map {func list} {
    set result {}
    foreach item $list {
        lappend result [$func $item]
    }
    return $result
}

proc filter {predicate list} {
    set result {}
    foreach item $list {
        if {[$predicate $item]} {
            lappend result $item
        }
    }
    return $result
}

proc square {x} {
    return [expr {$x * $x}]
}

proc is_even {x} {
    return [expr {$x % 2 == 0}]
}

set test_numbers {1 2 3 4 5 6 7 8 9 10}
puts "Original numbers: $test_numbers"
puts "Squared: [map square $test_numbers]"
puts "Even numbers: [filter is_even $test_numbers]"

# Time and date
puts "Current time: [clock format [clock seconds]]"
puts "Current time (ISO): [clock format [clock seconds] -format "%Y-%m-%d %H:%M:%S"]"
puts "Timestamp: [clock seconds]"

# Parse date
set date_string "2023-12-25 15:30:00"
if {[catch {clock scan $date_string -format "%Y-%m-%d %H:%M:%S"} timestamp]} {
    puts "Failed to parse date"
} else {
    puts "Parsed timestamp: $timestamp"
    puts "Formatted: [clock format $timestamp]"
}

# Mathematical operations
puts "Mathematical operations:"
puts "2 + 3 = [expr {2 + 3}]"
puts "10 - 4 = [expr {10 - 4}]"
puts "6 * 7 = [expr {6 * 7}]"
puts "15 / 3 = [expr {15 / 3}]"
puts "17 % 5 = [expr {17 % 5}]"
puts "2^8 = [expr {2**8}]"
puts "sqrt(16) = [expr {sqrt(16)}]"
puts "sin(pi/2) = [expr {sin(3.14159/2)}]"
puts "cos(0) = [expr {cos(0)}]"
puts "log(10) = [expr {log(10)}]"
puts "exp(1) = [expr {exp(1)}]"
puts "abs(-5) = [expr {abs(-5)}]"
puts "round(3.7) = [expr {round(3.7)}]"
puts "floor(3.7) = [expr {floor(3.7)}]"
puts "ceil(3.2) = [expr {ceil(3.2)}]"

# Random numbers
puts "Random integer 1-100: [expr {int(rand() * 100) + 1}]"
puts "Random float 0-1: [expr {rand()}]"

# Command substitution and variable substitution
set command "date"
puts "Command output: [exec date]" ;# This might not work on all systems

# Variable substitution examples
set var1 "Hello"
set var2 "World"
puts "$var1, $var2!"
puts "${var1}, ${var2}!"

# Escape sequences
puts "Tab:\tTabbed text"
puts "Newline:\nNew line"
puts "Quote: \"Hello\""
puts "Backslash: \\"

# Comments
# This is a single-line comment

# Multi-line comment (using if 0)
if 0 {
    This is a multi-line comment
    that spans several lines
    and won't be executed
}

# Procedure with default arguments and variable arguments
proc flexible_proc {required {optional "default"} args} {
    puts "Required: $required"
    puts "Optional: $optional"
    puts "Additional args: $args"
}

flexible_proc "test"
flexible_proc "test" "custom"
flexible_proc "test" "custom" "extra1" "extra2"

# Lambda-like procedures (anonymous procedures)
set lambda [list {x} {return [expr {$x * 2}]}]
proc apply_lambda {lambda_proc arg} {
    return [apply $lambda_proc $arg]
}

puts "Lambda result: [apply_lambda $lambda 5]"

# Eval and subst
set code {puts "This code was evaluated"}
eval $code

set template "Hello, \$name! Today is [clock format [clock seconds] -format %A]"
set name "TCL User"
puts [subst $template]

puts "TCL lexer test completed successfully!"
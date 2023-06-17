;; WebAssembly Text Format (WAT) Test File - Comprehensive Syntax Coverage
;; This file tests various WAT syntax elements for lexer testing

;; Module declaration
(module
  ;; Import section
  (import "env" "memory" (memory 1))
  (import "env" "print" (func $print (param i32)))
  (import "env" "abort" (func $abort (param i32 i32 i32 i32)))
  (import "console" "log" (func $log (param i32)))
  
  ;; Type definitions
  (type $binary_op (func (param i32 i32) (result i32)))
  (type $unary_op (func (param i32) (result i32)))
  (type $void_func (func))
  (type $callback (func (param i32) (result i32)))
  
  ;; Memory declaration
  (memory $mem 2 4)
  
  ;; Table declaration
  (table $table 10 funcref)
  
  ;; Global variables
  (global $counter (mut i32) (i32.const 0))
  (global $pi f32 (f32.const 3.14159))
  (global $max_value i32 (i32.const 1000))
  (global $flag (mut i32) (i32.const 0))
  
  ;; Data section
  (data (i32.const 0) "Hello, WebAssembly!")
  (data (i32.const 32) "Testing WAT syntax")
  (data $string1 (i32.const 64) "Another string")
  
  ;; Element section
  (elem (i32.const 0) $add $subtract $multiply $divide)
  
  ;; Function declarations
  
  ;; Simple arithmetic functions
  (func $add (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    i32.add
  )
  
  (func $subtract (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    i32.sub
  )
  
  (func $multiply (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    i32.mul
  )
  
  (func $divide (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    i32.div_s
  )
  
  ;; Function with local variables
  (func $factorial (param $n i32) (result i32)
    (local $result i32)
    (local $i i32)
    
    ;; Initialize result to 1
    i32.const 1
    local.set $result
    
    ;; Initialize counter to 1
    i32.const 1
    local.set $i
    
    ;; Loop
    (loop $loop
      ;; Check if i <= n
      local.get $i
      local.get $n
      i32.le_s
      (if
        (then
          ;; result = result * i
          local.get $result
          local.get $i
          i32.mul
          local.set $result
          
          ;; i = i + 1
          local.get $i
          i32.const 1
          i32.add
          local.set $i
          
          ;; Continue loop
          br $loop
        )
      )
    )
    
    local.get $result
  )
  
  ;; Fibonacci function with recursion
  (func $fibonacci (param $n i32) (result i32)
    local.get $n
    i32.const 2
    i32.lt_s
    (if (result i32)
      (then
        local.get $n
      )
      (else
        local.get $n
        i32.const 1
        i32.sub
        call $fibonacci
        
        local.get $n
        i32.const 2
        i32.sub
        call $fibonacci
        
        i32.add
      )
    )
  )
  
  ;; Function with multiple return values
  (func $divmod (param $a i32) (param $b i32) (result i32 i32)
    local.get $a
    local.get $b
    i32.div_s
    
    local.get $a
    local.get $b
    i32.rem_s
  )
  
  ;; Function with floating point operations
  (func $float_ops (param $a f32) (param $b f32) (result f32)
    local.get $a
    local.get $b
    f32.add
    
    local.get $a
    local.get $b
    f32.mul
    
    f32.div
  )
  
  ;; Function with 64-bit integers
  (func $long_ops (param $a i64) (param $b i64) (result i64)
    local.get $a
    local.get $b
    i64.add
    
    i64.const 2
    i64.mul
  )
  
  ;; Function with double precision floats
  (func $double_ops (param $a f64) (param $b f64) (result f64)
    local.get $a
    f64.sqrt
    
    local.get $b
    f64.sin
    
    f64.add
  )
  
  ;; Memory operations
  (func $memory_test (param $offset i32) (param $value i32)
    ;; Store 32-bit integer
    local.get $offset
    local.get $value
    i32.store
    
    ;; Store 16-bit integer
    local.get $offset
    i32.const 4
    i32.add
    local.get $value
    i32.store16
    
    ;; Store 8-bit integer
    local.get $offset
    i32.const 6
    i32.add
    local.get $value
    i32.store8
  )
  
  (func $memory_load (param $offset i32) (result i32)
    ;; Load 32-bit integer
    local.get $offset
    i32.load
    
    ;; Load 16-bit unsigned
    local.get $offset
    i32.const 4
    i32.add
    i32.load16_u
    
    i32.add
    
    ;; Load 8-bit signed
    local.get $offset
    i32.const 6
    i32.add
    i32.load8_s
    
    i32.add
  )
  
  ;; Control flow examples
  (func $control_flow (param $x i32) (result i32)
    (local $result i32)
    
    ;; If-else statement
    local.get $x
    i32.const 0
    i32.gt_s
    (if (result i32)
      (then
        local.get $x
        i32.const 2
        i32.mul
      )
      (else
        local.get $x
        i32.const -1
        i32.mul
      )
    )
    local.set $result
    
    ;; Block with break
    (block $exit
      local.get $x
      i32.const 100
      i32.gt_s
      (if
        (then
          i32.const 100
          local.set $result
          br $exit
        )
      )
      
      local.get $result
      i32.const 10
      i32.add
      local.set $result
    )
    
    local.get $result
  )
  
  ;; Loop examples
  (func $sum_to_n (param $n i32) (result i32)
    (local $sum i32)
    (local $i i32)
    
    i32.const 0
    local.set $sum
    
    i32.const 1
    local.set $i
    
    (loop $loop
      local.get $i
      local.get $n
      i32.le_s
      (if
        (then
          local.get $sum
          local.get $i
          i32.add
          local.set $sum
          
          local.get $i
          i32.const 1
          i32.add
          local.set $i
          
          br $loop
        )
      )
    )
    
    local.get $sum
  )
  
  ;; Table operations
  (func $table_test (param $index i32) (param $value i32) (result i32)
    ;; Call function from table
    local.get $value
    i32.const 5
    local.get $index
    call_indirect (type $binary_op)
  )
  
  ;; Global variable operations
  (func $global_test
    ;; Increment counter
    global.get $counter
    i32.const 1
    i32.add
    global.set $counter
    
    ;; Set flag
    i32.const 1
    global.set $flag
  )
  
  ;; Comparison operations
  (func $compare (param $a i32) (param $b i32) (result i32)
    (local $result i32)
    
    ;; Equal
    local.get $a
    local.get $b
    i32.eq
    (if
      (then
        i32.const 1
        local.set $result
      )
    )
    
    ;; Not equal
    local.get $a
    local.get $b
    i32.ne
    (if
      (then
        local.get $result
        i32.const 2
        i32.add
        local.set $result
      )
    )
    
    ;; Less than
    local.get $a
    local.get $b
    i32.lt_s
    (if
      (then
        local.get $result
        i32.const 4
        i32.add
        local.set $result
      )
    )
    
    ;; Greater than
    local.get $a
    local.get $b
    i32.gt_s
    (if
      (then
        local.get $result
        i32.const 8
        i32.add
        local.set $result
      )
    )
    
    local.get $result
  )
  
  ;; Bitwise operations
  (func $bitwise (param $a i32) (param $b i32) (result i32)
    ;; AND
    local.get $a
    local.get $b
    i32.and
    
    ;; OR
    local.get $a
    local.get $b
    i32.or
    
    i32.xor
    
    ;; NOT (complement)
    local.get $a
    i32.const -1
    i32.xor
    
    i32.add
    
    ;; Shift left
    local.get $b
    i32.const 2
    i32.shl
    
    i32.add
    
    ;; Shift right
    local.get $b
    i32.const 1
    i32.shr_s
    
    i32.add
  )
  
  ;; Type conversion
  (func $convert (param $i i32) (param $f f32) (result f64)
    ;; Convert i32 to f32
    local.get $i
    f32.convert_i32_s
    
    ;; Add f32 parameter
    local.get $f
    f32.add
    
    ;; Convert to f64
    f64.promote_f32
    
    ;; Convert i32 to f64 and add
    local.get $i
    f64.convert_i32_s
    f64.add
  )
  
  ;; Select operation
  (func $select_test (param $a i32) (param $b i32) (param $condition i32) (result i32)
    local.get $a
    local.get $b
    local.get $condition
    select
  )
  
  ;; Unreachable and trap
  (func $trap_test (param $x i32)
    local.get $x
    i32.const 0
    i32.eq
    (if
      (then
        unreachable
      )
    )
  )
  
  ;; Function with complex control flow
  (func $complex_control (param $n i32) (result i32)
    (local $result i32)
    (local $temp i32)
    
    i32.const 0
    local.set $result
    
    (block $outer
      (loop $inner
        local.get $n
        i32.const 0
        i32.le_s
        br_if $outer
        
        local.get $n
        i32.const 2
        i32.rem_s
        i32.const 0
        i32.eq
        (if
          (then
            local.get $result
            local.get $n
            i32.add
            local.set $result
          )
          (else
            local.get $result
            local.get $n
            i32.const 2
            i32.mul
            i32.add
            local.set $result
          )
        )
        
        local.get $n
        i32.const 1
        i32.sub
        local.set $n
        
        br $inner
      )
    )
    
    local.get $result
  )
  
  ;; String operations (using memory)
  (func $string_length (param $ptr i32) (result i32)
    (local $len i32)
    (local $char i32)
    
    i32.const 0
    local.set $len
    
    (loop $loop
      local.get $ptr
      local.get $len
      i32.add
      i32.load8_u
      local.tee $char
      
      i32.const 0
      i32.ne
      (if
        (then
          local.get $len
          i32.const 1
          i32.add
          local.set $len
          br $loop
        )
      )
    )
    
    local.get $len
  )
  
  ;; Export section
  (export "add" (func $add))
  (export "subtract" (func $subtract))
  (export "multiply" (func $multiply))
  (export "divide" (func $divide))
  (export "factorial" (func $factorial))
  (export "fibonacci" (func $fibonacci))
  (export "divmod" (func $divmod))
  (export "memory" (memory $mem))
  (export "table" (table $table))
  (export "counter" (global $counter))
  (export "pi" (global $pi))
  (export "memory_test" (func $memory_test))
  (export "memory_load" (func $memory_load))
  (export "control_flow" (func $control_flow))
  (export "sum_to_n" (func $sum_to_n))
  (export "global_test" (func $global_test))
  (export "compare" (func $compare))
  (export "bitwise" (func $bitwise))
  (export "convert" (func $convert))
  (export "select_test" (func $select_test))
  (export "complex_control" (func $complex_control))
  (export "string_length" (func $string_length))
  
  ;; Start function
  (start $global_test)
)
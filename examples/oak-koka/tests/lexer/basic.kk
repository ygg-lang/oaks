// Koka Test File - Effects and Handlers
// This demonstrates actual Koka syntax

module basic

// 1. Basic type and function
fun square(x : int) : int {
    x * x
}

// 2. Algebraic Effects
effect emit {
    control emit( msg : string ) : ()
}

// 3. Effect handlers
fun test-emit() {
    with control emit(msg) {
        println("emitted: " + msg)
        resume(())
    }
    emit("hello koka")
}

// 4. Multiple effects and inferred types
fun combine( f : () -> <emit,console> () ) : <console> () {
    with handler {
        control emit(msg) { println("captured: " + msg); resume(()) }
    }
    f()
}

// 5. Match and pattern matching
fun describe( x : list<int> ) : string {
    match(x) {
        Nil -> "empty"
        Cons(head, tail) -> "starts with " + head.show
    }
}

// 6. Value and Variable
val constant-pi = 3.14159
var counter := 0

fun increment() {
    counter := counter + 1
}

// 7. Main entry point
pub fun main() {
    val res = square(7)
    println("Square of 7 is: " + res.show)
    test-emit()
}

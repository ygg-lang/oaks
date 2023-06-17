"Smalltalk Test File - Comprehensive Syntax Coverage
This file tests various Smalltalk syntax elements for lexer testing"

"Comments in Smalltalk are enclosed in double quotes"
"This is a single line comment"

"Multi-line comments
can span multiple lines
and contain any text"

"Basic literals and constants"
Object subclass: #BasicLiterals
    instanceVariableNames: ''
    classVariableNames: ''
    poolDictionaries: ''
    category: 'Examples-Basic'!

!BasicLiterals methodsFor: 'literals'!

numbers
    "Integer literals"
    | integers |
    integers := Array with: 0 with: 42 with: -17 with: 1000000.
    
    "Radix notation"
    | radixNumbers |
    radixNumbers := Array 
        with: 16r1A    "hexadecimal"
        with: 8r755    "octal"
        with: 2r1010   "binary"
        with: 36rZZ.   "base 36"
    
    "Float literals"
    | floats |
    floats := Array 
        with: 3.14159
        with: -2.718
        with: 1.23e-4
        with: 6.022e23.
    
    "Scaled decimal"
    | scaled |
    scaled := Array
        with: 3.14s2    "3.14 with scale 2"
        with: 123.45s4. "123.45 with scale 4"
    
    "Fractions"
    | fractions |
    fractions := Array
        with: (1/2)
        with: (22/7)
        with: (-3/4).
    
    ^Array with: integers with: radixNumbers with: floats with: scaled with: fractions!

characters
    "Character literals"
    | chars |
    chars := Array
        with: $a
        with: $Z
        with: $5
        with: $ 
        with: $$
        with: $'
        with: $"
        with: Character cr
        with: Character lf
        with: Character tab
        with: Character space.
    ^chars!

strings
    "String literals"
    | strings |
    strings := Array
        with: 'Hello, World!'
        with: 'Single quotes define strings'
        with: 'Embedded ''quotes'' are doubled'
        with: 'Multi-line
strings are
allowed'
        with: ''  "empty string"
        with: 'Special characters: ', Character cr asString, Character tab asString.
    ^strings!

symbols
    "Symbol literals"
    | symbols |
    symbols := Array
        with: #hello
        with: #'hello world'
        with: #'with spaces and punctuation!'
        with: #+
        with: #-
        with: #*
        with: #/
        with: #=
        with: #<
        with: #>
        with: #<=
        with: #>=
        with: #~=
        with: #at:
        with: #at:put:
        with: #ifTrue:ifFalse:.
    ^symbols!

arrays
    "Array literals"
    | arrays |
    arrays := Array
        with: #()                           "empty array"
        with: #(1 2 3 4 5)                 "number array"
        with: #('hello' 'world')           "string array"
        with: #(#a #b #c)                  "symbol array"
        with: #(1 'hello' #symbol $c)      "mixed array"
        with: #(#(1 2) #(3 4))             "nested arrays"
        with: #(true false nil).           "special literals"
    ^arrays!

booleans
    "Boolean literals"
    | booleans |
    booleans := Array
        with: true
        with: false
        with: nil.
    ^booleans!
! !

"Class definitions and inheritance"
Object subclass: #Animal
    instanceVariableNames: 'name age'
    classVariableNames: 'Population'
    poolDictionaries: ''
    category: 'Examples-Classes'!

!Animal class methodsFor: 'class initialization'!

initialize
    "Initialize class variables"
    Population := 0.
! !

!Animal class methodsFor: 'instance creation'!

named: aName
    "Create a new animal with the given name"
    ^self new name: aName; yourself!

named: aName age: anAge
    "Create a new animal with name and age"
    ^self new name: aName; age: anAge; yourself!
! !

!Animal methodsFor: 'initialization'!

initialize
    "Initialize instance"
    super initialize.
    name := 'Unknown'.
    age := 0.
    self class incrementPopulation.!

name: aName
    "Set the animal's name"
    name := aName!

age: anAge
    "Set the animal's age"
    age := anAge!
! !

!Animal methodsFor: 'accessing'!

name
    "Return the animal's name"
    ^name!

age
    "Return the animal's age"
    ^age!

description
    "Return a description of the animal"
    ^'Animal named ', name, ' is ', age printString, ' years old'!
! !

!Animal methodsFor: 'actions'!

makeSound
    "Make a generic animal sound"
    ^'Some animal sound'!

eat: food
    "The animal eats something"
    ^'The animal eats ', food!

sleep
    "The animal sleeps"
    ^name, ' is sleeping'!
! !

!Animal class methodsFor: 'population'!

incrementPopulation
    "Increment the population count"
    Population := Population + 1!

population
    "Return the current population"
    ^Population!
! !

"Inheritance example"
Animal subclass: #Dog
    instanceVariableNames: 'breed owner'
    classVariableNames: ''
    poolDictionaries: ''
    category: 'Examples-Classes'!

!Dog methodsFor: 'initialization'!

breed: aBreed
    "Set the dog's breed"
    breed := aBreed!

owner: anOwner
    "Set the dog's owner"
    owner := anOwner!
! !

!Dog methodsFor: 'accessing'!

breed
    "Return the dog's breed"
    ^breed!

owner
    "Return the dog's owner"
    ^owner!

description
    "Return a description of the dog"
    | desc |
    desc := super description.
    breed ifNotNil: [desc := desc, ', breed: ', breed].
    owner ifNotNil: [desc := desc, ', owner: ', owner].
    ^desc!
! !

!Dog methodsFor: 'actions'!

makeSound
    "Dogs bark"
    ^'Woof! Woof!'!

wagTail
    "Dogs wag their tails"
    ^name, ' wags tail happily'!

fetch: object
    "Dogs can fetch things"
    ^name, ' fetches the ', object!
! !

"Another inheritance example"
Animal subclass: #Cat
    instanceVariableNames: 'indoor'
    classVariableNames: ''
    poolDictionaries: ''
    category: 'Examples-Classes'!

!Cat methodsFor: 'initialization'!

indoor: aBoolean
    "Set whether the cat is indoor"
    indoor := aBoolean!
! !

!Cat methodsFor: 'accessing'!

indoor
    "Return whether the cat is indoor"
    ^indoor!

description
    "Return a description of the cat"
    | desc |
    desc := super description.
    indoor ifNotNil: [
        desc := desc, ', ', (indoor ifTrue: ['indoor'] ifFalse: ['outdoor']), ' cat'].
    ^desc!
! !

!Cat methodsFor: 'actions'!

makeSound
    "Cats meow"
    ^'Meow!'!

purr
    "Cats purr when happy"
    ^name, ' purrs contentedly'!

hunt
    "Cats hunt"
    ^indoor 
        ifTrue: [name, ' stalks a toy mouse']
        ifFalse: [name, ' hunts real prey']!
! !

"Method categories and protocols"
Object subclass: #Calculator
    instanceVariableNames: 'memory'
    classVariableNames: ''
    poolDictionaries: ''
    category: 'Examples-Math'!

!Calculator methodsFor: 'initialization'!

initialize
    "Initialize the calculator"
    super initialize.
    memory := 0.!
! !

!Calculator methodsFor: 'arithmetic'!

add: a to: b
    "Add two numbers"
    ^a + b!

subtract: a from: b
    "Subtract a from b"
    ^b - a!

multiply: a by: b
    "Multiply two numbers"
    ^a * b!

divide: a by: b
    "Divide a by b"
    b = 0 ifTrue: [self error: 'Division by zero'].
    ^a / b!

power: base to: exponent
    "Raise base to the power of exponent"
    ^base raisedTo: exponent!
! !

!Calculator methodsFor: 'memory operations'!

store: value
    "Store a value in memory"
    memory := value!

recall
    "Recall the value from memory"
    ^memory!

clearMemory
    "Clear the memory"
    memory := 0!

addToMemory: value
    "Add a value to memory"
    memory := memory + value!
! !

!Calculator methodsFor: 'advanced operations'!

factorial: n
    "Calculate factorial of n"
    n < 0 ifTrue: [self error: 'Factorial of negative number'].
    n <= 1 ifTrue: [^1].
    ^n * (self factorial: n - 1)!

fibonacci: n
    "Calculate nth Fibonacci number"
    n < 0 ifTrue: [self error: 'Fibonacci of negative number'].
    n <= 1 ifTrue: [^n].
    ^(self fibonacci: n - 1) + (self fibonacci: n - 2)!

gcd: a with: b
    "Calculate greatest common divisor"
    b = 0 ifTrue: [^a abs].
    ^self gcd: b with: a \\ b!

isPrime: n
    "Check if n is prime"
    n < 2 ifTrue: [^false].
    n = 2 ifTrue: [^true].
    n even ifTrue: [^false].
    3 to: n sqrt do: [:i |
        (n \\ i) = 0 ifTrue: [^false]].
    ^true!
! !

"Control structures and blocks"
Object subclass: #ControlStructures
    instanceVariableNames: ''
    classVariableNames: ''
    poolDictionaries: ''
    category: 'Examples-Control'!

!ControlStructures methodsFor: 'conditionals'!

testConditionals: x
    "Test various conditional structures"
    | result |
    
    "Simple if-then"
    x > 0 ifTrue: [result := 'positive'].
    
    "If-then-else"
    result := x > 0 
        ifTrue: ['positive'] 
        ifFalse: ['non-positive'].
    
    "Nested conditionals"
    result := x > 0
        ifTrue: [
            x even 
                ifTrue: ['positive even']
                ifFalse: ['positive odd']]
        ifFalse: [
            x = 0
                ifTrue: ['zero']
                ifFalse: ['negative']].
    
    "Multiple conditions"
    (x > 0 and: [x < 10]) ifTrue: [result := 'small positive'].
    (x < 0 or: [x > 100]) ifTrue: [result := 'extreme value'].
    
    ^result!

switchExample: value
    "Simulate switch-case using cascaded conditionals"
    value = 1 ifTrue: [^'one'].
    value = 2 ifTrue: [^'two'].
    value = 3 ifTrue: [^'three'].
    (value between: 4 and: 10) ifTrue: [^'small number'].
    value > 10 ifTrue: [^'large number'].
    ^'unknown'!
! !

!ControlStructures methodsFor: 'loops'!

testLoops
    "Test various loop structures"
    | result sum counter |
    result := OrderedCollection new.
    
    "Simple loop with to:do:"
    1 to: 5 do: [:i | result add: i].
    
    "Loop with step"
    1 to: 10 by: 2 do: [:i | result add: i].
    
    "Reverse loop"
    10 to: 1 by: -1 do: [:i | result add: i].
    
    "While loop"
    sum := 0.
    counter := 1.
    [counter <= 10] whileTrue: [
        sum := sum + counter.
        counter := counter + 1].
    
    "Until loop"
    counter := 1.
    [counter > 5] whileFalse: [
        result add: counter.
        counter := counter + 1].
    
    "Collection iteration"
    #(1 2 3 4 5) do: [:each | result add: each squared].
    
    "Collection iteration with index"
    #('a' 'b' 'c') doWithIndex: [:each :index |
        result add: index -> each].
    
    "Select and reject"
    result addAll: ((1 to: 10) select: [:n | n even]).
    result addAll: ((1 to: 10) reject: [:n | n odd]).
    
    "Collect (map)"
    result addAll: ((1 to: 5) collect: [:n | n * n]).
    
    "Detect (find)"
    | found |
    found := (1 to: 10) detect: [:n | n > 7] ifNone: [nil].
    found ifNotNil: [result add: found].
    
    ^result!

timesRepeat: n
    "Repeat something n times"
    | result |
    result := OrderedCollection new.
    n timesRepeat: [result add: 'repeated'].
    ^result!
! !

!ControlStructures methodsFor: 'exception handling'!

exceptionHandling
    "Test exception handling"
    | result |
    result := OrderedCollection new.
    
    "Basic exception handling"
    [
        result add: 'before error'.
        1 / 0.
        result add: 'after error'.
    ] on: ZeroDivide do: [:ex |
        result add: 'caught division by zero'.
        ex return: nil].
    
    "Multiple exception types"
    [
        'not a number' + 1.
    ] on: Error do: [:ex |
        result add: 'caught error: ', ex messageText].
    
    "Ensure (finally)"
    [
        result add: 'in protected block'.
        self error: 'test error'.
    ] ensure: [
        result add: 'in ensure block'].
    
    "Custom exception"
    [
        self error: 'Custom error message'.
    ] on: Error do: [:ex |
        result add: 'Custom error: ', ex messageText].
    
    ^result!
! !

"Collections and iteration"
Object subclass: #CollectionExamples
    instanceVariableNames: ''
    classVariableNames: ''
    poolDictionaries: ''
    category: 'Examples-Collections'!

!CollectionExamples methodsFor: 'arrays'!

arrayOperations
    "Test array operations"
    | arr result |
    result := OrderedCollection new.
    
    "Array creation"
    arr := Array new: 5.
    arr at: 1 put: 'first'.
    arr at: 2 put: 'second'.
    arr at: 5 put: 'last'.
    
    "Array access"
    result add: (arr at: 1).
    result add: (arr at: 5).
    
    "Array with initial values"
    arr := Array with: 1 with: 2 with: 3 with: 4 with: 5.
    result addAll: arr.
    
    "Array iteration"
    arr do: [:each | result add: each * 2].
    
    ^result!

orderedCollections
    "Test OrderedCollection operations"
    | coll |
    coll := OrderedCollection new.
    
    "Adding elements"
    coll add: 'first'.
    coll add: 'second'.
    coll addFirst: 'beginning'.
    coll addLast: 'end'.
    coll addAll: #('a' 'b' 'c').
    
    "Accessing elements"
    coll first.
    coll last.
    coll at: 2.
    
    "Removing elements"
    coll removeFirst.
    coll removeLast.
    coll remove: 'second'.
    coll removeAt: 1.
    
    "Collection operations"
    coll select: [:each | each size > 1].
    coll reject: [:each | each = 'a'].
    coll collect: [:each | each asUppercase].
    coll detect: [:each | each beginsWith: 'b'] ifNone: [nil].
    
    ^coll!

sets
    "Test Set operations"
    | set1 set2 |
    set1 := Set new.
    set1 add: 1; add: 2; add: 3; add: 2. "duplicate ignored"
    
    set2 := Set with: 3 with: 4 with: 5.
    
    "Set operations"
    | union intersection difference |
    union := set1 union: set2.
    intersection := set1 intersection: set2.
    difference := set1 difference: set2.
    
    ^Array with: union with: intersection with: difference!

dictionaries
    "Test Dictionary operations"
    | dict |
    dict := Dictionary new.
    
    "Adding key-value pairs"
    dict at: 'name' put: 'John'.
    dict at: 'age' put: 30.
    dict at: 'city' put: 'New York'.
    
    "Accessing values"
    dict at: 'name'.
    dict at: 'country' ifAbsent: ['Unknown'].
    
    "Testing keys"
    dict includesKey: 'age'.
    dict includesKey: 'country'.
    
    "Iteration"
    dict keysDo: [:key | Transcript show: key; cr].
    dict valuesDo: [:value | Transcript show: value; cr].
    dict keysAndValuesDo: [:key :value |
        Transcript show: key, ' -> ', value printString; cr].
    
    ^dict!
! !

"Blocks and closures"
Object subclass: #BlockExamples
    instanceVariableNames: ''
    classVariableNames: ''
    poolDictionaries: ''
    category: 'Examples-Blocks'!

!BlockExamples methodsFor: 'basic blocks'!

simpleBlocks
    "Test simple block operations"
    | block result |
    
    "Block with no arguments"
    block := [42].
    result := block value.
    
    "Block with one argument"
    block := [:x | x * x].
    result := block value: 5.
    
    "Block with multiple arguments"
    block := [:x :y | x + y].
    result := block value: 3 value: 4.
    
    "Block with local variables"
    block := [
        | temp |
        temp := 10.
        temp * 2].
    result := block value.
    
    ^result!

closures
    "Test closure behavior"
    | counter makeCounter |
    
    "Simple closure"
    counter := 0.
    makeCounter := [counter := counter + 1].
    
    "Multiple calls to closure"
    | results |
    results := OrderedCollection new.
    results add: makeCounter value.
    results add: makeCounter value.
    results add: makeCounter value.
    
    ^results!

higherOrderFunctions
    "Test higher-order functions with blocks"
    | numbers |
    numbers := #(1 2 3 4 5 6 7 8 9 10).
    
    "Map operation"
    | squares |
    squares := numbers collect: [:n | n * n].
    
    "Filter operation"
    | evens |
    evens := numbers select: [:n | n even].
    
    "Reduce operation"
    | sum |
    sum := numbers inject: 0 into: [:acc :n | acc + n].
    
    "Find operation"
    | found |
    found := numbers detect: [:n | n > 5] ifNone: [nil].
    
    "All/any operations"
    | allPositive anyEven |
    allPositive := numbers allSatisfy: [:n | n > 0].
    anyEven := numbers anySatisfy: [:n | n even].
    
    ^Array 
        with: squares 
        with: evens 
        with: sum 
        with: found 
        with: allPositive 
        with: anyEven!

blockComposition
    "Test block composition"
    | add multiply compose |
    
    add := [:x | x + 1].
    multiply := [:x | x * 2].
    
    "Manual composition"
    compose := [:x | multiply value: (add value: x)].
    
    "Using composition"
    | result |
    result := compose value: 5. "Should be (5 + 1) * 2 = 12"
    
    ^result!
! !

"Metaclasses and reflection"
Object subclass: #MetaExample
    instanceVariableNames: 'data'
    classVariableNames: ''
    poolDictionaries: ''
    category: 'Examples-Meta'!

!MetaExample methodsFor: 'reflection'!

classInformation
    "Get information about the class"
    | info |
    info := Dictionary new.
    
    info at: 'className' put: self class name.
    info at: 'superclass' put: self class superclass name.
    info at: 'instanceVariables' put: self class instanceVariableNames.
    info at: 'methods' put: self class methodDict keys.
    info at: 'category' put: self class category.
    
    ^info!

methodLookup: selector
    "Look up a method"
    | method |
    method := self class lookupSelector: selector.
    ^method ifNotNil: [method sourceCode] ifNil: ['Method not found']!

respondsToMessage: selector
    "Check if object responds to a message"
    ^self respondsTo: selector!

performMessage: selector
    "Perform a message dynamically"
    ^self perform: selector!

performMessage: selector with: argument
    "Perform a message with argument"
    ^self perform: selector with: argument!
! !

!MetaExample methodsFor: 'dynamic behavior'!

doesNotUnderstand: aMessage
    "Handle unknown messages"
    | selector |
    selector := aMessage selector.
    
    "Check if it's a getter"
    (selector endsWith: 'Value') ifTrue: [
        ^'Dynamic getter for ', selector].
    
    "Check if it's a setter"
    (selector endsWith: ':') ifTrue: [
        ^'Dynamic setter for ', selector].
    
    "Default behavior"
    ^super doesNotUnderstand: aMessage!

addMethod: selector withCode: codeString
    "Dynamically add a method (simplified)"
    "This would require compiler access in real implementation"
    ^'Would add method ', selector, ' with code: ', codeString!
! !

"Streams and I/O"
Object subclass: #StreamExamples
    instanceVariableNames: ''
    classVariableNames: ''
    poolDictionaries: ''
    category: 'Examples-Streams'!

!StreamExamples methodsFor: 'string streams'!

stringStreams
    "Test string stream operations"
    | readStream writeStream |
    
    "Read stream"
    readStream := 'Hello, World!' readStream.
    | chars |
    chars := OrderedCollection new.
    [readStream atEnd] whileFalse: [
        chars add: readStream next].
    
    "Write stream"
    writeStream := String new writeStream.
    writeStream 
        nextPutAll: 'Hello';
        nextPut: $,;
        space;
        nextPutAll: 'World';
        nextPut: $!.
    
    ^Array with: chars with: writeStream contents!

collectionStreams
    "Test collection stream operations"
    | stream result |
    
    "Reading from collection"
    stream := #(1 2 3 4 5) readStream.
    result := OrderedCollection new.
    
    result add: stream next.      "1"
    result add: stream peek.      "2 (doesn't advance)"
    result add: stream next.      "2"
    result addAll: stream upToEnd. "3 4 5"
    
    "Writing to collection"
    stream := Array new: 5 writeStream.
    1 to: 5 do: [:i | stream nextPut: i].
    
    ^Array with: result with: stream contents!
! !

"Process and concurrency (basic)"
Object subclass: #ProcessExamples
    instanceVariableNames: ''
    classVariableNames: ''
    poolDictionaries: ''
    category: 'Examples-Process'!

!ProcessExamples methodsFor: 'basic processes'!

forkExample
    "Example of forking a process"
    | result |
    result := OrderedCollection new.
    
    "Fork a simple process"
    [
        1 to: 5 do: [:i |
            result add: 'Process: ', i printString.
            (Delay forMilliseconds: 100) wait].
    ] fork.
    
    "Main process continues"
    result add: 'Main process'.
    
    ^result!

semaphoreExample
    "Example using semaphores"
    | semaphore result |
    semaphore := Semaphore new.
    result := OrderedCollection new.
    
    "Producer process"
    [
        1 to: 3 do: [:i |
            result add: 'Producing ', i printString.
            semaphore signal].
    ] fork.
    
    "Consumer process"
    [
        3 timesRepeat: [
            semaphore wait.
            result add: 'Consumed item'].
    ] fork.
    
    ^result!
! !

"Testing framework example"
TestCase subclass: #ExampleTests
    instanceVariableNames: ''
    classVariableNames: ''
    poolDictionaries: ''
    category: 'Examples-Tests'!

!ExampleTests methodsFor: 'tests'!

testArithmetic
    "Test basic arithmetic"
    self assert: 2 + 2 equals: 4.
    self assert: 5 - 3 equals: 2.
    self assert: 3 * 4 equals: 12.
    self assert: 8 / 2 equals: 4.!

testCollections
    "Test collection operations"
    | coll |
    coll := OrderedCollection with: 1 with: 2 with: 3.
    
    self assert: coll size equals: 3.
    self assert: coll first equals: 1.
    self assert: coll last equals: 3.
    
    coll add: 4.
    self assert: coll size equals: 4.!

testExceptions
    "Test exception handling"
    self 
        should: [1 / 0] 
        raise: ZeroDivide.
    
    self 
        shouldnt: [1 / 2] 
        raise: ZeroDivide.!
! !

"Workspace expressions and examples"
"These would typically be evaluated in a workspace"

"Basic expressions"
2 + 3.
'Hello' , ' World'.
#(1 2 3) collect: [:each | each squared].

"Object creation and message sending"
| dog |
dog := Dog new.
dog name: 'Buddy'; breed: 'Golden Retriever'; age: 3.
dog makeSound.
dog description.

"Collection manipulation"
| numbers evens squares |
numbers := 1 to: 10.
evens := numbers select: [:n | n even].
squares := numbers collect: [:n | n * n].

"Block examples"
| factorial |
factorial := [:n | 
    n <= 1 
        ifTrue: [1] 
        ifFalse: [n * (factorial value: n - 1)]].
factorial value: 5.

"Class creation example"
Object subclass: #Point
    instanceVariableNames: 'x y'
    classVariableNames: ''
    poolDictionaries: ''
    category: 'Examples-Geometry'.

"Method definition for Point"
!Point methodsFor: 'initialization'!
x: xCoord y: yCoord
    x := xCoord.
    y := yCoord.
! !

!Point methodsFor: 'accessing'!
x
    ^x!

y
    ^y!

distanceTo: anotherPoint
    | dx dy |
    dx := x - anotherPoint x.
    dy := y - anotherPoint y.
    ^(dx * dx + (dy * dy)) sqrt!
! !

"Using the Point class"
| p1 p2 distance |
p1 := Point new x: 0 y: 0.
p2 := Point new x: 3 y: 4.
distance := p1 distanceTo: p2.

"Advanced features"

"Pragma example"
!Object methodsFor: 'examples'!
exampleMethod
    <primitive: 'primitiveExample'>
    "This method has a primitive pragma"
    ^self!
! !

"Class variables and pool dictionaries"
Object subclass: #Counter
    instanceVariableNames: 'value'
    classVariableNames: 'TotalCount DefaultIncrement'
    poolDictionaries: 'TextConstants'
    category: 'Examples-Advanced'!

!Counter class methodsFor: 'class initialization'!
initialize
    TotalCount := 0.
    DefaultIncrement := 1.!
! !

"Shared pools"
SharedPool subclass: #MathConstants
    instanceVariableNames: ''
    classVariableNames: 'Pi E GoldenRatio'
    poolDictionaries: ''
    category: 'Examples-Constants'!

!MathConstants class methodsFor: 'initialization'!
initialize
    Pi := 3.14159265358979323846.
    E := 2.71828182845904523536.
    GoldenRatio := 1.61803398874989484820.!
! !

"Trait-like behavior (using composition)"
Object subclass: #Comparable
    instanceVariableNames: ''
    classVariableNames: ''
    poolDictionaries: ''
    category: 'Examples-Traits'!

!Comparable methodsFor: 'comparing'!
< other
    "Subclasses must implement"
    self subclassResponsibility!

<= other
    ^(self < other) or: [self = other]!

> other
    ^(self <= other) not!

>= other
    ^(self < other) not!

between: min and: max
    ^(self >= min) and: [self <= max]!
! !

"Final comprehensive example"
Object subclass: #BankAccount
    instanceVariableNames: 'accountNumber balance owner transactions'
    classVariableNames: 'NextAccountNumber InterestRate'
    poolDictionaries: ''
    category: 'Examples-Final'!

!BankAccount class methodsFor: 'class initialization'!
initialize
    NextAccountNumber := 1000.
    InterestRate := 0.02.!

nextAccountNumber
    | number |
    number := NextAccountNumber.
    NextAccountNumber := NextAccountNumber + 1.
    ^number!

interestRate
    ^InterestRate!

interestRate: aRate
    InterestRate := aRate!
! !

!BankAccount class methodsFor: 'instance creation'!
forOwner: ownerName
    ^self new initializeForOwner: ownerName!

forOwner: ownerName withBalance: initialBalance
    ^(self new initializeForOwner: ownerName) 
        deposit: initialBalance;
        yourself!
! !

!BankAccount methodsFor: 'initialization'!
initializeForOwner: ownerName
    super initialize.
    accountNumber := self class nextAccountNumber.
    balance := 0.
    owner := ownerName.
    transactions := OrderedCollection new.
    self addTransaction: 'Account opened'.!
! !

!BankAccount methodsFor: 'accessing'!
accountNumber
    ^accountNumber!

balance
    ^balance!

owner
    ^owner!

transactions
    ^transactions copy!
! !

!BankAccount methodsFor: 'operations'!
deposit: amount
    amount <= 0 ifTrue: [
        self error: 'Deposit amount must be positive'].
    
    balance := balance + amount.
    self addTransaction: 'Deposit: $', amount printString.
    ^balance!

withdraw: amount
    amount <= 0 ifTrue: [
        self error: 'Withdrawal amount must be positive'].
    
    amount > balance ifTrue: [
        self error: 'Insufficient funds'].
    
    balance := balance - amount.
    self addTransaction: 'Withdrawal: $', amount printString.
    ^balance!

transfer: amount to: otherAccount
    self withdraw: amount.
    otherAccount deposit: amount.
    self addTransaction: 'Transfer out: $', amount printString, 
                        ' to account ', otherAccount accountNumber printString.
    otherAccount addTransaction: 'Transfer in: $', amount printString,
                                ' from account ', accountNumber printString.!

applyInterest
    | interest |
    interest := balance * self class interestRate.
    balance := balance + interest.
    self addTransaction: 'Interest: $', interest printString.
    ^interest!
! !

!BankAccount methodsFor: 'private'!
addTransaction: description
    | timestamp |
    timestamp := DateAndTime now.
    transactions add: timestamp printString, ': ', description.!
! !

!BankAccount methodsFor: 'printing'!
printOn: aStream
    aStream 
        nextPutAll: 'BankAccount(';
        nextPutAll: accountNumber printString;
        nextPutAll: ', $';
        nextPutAll: balance printString;
        nextPutAll: ', ';
        nextPutAll: owner;
        nextPut: $)!
! !
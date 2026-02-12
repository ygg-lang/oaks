NB. J Language Lexer Test
NB. Basic Arithmetic and Functions

NB. Simple assignment
a =: 10
b =: 20
c =: a + b

NB. Verbs (Functions)
NB. Double indentation for nested blocks if applicable (J is line-based mostly)
plus =: +
times =: *

NB. Factorial using recursion
fact =: 3 : 0
    if. y. = 0 do. 1
    else. y. * fact y. - 1
    end.
)

NB. Apply factorial
result =: fact 5

NB. Lists and operations
list =: 1 2 3 4 5
squared =: list ^ 2
sum =: +/ list
avg =: sum % # list

NB. Control structures
test_control =: 3 : 0
    if. y. > 10 do.
        'Greater than 10'
    elseif. y. = 10 do.
        'Equal to 10'
    else.
        'Less than 10'
    end.
)

NB. While loop
count =: 0
while_loop =: 3 : 0
    while. count < 5 do.
        count =: count + 1
    end.
    count
)

NB. String manipulation
str =: 'Hello, J!'
len =: # str
rev =: |. str

NB. Higher order functions (Adverbs/Conjunctions)
inc =: +&1
inc_list =: inc list

NB. Boxed arrays
boxed =: < 'Text' ; 123 ; 3.14

NB. Matrix operations
matrix =: 2 2 $ 1 2 3 4
det =: -/ . * matrix

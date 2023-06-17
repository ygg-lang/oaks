% Prolog Test File - Comprehensive Syntax Coverage
% This file tests various Prolog syntax elements for lexer testing

% Module declaration (SWI-Prolog)
:- module(comprehensive_test, [
    person/3,
    parent/2,
    ancestor/2,
    factorial/2,
    fibonacci/2,
    append_list/3,
    reverse_list/2,
    member_check/2,
    length_list/2,
    sum_list/2,
    max_list/2,
    sort_list/2,
    quicksort/2,
    binary_tree/1,
    tree_member/2,
    tree_insert/3,
    graph_edge/2,
    path/3,
    connected/2,
    solve_puzzle/1
]).

% Directives and pragmas
:- use_module(library(lists)).
:- use_module(library(clpfd)).
:- use_module(library(dcg/basics)).
:- use_module(library(apply)).
:- use_module(library(aggregate)).

:- dynamic(fact/1).
:- multifile(person/3).
:- discontiguous(test_case/2).

% Operators
:- op(500, yfx, likes).
:- op(600, xfx, loves).
:- op(700, xfx, married_to).

% Facts - Basic database
person(john, doe, 30).
person(jane, smith, 25).
person(bob, johnson, 35).
person(alice, brown, 28).
person(charlie, davis, 40).
person(diana, wilson, 32).
person(eve, miller, 27).
person(frank, garcia, 45).

% Facts with different arities
age(john, 30).
age(jane, 25).
age(bob, 35).
age(alice, 28).

% Gender facts
male(john).
male(bob).
male(charlie).
male(frank).

female(jane).
female(alice).
female(diana).
female(eve).

% Family relationships
parent(john, alice).
parent(jane, alice).
parent(bob, charlie).
parent(alice, diana).
parent(charlie, eve).
parent(diana, frank).

% Marriage relationships
married(john, jane).
married(bob, alice).
married(charlie, diana).

% Preferences and relationships
john likes pizza.
jane likes pasta.
bob likes burgers.
alice likes salad.

john loves jane.
bob loves alice.
charlie loves diana.

john married_to jane.
bob married_to alice.
charlie married_to diana.

% Location facts
lives_in(john, new_york).
lives_in(jane, new_york).
lives_in(bob, london).
lives_in(alice, london).
lives_in(charlie, paris).
lives_in(diana, paris).

% Occupation facts
works_as(john, engineer).
works_as(jane, doctor).
works_as(bob, teacher).
works_as(alice, lawyer).
works_as(charlie, artist).
works_as(diana, scientist).

% Rules - Basic logical relationships

% Grandparent relationship
grandparent(X, Z) :-
    parent(X, Y),
    parent(Y, Z).

% Ancestor relationship (recursive)
ancestor(X, Y) :-
    parent(X, Y).
ancestor(X, Y) :-
    parent(X, Z),
    ancestor(Z, Y).

% Sibling relationship
sibling(X, Y) :-
    parent(Z, X),
    parent(Z, Y),
    X \= Y.

% Uncle/Aunt relationship
uncle(X, Y) :-
    male(X),
    sibling(X, Z),
    parent(Z, Y).

aunt(X, Y) :-
    female(X),
    sibling(X, Z),
    parent(Z, Y).

% Cousin relationship
cousin(X, Y) :-
    parent(A, X),
    parent(B, Y),
    sibling(A, B).

% Same generation
same_generation(X, Y) :-
    person(X, _, _),
    person(Y, _, _),
    X \= Y,
    \+ ancestor(X, Y),
    \+ ancestor(Y, X).

% Mathematical predicates

% Factorial
factorial(0, 1) :- !.
factorial(N, F) :-
    N > 0,
    N1 is N - 1,
    factorial(N1, F1),
    F is N * F1.

% Fibonacci sequence
fibonacci(0, 0) :- !.
fibonacci(1, 1) :- !.
fibonacci(N, F) :-
    N > 1,
    N1 is N - 1,
    N2 is N - 2,
    fibonacci(N1, F1),
    fibonacci(N2, F2),
    F is F1 + F2.

% Greatest Common Divisor
gcd(X, 0, X) :- X > 0, !.
gcd(X, Y, G) :-
    Y > 0,
    R is X mod Y,
    gcd(Y, R, G).

% Prime number check
is_prime(2) :- !.
is_prime(N) :-
    N > 2,
    N mod 2 =\= 0,
    \+ has_factor(N, 3).

has_factor(N, F) :-
    F * F =< N,
    N mod F =:= 0.
has_factor(N, F) :-
    F * F < N,
    F2 is F + 2,
    has_factor(N, F2).

% List operations

% Append lists
append_list([], L, L).
append_list([H|T], L, [H|R]) :-
    append_list(T, L, R).

% Reverse list
reverse_list(List, Reversed) :-
    reverse_list(List, [], Reversed).

reverse_list([], Acc, Acc).
reverse_list([H|T], Acc, Reversed) :-
    reverse_list(T, [H|Acc], Reversed).

% Member check
member_check(X, [X|_]).
member_check(X, [_|T]) :-
    member_check(X, T).

% Length of list
length_list([], 0).
length_list([_|T], N) :-
    length_list(T, N1),
    N is N1 + 1.

% Sum of list elements
sum_list([], 0).
sum_list([H|T], Sum) :-
    sum_list(T, TailSum),
    Sum is H + TailSum.

% Maximum element in list
max_list([X], X).
max_list([H|T], Max) :-
    max_list(T, TailMax),
    Max is max(H, TailMax).

% Minimum element in list
min_list([X], X).
min_list([H|T], Min) :-
    min_list(T, TailMin),
    Min is min(H, TailMin).

% Sort list (insertion sort)
sort_list([], []).
sort_list([H|T], Sorted) :-
    sort_list(T, SortedTail),
    insert_sorted(H, SortedTail, Sorted).

insert_sorted(X, [], [X]).
insert_sorted(X, [H|T], [X,H|T]) :-
    X =< H, !.
insert_sorted(X, [H|T], [H|Result]) :-
    X > H,
    insert_sorted(X, T, Result).

% Quicksort
quicksort([], []).
quicksort([H|T], Sorted) :-
    partition(H, T, Less, Greater),
    quicksort(Less, SortedLess),
    quicksort(Greater, SortedGreater),
    append_list(SortedLess, [H|SortedGreater], Sorted).

partition(_, [], [], []).
partition(Pivot, [H|T], [H|Less], Greater) :-
    H =< Pivot, !,
    partition(Pivot, T, Less, Greater).
partition(Pivot, [H|T], Less, [H|Greater]) :-
    H > Pivot,
    partition(Pivot, T, Less, Greater).

% Remove duplicates
remove_duplicates([], []).
remove_duplicates([H|T], [H|Result]) :-
    \+ member_check(H, T), !,
    remove_duplicates(T, Result).
remove_duplicates([H|T], Result) :-
    member_check(H, T),
    remove_duplicates(T, Result).

% Flatten nested list
flatten_list([], []).
flatten_list([H|T], Flattened) :-
    is_list(H), !,
    flatten_list(H, FlatH),
    flatten_list(T, FlatT),
    append_list(FlatH, FlatT, Flattened).
flatten_list([H|T], [H|FlatT]) :-
    \+ is_list(H),
    flatten_list(T, FlatT).

% Binary tree operations

% Binary tree structure: empty or tree(Left, Value, Right)
binary_tree(empty).
binary_tree(tree(Left, _, Right)) :-
    binary_tree(Left),
    binary_tree(Right).

% Tree membership
tree_member(X, tree(_, X, _)).
tree_member(X, tree(Left, _, _)) :-
    tree_member(X, Left).
tree_member(X, tree(_, _, Right)) :-
    tree_member(X, Right).

% Tree insertion (binary search tree)
tree_insert(X, empty, tree(empty, X, empty)).
tree_insert(X, tree(Left, Y, Right), tree(NewLeft, Y, Right)) :-
    X < Y,
    tree_insert(X, Left, NewLeft).
tree_insert(X, tree(Left, Y, Right), tree(Left, Y, NewRight)) :-
    X > Y,
    tree_insert(X, Right, NewRight).
tree_insert(X, tree(Left, X, Right), tree(Left, X, Right)).

% Tree traversal
inorder(empty, []).
inorder(tree(Left, X, Right), Traversal) :-
    inorder(Left, LeftTraversal),
    inorder(Right, RightTraversal),
    append_list(LeftTraversal, [X|RightTraversal], Traversal).

preorder(empty, []).
preorder(tree(Left, X, Right), [X|Traversal]) :-
    preorder(Left, LeftTraversal),
    preorder(Right, RightTraversal),
    append_list(LeftTraversal, RightTraversal, Traversal).

postorder(empty, []).
postorder(tree(Left, X, Right), Traversal) :-
    postorder(Left, LeftTraversal),
    postorder(Right, RightTraversal),
    append_list(LeftTraversal, RightTraversal, TempTraversal),
    append_list(TempTraversal, [X], Traversal).

% Graph operations

% Graph edges
graph_edge(a, b).
graph_edge(b, c).
graph_edge(c, d).
graph_edge(a, e).
graph_edge(e, f).
graph_edge(f, d).
graph_edge(b, g).
graph_edge(g, h).
graph_edge(h, d).

% Path finding
path(X, Y, [X, Y]) :-
    graph_edge(X, Y).
path(X, Y, [X|Path]) :-
    graph_edge(X, Z),
    path(Z, Y, Path).

% Connected components
connected(X, Y) :-
    path(X, Y, _).
connected(X, Y) :-
    path(Y, X, _).

% Shortest path (breadth-first search)
shortest_path(Start, End, Path) :-
    bfs([[Start]], End, Path).

bfs([[End|Path]|_], End, [End|Path]).
bfs([Path|Paths], End, ShortestPath) :-
    extend_path(Path, NewPaths),
    append_list(Paths, NewPaths, AllPaths),
    bfs(AllPaths, End, ShortestPath).

extend_path([Node|Path], NewPaths) :-
    findall([NewNode, Node|Path],
            (graph_edge(Node, NewNode), \+ member_check(NewNode, [Node|Path])),
            NewPaths).

% Constraint Logic Programming (CLP)

% N-Queens problem
n_queens(N, Queens) :-
    length(Queens, N),
    Queens ins 1..N,
    safe_queens(Queens).

safe_queens([]).
safe_queens([Q|Qs]) :-
    safe_queens(Qs),
    no_attack(Q, Qs, 1).

no_attack(_, [], _).
no_attack(Q, [Q1|Qs], Dist) :-
    Q #\= Q1,
    abs(Q - Q1) #\= Dist,
    Dist1 is Dist + 1,
    no_attack(Q, Qs, Dist1).

% Sudoku solver (simplified 4x4)
sudoku_4x4(Puzzle) :-
    Puzzle = [S11, S12, S13, S14,
              S21, S22, S23, S24,
              S31, S32, S33, S34,
              S41, S42, S43, S44],
    Puzzle ins 1..4,
    
    % Row constraints
    all_different([S11, S12, S13, S14]),
    all_different([S21, S22, S23, S24]),
    all_different([S31, S32, S33, S34]),
    all_different([S41, S42, S43, S44]),
    
    % Column constraints
    all_different([S11, S21, S31, S41]),
    all_different([S12, S22, S32, S42]),
    all_different([S13, S23, S33, S43]),
    all_different([S14, S24, S34, S44]),
    
    % Box constraints
    all_different([S11, S12, S21, S22]),
    all_different([S13, S14, S23, S24]),
    all_different([S31, S32, S41, S42]),
    all_different([S33, S34, S43, S44]),
    
    label(Puzzle).

% Definite Clause Grammars (DCG)

% Simple sentence parser
sentence --> noun_phrase, verb_phrase.
noun_phrase --> determiner, noun.
noun_phrase --> noun.
verb_phrase --> verb, noun_phrase.
verb_phrase --> verb.

determiner --> [the].
determiner --> [a].
determiner --> [an].

noun --> [cat].
noun --> [dog].
noun --> [mouse].
noun --> [cheese].

verb --> [chases].
verb --> [sees].
verb --> [eats].

% Number parser
number(N) --> [N], { number(N) }.

% Expression parser
expr(X) --> term(X).
expr(X) --> term(Y), [+], expr(Z), { X is Y + Z }.
expr(X) --> term(Y), [-], expr(Z), { X is Y - Z }.

term(X) --> factor(X).
term(X) --> factor(Y), [*], term(Z), { X is Y * Z }.
term(X) --> factor(Y), [/], term(Z), { X is Y / Z }.

factor(X) --> number(X).
factor(X) --> ['('], expr(X), [')'].

% Meta-predicates and higher-order programming

% Map predicate over list
map_list(_, [], []).
map_list(Pred, [H|T], [MH|MT]) :-
    call(Pred, H, MH),
    map_list(Pred, T, MT).

% Filter list elements
filter_list(_, [], []).
filter_list(Pred, [H|T], [H|FT]) :-
    call(Pred, H), !,
    filter_list(Pred, T, FT).
filter_list(Pred, [_|T], FT) :-
    filter_list(Pred, T, FT).

% Fold (reduce) list
fold_left(_, Acc, [], Acc).
fold_left(Pred, Acc, [H|T], Result) :-
    call(Pred, Acc, H, NewAcc),
    fold_left(Pred, NewAcc, T, Result).

% Utility predicates for higher-order operations
double(X, Y) :- Y is X * 2.
square(X, Y) :- Y is X * X.
is_even(X) :- X mod 2 =:= 0.
add(X, Y, Z) :- Z is X + Y.

% Database manipulation

% Assert and retract facts dynamically
add_fact(Fact) :-
    assertz(fact(Fact)).

remove_fact(Fact) :-
    retract(fact(Fact)).

list_facts(Facts) :-
    findall(F, fact(F), Facts).

% Puzzle solving

% Tower of Hanoi
hanoi(1, A, B, _) :-
    format('Move disk from ~w to ~w~n', [A, B]).
hanoi(N, A, B, C) :-
    N > 1,
    N1 is N - 1,
    hanoi(N1, A, C, B),
    hanoi(1, A, B, _),
    hanoi(N1, C, B, A).

% Missionaries and Cannibals problem
solve_missionaries_cannibals(Solution) :-
    solve_mc(state(3,3,left), state(0,0,right), [state(3,3,left)], Solution).

solve_mc(State, State, Path, Path).
solve_mc(State, Goal, Visited, Solution) :-
    move_mc(State, NextState),
    \+ member_check(NextState, Visited),
    solve_mc(NextState, Goal, [NextState|Visited], Solution).

move_mc(state(M1,C1,left), state(M2,C2,right)) :-
    move_people(DM, DC),
    M2 is M1 - DM,
    C2 is C1 - DC,
    valid_state(state(M2,C2,right)),
    valid_state(state(M1,C1,left)).

move_mc(state(M1,C1,right), state(M2,C2,left)) :-
    move_people(DM, DC),
    M2 is M1 + DM,
    C2 is C1 + DC,
    valid_state(state(M2,C2,left)),
    valid_state(state(M1,C1,right)).

move_people(2, 0).
move_people(1, 0).
move_people(1, 1).
move_people(0, 1).
move_people(0, 2).

valid_state(state(M, C, _)) :-
    M >= 0, C >= 0, M =< 3, C =< 3,
    (M >= C ; M = 0),
    M2 is 3 - M, C2 is 3 - C,
    (M2 >= C2 ; M2 = 0).

% Test cases and examples

test_case(factorial, [
    factorial(0, 1),
    factorial(5, 120),
    factorial(10, 3628800)
]).

test_case(fibonacci, [
    fibonacci(0, 0),
    fibonacci(1, 1),
    fibonacci(10, 55)
]).

test_case(list_operations, [
    append_list([1,2], [3,4], [1,2,3,4]),
    reverse_list([1,2,3,4], [4,3,2,1]),
    sum_list([1,2,3,4,5], 15)
]).

test_case(family_relations, [
    parent(john, alice),
    grandparent(john, diana),
    ancestor(john, frank)
]).

% Run all tests
run_tests :-
    test_case(Category, Tests),
    format('Testing ~w:~n', [Category]),
    run_test_list(Tests),
    nl,
    fail.
run_tests.

run_test_list([]).
run_test_list([Test|Tests]) :-
    (call(Test) ->
        format('  PASS: ~w~n', [Test])
    ;   format('  FAIL: ~w~n', [Test])
    ),
    run_test_list(Tests).

% Main demonstration predicate
demonstrate_prolog :-
    format('=== Prolog Comprehensive Test ===~n'),
    
    % Test basic facts and rules
    format('Family relationships:~n'),
    forall(parent(X, Y), format('  ~w is parent of ~w~n', [X, Y])),
    
    format('~nGrandparents:~n'),
    forall(grandparent(X, Y), format('  ~w is grandparent of ~w~n', [X, Y])),
    
    % Test mathematical predicates
    format('~nMathematical operations:~n'),
    factorial(5, F5),
    format('  5! = ~w~n', [F5]),
    fibonacci(10, Fib10),
    format('  fibonacci(10) = ~w~n', [Fib10]),
    
    % Test list operations
    format('~nList operations:~n'),
    append_list([1,2,3], [4,5,6], AppendResult),
    format('  append([1,2,3], [4,5,6]) = ~w~n', [AppendResult]),
    reverse_list([1,2,3,4,5], ReverseResult),
    format('  reverse([1,2,3,4,5]) = ~w~n', [ReverseResult]),
    quicksort([5,2,8,1,9,3], SortResult),
    format('  quicksort([5,2,8,1,9,3]) = ~w~n', [SortResult]),
    
    % Test tree operations
    format('~nBinary tree operations:~n'),
    Tree = tree(tree(empty, 2, empty), 5, tree(empty, 8, empty)),
    inorder(Tree, InorderResult),
    format('  inorder traversal = ~w~n', [InorderResult]),
    
    % Test graph operations
    format('~nGraph operations:~n'),
    (path(a, d, Path) ->
        format('  path from a to d: ~w~n', [Path])
    ;   format('  no path from a to d~n')
    ),
    
    % Test constraint solving
    format('~nConstraint solving:~n'),
    (n_queens(4, Queens) ->
        format('  4-queens solution: ~w~n', [Queens])
    ;   format('  no 4-queens solution found~n')
    ),
    
    % Test DCG parsing
    format('~nNatural language parsing:~n'),
    Sentence = [the, cat, chases, a, mouse],
    (phrase(sentence, Sentence) ->
        format('  "~w" is a valid sentence~n', [Sentence])
    ;   format('  "~w" is not a valid sentence~n', [Sentence])
    ),
    
    % Test higher-order predicates
    format('~nHigher-order operations:~n'),
    map_list(double, [1,2,3,4,5], Doubled),
    format('  double([1,2,3,4,5]) = ~w~n', [Doubled]),
    filter_list(is_even, [1,2,3,4,5,6], Evens),
    format('  filter_even([1,2,3,4,5,6]) = ~w~n', [Evens]),
    
    format('~n=== Test completed ===~n').

% Entry point for testing
:- initialization(demonstrate_prolog).
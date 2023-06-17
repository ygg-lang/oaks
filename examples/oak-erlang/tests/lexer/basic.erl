% Erlang test file for lexer testing
% Comments start with percent sign

%% Module definition with module attribute
-module(math_operations).
-author('Test Author').
-version('1.0').

%% Export functions
-export([factorial/1, fibonacci/1, list_sum/1, map/2, filter/2]).
-export([start/0, stop/0, process_message/1]).

%% Type specifications
-type number() :: integer() | float().
-type list_of(T) :: [T].

%% Constants
-define(PI, 3.14159265359).
-define(MAX_VALUE, 1000).

%% Record definitions
-record(person, {
    name :: string(),
    age :: integer(),
    email :: string() | undefined
}).

-record(car, {
    make :: string(),
    model :: string(),
    year :: integer(),
    color = "red" :: string()
}).

%% Function definitions

% Basic function with pattern matching
factorial(0) -> 1;
factorial(N) when N > 0 -> N * factorial(N - 1).

% Fibonacci sequence
fibonacci(0) -> 0;
fibonacci(1) -> 1;
fibonacci(N) when N > 1 -> fibonacci(N - 1) + fibonacci(N - 2).

% List operations with pattern matching
list_sum([]) -> 0;
list_sum([Head | Tail]) -> Head + list_sum(Tail).

% Map function implementation
map(_, []) -> [];
map(Function, [Head | Tail]) -> [Function(Head) | map(Function, Tail)].

% Filter function implementation
filter(_, []) -> [];
filter(Predicate, [Head | Tail]) ->
    case Predicate(Head) of
        true -> [Head | filter(Predicate, Tail)];
        false -> filter(Predicate, Tail)
    end.

% Function with multiple clauses and guards
absolute_value(X) when X >= 0 -> X;
absolute_value(X) when X < 0 -> -X.

% List comprehensions
double_numbers(List) -> [X * 2 || X <- List, X > 0].

square_numbers(List) -> [X * X || X <- List, is_integer(X)].

% Binary comprehensions
binary_to_list(Binary) -> [X || <<X:8>> <= Binary].

% Case expression
process_value(Value) ->
    case Value of
        0 -> zero;
        N when N > 0 -> positive;
        N when N < 0 -> negative;
        _ -> unknown
    end.

% If expression
get_grade(Score) ->
    if
        Score >= 90 -> 'A';
        Score >= 80 -> 'B';
        Score >= 70 -> 'C';
        Score >= 60 -> 'D';
        true -> 'F'
    end.

% Try-catch expression
safe_divide(A, B) ->
    try A / B of
        Result -> {ok, Result}
    catch
        error:badarith -> {error, division_by_zero};
        error:Error -> {error, Error}
    after
        io:format("Division attempted with ~p and ~p~n", [A, B])
    end.

% Receive expression with pattern matching
message_loop() ->
    receive
        {sender, hello} ->
            sender ! {self(), hi},
            message_loop();
        {sender, goodbye} ->
            sender ! {self(), bye},
            message_loop();
        stop ->
            io:format("Stopping message loop~n");
        Unknown ->
            io:format("Unknown message: ~p~n", [Unknown]),
            message_loop()
    after
        5000 ->
            io:format("Timeout occurred~n"),
            message_loop()
    end.

% Concurrency primitives
start() ->
    spawn(fun() -> init_process() end).

stop() ->
    exit(normal).

init_process() ->
    process_flag(trap_exit, true),
    loop().

loop() ->
    receive
        {'EXIT', Pid, Reason} ->
            io:format("Process ~p exited with reason ~p~n", [Pid, Reason]),
            loop();
        Message ->
            process_message(Message),
            loop()
    end.

process_message({From, Data}) ->
    From ! {self(), processed, Data};
process_message(Unknown) ->
    io:format("Unknown message format: ~p~n", [Unknown]).

% Working with records
create_person(Name, Age) ->
    #person{name = Name, age = Age}.

update_person_email(Person, Email) ->
    Person#person{email = Email}.

get_person_name(#person{name = Name}) -> Name.

% Binary pattern matching
parse_binary(<<A:8, B:8, Rest/binary>>) ->
    {A, B, Rest};
parse_binary(<<A:16, Rest/binary>>) ->
    {A, Rest};
parse_binary(_) ->
    {error, invalid_format}.

% Bit syntax
extract_bits(<<Header:4, Type:2, Flags:2, Data/binary>>) ->
    {Header, Type, Flags, Data}.

% Fun expressions (anonymous functions)
square = fun(X) -> X * X end.
double = fun(X) -> X * X * 2 end.

% Higher-order functions
apply_twice(Fun, Value) -> Fun(Fun(Value)).

compose(F, G) -> fun(X) -> F(G(X)) end.

% Module with behaviors
-module(my_behavior).
-callback init(Args :: term()) -> {ok, State :: term()} | {error, Reason :: term()}.
-callback handle_call(Request :: term(), From :: term(), State :: term()) ->
    {reply, Reply :: term(), NewState :: term()} |
    {noreply, NewState :: term()} |
    {stop, Reason :: term(), NewState :: term()}.

% OTP Application
-module(my_app).
-behaviour(application).

-export([start/2, stop/1]).

start(_StartType, _StartArgs) ->
    my_sup:start_link().

stop(_State) ->
    ok.

% Include files
-include("header.hrl").
-include_lib("kernel/include/file.hrl").
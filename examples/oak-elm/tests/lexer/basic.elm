# Elixir test file for lexer testing

# Module definition with documentation
defmodule MathOperations do
  @moduledoc """
  Provides basic mathematical operations and utilities.
  """

  @pi 3.14159265359

  # Function with guard clauses
  def absolute_value(x) when x >= 0, do: x
  def absolute_value(x) when x < 0, do: -x

  # Function with default parameters
  def greet(name, greeting \\ "Hello") do
    "#{greeting}, #{name}!"
  end

  # Recursive function
  def factorial(0), do: 1
  def factorial(n) when n > 0, do: n * factorial(n - 1)

  # Higher-order function
  def apply_operation(a, b, operation) do
    operation.(a, b)
  end

  # Pipe operator usage
  def process_data(data) do
    data
    |> Enum.map(&(&1 * 2))
    |> Enum.filter(&(&1 > 10))
    |> Enum.sum()
  end
end

# Struct definition
defmodule Person do
  defstruct [:name, :age, :email]
end

# Protocol definition
defprotocol Stringifiable do
  def to_string(data)
end

# Protocol implementation
defimpl Stringifiable, for: Person do
  def to_string(%Person{name: name, age: age}) do
    "#{name} (#{age} years old)"
  end
end

# GenServer behavior
defmodule Counter do
  use GenServer

  # Client API
  def start_link(initial_value \\ 0) do
    GenServer.start_link(__MODULE__, initial_value, name: __MODULE__)
  end

  def increment do
    GenServer.cast(__MODULE__, :increment)
  end

  def get_count do
    GenServer.call(__MODULE__, :get_count)
  end

  # Server callbacks
  @impl true
  def init(initial_value) do
    {:ok, initial_value}
  end

  @impl true
  def handle_cast(:increment, count) do
    {:noreply, count + 1}
  end

  @impl true
  def handle_call(:get_count, _from, count) do
    {:reply, count, count}
  end
end

# Supervisor
defmodule MyApp.Supervisor do
  use Supervisor

  def start_link(init_arg) do
    Supervisor.start_link(__MODULE__, init_arg, name: __MODULE__)
  end

  @impl true
  def init(_init_arg) do
    children = [
      {Counter, 0},
      {Task.Supervisor, name: MyApp.TaskSupervisor}
    ]

    Supervisor.init(children, strategy: :one_for_one)
  end
end

# Agent usage
defmodule BankAccount do
  def start_link(initial_balance \\ 0) do
    Agent.start_link(fn -> initial_balance end, name: __MODULE__)
  end

  def deposit(amount) do
    Agent.update(__MODULE__, fn balance -> balance + amount end)
  end

  def balance do
    Agent.get(__MODULE__, fn balance -> balance end)
  end
end

# Task async/await
defmodule AsyncExample do
  def long_running_task do
    Task.async(fn ->
      Process.sleep(1000)
      "Task completed!"
    end)
  end

  def run_async do
    task = long_running_task()
    result = Task.await(task)
    IO.puts(result)
  end
end

# Stream processing
defmodule DataProcessor do
  def process_large_dataset do
    1..1_000_000
    |> Stream.map(&(&1 * 2))
    |> Stream.filter(&(&1 > 100))
    |> Stream.take(10)
    |> Enum.to_list()
  end
end

# Pattern matching with lists
defmodule ListOperations do
  def sum_list([]), do: 0
  def sum_list([head | tail]), do: head + sum_list(tail)

  def reverse_list(list), do: reverse_list(list, [])
  defp reverse_list([], acc), do: acc
  defp reverse_list([head | tail], acc), do: reverse_list(tail, [head | acc])
end

# Map operations
defmodule MapExamples do
  def create_user do
    %{
      name: "John Doe",
      age: 30,
      address: %{
        street: "123 Main St",
        city: "Anytown",
        zip: "12345"
      }
    }
  end

  def update_user(user) do
    user
    |> Map.put(:email, "john@example.com")
    |> Map.update!(:age, &(&1 + 1))
  end
end

# Comprehensions
defmodule Comprehensions do
  def generate_pairs do
    for x <- 1..3, y <- 1..3, x != y, do: {x, y}
  end

  def process_with_filters do
    for n <- 1..100,
        rem(n, 2) == 0,
        n > 50,
        do: n * n
  end
end

# Exception handling
defmodule ErrorHandling do
  def safe_divide(a, 0), do: {:error, "Division by zero"}
  def safe_divide(a, b), do: {:ok, a / b}

  def process_with_rescue do
    try do
      String.to_integer("not a number")
    rescue
      ArgumentError -> {:error, "Invalid number format"}
    catch
      :throw, value -> {:thrown, value}
    else
      number -> {:success, number}
    after
      IO.puts("Cleanup code executed")
    end
  end
end

# Behaviours
defmodule Worker do
  @callback init(args :: term()) :: {:ok, state :: term()} | {:error, reason :: term()}
  @callback handle_work(work :: term(), state :: term()) :: {:ok, result :: term(), new_state :: term()}
end

# Macro definition
defmodule MyMacros do
  defmacro unless(condition, clauses) do
    quote do
      if(!unquote(condition), unquote(clauses))
    end
  end

  defmacro log_execution(expression) do
    quote do
      IO.puts("Executing: #{unquote(Macro.to_string(expression))}")
      unquote(expression)
    end
  end
end

# Import and alias usage
defmodule ModuleUsage do
  import String, only: [upcase: 1, downcase: 1]
  alias Enum, as: E

  def process_text(text) do
    text
    |> upcase()
    |> String.split()
    |> E.map(&downcase/1)
  end
end

# Registry usage
defmodule ProcessRegistry do
  def start_link do
    Registry.start_link(keys: :unique, name: MyRegistry)
  end

  def register_process(name, pid) do
    Registry.register(MyRegistry, name, pid)
  end

  def lookup_process(name) do
    case Registry.lookup(MyRegistry, name) do
      [{pid, _}] -> {:ok, pid}
      [] -> {:error, :not_found}
    end
  end
end
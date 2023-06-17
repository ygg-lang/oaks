def fibonacci(n):
    """Calculate fibonacci number using control flow."""
    if n <= 0:
        return 0
    elif n == 1:
        return 1
    else:
        return fibonacci(n-1) + fibonacci(n-2)

def process_numbers(numbers):
    """Process a list of numbers with various control structures."""
    result = []
    
    for num in numbers:
        if num < 0:
            continue
        elif num == 0:
            break
        else:
            result.append(num * 2)
    
    return result

def count_down(start):
    """Count down using while loop."""
    current = start
    while current > 0:
        print(f"Count: {current}")
        current -= 1
    
    print("Done!")

# Exception handling
try:
    value = int(input("Enter a number: "))
    result = 10 / value
    print(f"Result: {result}")
except ValueError:
    print("Invalid input!")
except ZeroDivisionError:
    print("Cannot divide by zero!")
finally:
    print("Cleanup complete")
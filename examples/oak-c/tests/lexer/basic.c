#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Function prototype
int factorial(int n);

// Global variable
static int counter = 0;

// Structure definition
typedef struct {
    int x;
    int y;
    char name[50];
} Point;

// Enum definition
typedef enum {
    RED = 0,
    GREEN = 1,
    BLUE = 2
} Color;

// Main function
int main(int argc, char *argv[]) {
    // Variable declarations
    int number = 5;
    Point p1 = {10, 20, "Origin"};
    Color c = RED;
    
    // Pointer usage
    int *ptr = &number;
    
    // Conditional statements
    if (number > 0) {
        printf("Positive number\n");
    } else if (number < 0) {
        printf("Negative number\n");
    } else {
        printf("Zero\n");
    }
    
    // Loop statements
    for (int i = 0; i < number; i++) {
        printf("Iteration %d\n", i);
    }
    
    // Function call
    int result = factorial(number);
    printf("Factorial of %d is %d\n", number, result);
    
    // Memory allocation
    int *array = (int *)malloc(number * sizeof(int));
    if (array != NULL) {
        for (int i = 0; i < number; i++) {
            array[i] = i * i;
        }
        free(array);
    }
    
    return 0;
}

// Function definition
int factorial(int n) {
    if (n <= 1) {
        return 1;
    }
    return n * factorial(n - 1);
}
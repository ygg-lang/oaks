/* Comprehensive C Language Lexer Test */
/* Covers C99, C11, and some C23 features */

#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

// Macros
#define MAX_BUFFER_SIZE 1024
#define SQUARE(x) ((x) * (x))
#define CONCAT(a, b) a ## b
#define STRINGIFY(x) #x

// Typedefs and Structs
typedef struct Point {
    int x;
    int y;
    struct Point* next;
} Point;

typedef union Data {
    int i;
    float f;
    char str[20];
} Data;

// Enums
enum Color {
    RED,
    GREEN,
    BLUE = 10
};

// Global Variables
static int counter = 0;
const double PI = 3.1415926535;
extern void helper_function(void);

// Function Prototypes
int add(int a, int b);
void print_point(Point p);

// Main Function
int main(int argc, char* argv[]) {
    // Variables
    int a = 10;
    long b = 20000L;
    unsigned int c = 30U;
    float f = 1.5f;
    double d = 2.5;
    char ch = 'A';
    char* str = "Hello, World!";
    
    // Arrays
    int numbers[5] = {1, 2, 3, 4, 5};
    int matrix[2][2] = {{1, 0}, {0, 1}};

    // Pointers
    int* ptr = &a;
    *ptr = 20;

    // Dynamic Memory
    Point* p = (Point*)malloc(sizeof(Point));
    if (p == NULL) {
        fprintf(stderr, "Memory allocation failed\n");
        return 1;
    }
    p->x = 10;
    p->y = 20;

    // Control Flow
    if (a > 5) {
        printf("Greater than 5\n");
    } else if (a == 5) {
        printf("Equal to 5\n");
    } else {
        printf("Less than 5\n");
    }

    // Loops
    for (int i = 0; i < 5; i++) {
        printf("%d ", i);
    }

    while (counter < 10) {
        counter++;
    }

    do {
        counter--;
    } while (counter > 0);

    // Switch Case
    switch (ch) {
        case 'A':
            printf("Alpha\n");
            break;
        case 'B':
            printf("Bravo\n");
            break;
        default:
            printf("Other\n");
    }

    // Bitwise Operators
    int x = 0x0F;
    int y = 0xF0;
    int z = x & y | (x ^ y) << 1 >> 1;
    int not_x = ~x;

    // Logical Operators
    bool is_valid = (a > 0) && (b > 0) || !false;

    // Ternary Operator
    int min = (a < b) ? a : b;

    // Goto
    goto cleanup;

cleanup:
    free(p);
    return 0;
}

// Function Definition
int add(int a, int b) {
    return a + b;
}

// Variadic Functions
void log_message(const char* format, ...) {
    // Implementation omitted
}

// Attributes (GNU C / C23)
[[nodiscard]] int compute(void) {
    return 42;
}

// Complex Types
_Complex double cplx = 1.0 + 2.0 * _Complex_I;

// Hex, Octal, Binary (C23) literals
int hex = 0xFF;
int oct = 0777;
int bin = 0b1010;

// Raw String Literals (C++ style, but sometimes supported in extensions)
// char* raw = R"(Raw string)";

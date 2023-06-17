int main() {
    // Arithmetic operators
    int a = 10 + 5;
    int b = 20 - 3;
    int c = 4 * 6;
    int d = 15 / 3;
    int e = 17 % 4;
    
    // Assignment operators
    a += 5;
    b -= 2;
    c *= 3;
    d /= 2;
    e %= 3;
    
    // Comparison operators
    if (a == b) return 1;
    if (a != b) return 2;
    if (a < b) return 3;
    if (a > b) return 4;
    if (a <= b) return 5;
    if (a >= b) return 6;
    
    // Logical operators
    if (a && b) return 7;
    if (a || b) return 8;
    if (!a) return 9;
    
    // Bitwise operators
    int f = a & b;
    int g = a | b;
    int h = a ^ b;
    int i = ~a;
    int j = a << 2;
    int k = a >> 1;
    
    // Increment/decrement
    ++a;
    --b;
    c++;
    d--;
    
    // Pointer operators
    int *ptr = &a;
    int value = *ptr;
    
    // Ternary operator
    int result = (a > b) ? a : b;
    
    return 0;
}
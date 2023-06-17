#include <stdio.h>

int main() {
    int n = 10;
    
    // if-else statement
    if (n > 0) {
        printf("Positive\n");
    } else if (n < 0) {
        printf("Negative\n");
    } else {
        printf("Zero\n");
    }
    
    // switch statement
    switch (n % 3) {
        case 0:
            printf("Divisible by 3\n");
            break;
        case 1:
            printf("Remainder 1\n");
            break;
        case 2:
            printf("Remainder 2\n");
            break;
        default:
            printf("Unknown\n");
            break;
    }
    
    // for loop
    for (int i = 0; i < n; i++) {
        printf("%d ", i);
    }
    printf("\n");
    
    // while loop
    int j = 0;
    while (j < 5) {
        printf("j = %d\n", j);
        j++;
    }
    
    // do-while loop
    int k = 0;
    do {
        printf("k = %d\n", k);
        k++;
    } while (k < 3);
    
    // goto statement
    if (n == 10) {
        goto end;
    }
    
    printf("This won't be printed\n");
    
end:
    printf("End of program\n");
    return 0;
}
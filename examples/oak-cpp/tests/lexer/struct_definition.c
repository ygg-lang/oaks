#include <stdio.h>
#include <string.h>

// Simple struct
struct Point {
    int x;
    int y;
};

// Struct with different types
struct Person {
    char name[50];
    int age;
    float height;
    double weight;
};

// Nested struct
struct Address {
    char street[100];
    char city[50];
    int zip_code;
};

struct Employee {
    struct Person info;
    struct Address address;
    int employee_id;
    float salary;
};

// Union
union Data {
    int i;
    float f;
    char str[20];
};

// Enum
enum Color {
    RED,
    GREEN,
    BLUE,
    YELLOW = 10,
    PURPLE
};

// Typedef
typedef struct {
    int width;
    int height;
} Rectangle;

typedef enum {
    MONDAY,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY,
    SUNDAY
} Weekday;

int main() {
    // Initialize struct
    struct Point p1 = {10, 20};
    struct Point p2 = {.x = 5, .y = 15};
    
    // Access struct members
    printf("Point 1: (%d, %d)\n", p1.x, p1.y);
    printf("Point 2: (%d, %d)\n", p2.x, p2.y);
    
    // Struct with strings
    struct Person person;
    strcpy(person.name, "John Doe");
    person.age = 30;
    person.height = 5.9f;
    person.weight = 70.5;
    
    printf("Person: %s, Age: %d, Height: %.1f, Weight: %.1f\n",
           person.name, person.age, person.height, person.weight);
    
    // Union usage
    union Data data;
    data.i = 10;
    printf("data.i = %d\n", data.i);
    
    data.f = 220.5f;
    printf("data.f = %.1f\n", data.f);
    
    strcpy(data.str, "C Programming");
    printf("data.str = %s\n", data.str);
    
    // Enum usage
    enum Color favorite_color = BLUE;
    printf("Favorite color: %d\n", favorite_color);
    
    // Typedef usage
    Rectangle rect = {800, 600};
    printf("Rectangle: %dx%d\n", rect.width, rect.height);
    
    Weekday today = FRIDAY;
    printf("Today is day %d of the week\n", today);
    
    return 0;
}
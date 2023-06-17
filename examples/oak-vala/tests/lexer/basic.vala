// Vala test file
using GLib;

public class Person : Object {
    public string name { get; set; }
    public int age { get; set; }
    
    public Person(string name, int age) {
        this.name = name;
        this.age = age;
    }
    
    public void greet() {
        stdout.printf("Hello, I'm %s and I'm %d years old.\n", name, age);
    }
}

public interface Drawable {
    public abstract void draw();
}

public class Circle : Object, Drawable {
    public double radius { get; set; }
    
    public Circle(double radius) {
        this.radius = radius;
    }
    
    public void draw() {
        stdout.printf("Drawing a circle with radius %.2f\n", radius);
    }
    
    public double area() {
        return Math.PI * radius * radius;
    }
}

public enum Color {
    RED,
    GREEN,
    BLUE
}

public struct Point {
    public double x;
    public double y;
    
    public Point(double x, double y) {
        this.x = x;
        this.y = y;
    }
    
    public double distance_to(Point other) {
        double dx = x - other.x;
        double dy = y - other.y;
        return Math.sqrt(dx * dx + dy * dy);
    }
}

public static int main(string[] args) {
    var person = new Person("Alice", 25);
    person.greet();
    
    var circle = new Circle(5.0);
    circle.draw();
    stdout.printf("Circle area: %.2f\n", circle.area());
    
    var point1 = Point(0.0, 0.0);
    var point2 = Point(3.0, 4.0);
    stdout.printf("Distance: %.2f\n", point1.distance_to(point2));
    
    // Arrays
    int[] numbers = {1, 2, 3, 4, 5};
    foreach (int num in numbers) {
        stdout.printf("%d ", num);
    }
    stdout.printf("\n");
    
    // Hash table
    var map = new HashTable<string, int>(str_hash, str_equal);
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    
    stdout.printf("Value for 'two': %d\n", map.lookup("two"));
    
    return 0;
}
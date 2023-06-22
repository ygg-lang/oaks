/*
 * Comprehensive Java Test File for Lexer Testing
 * This file includes a wide range of Java syntax elements to ensure thorough lexer coverage.
 */

package com.example.lexer;

import java.io.Serializable;
import java.math.BigDecimal;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.Optional;
import java.util.Set;
import java.util.concurrent.Callable;
import java.util.function.Consumer;
import java.util.function.Function;
import java.util.stream.Collectors;

// Single-line comment
/* Multi-line
 * comment
 */
/** Javadoc comment */

@SuppressWarnings({"unchecked", "rawtypes"})
@Deprecated
public final class JavaLexerTest<T extends Number & Comparable<T>> implements Serializable, MyInterface {

    // --- Fields ---
    public static final int MAX_VALUE = 100;
    private volatile String name;
    protected transient T value;
    final BigDecimal price = new BigDecimal("19.99");
    private static List<String> staticList = new ArrayList<>();
    private Map<String, ? extends Number> dataMap;

    // --- Constructors ---
    public JavaLexerTest() {
        this("Default", null);
    }

    public JavaLexerTest(String name, T value) {
        this.name = name;
        this.value = value;
        this.dataMap = new HashMap<>();
    }

    // --- Methods ---

    /**
     * A sample method demonstrating various features.
     * @param input A string input.
     * @param count An integer count.
     * @return A processed string.
     * @throws IllegalArgumentException if input is null.
     */
    public synchronized String processData(String input, int count) throws IllegalArgumentException {
        if (input == null) {
            throw new IllegalArgumentException("Input cannot be null");
        }

        // Local variables
        int sum = 0;
        double average = 0.0;
        boolean isActive = true;
        char initial = 'J';
        long timestamp = System.currentTimeMillis();
        float ratio = 0.5f;
        short sVal = 10;
        byte bVal = 0xA;

        // Control structures
        for (int i = 0; i < count; i++) {
            sum += i;
            if (i % 2 == 0) {
                System.out.println("Even: " + i);
            } else if (i % 3 == 0) {
                System.out.println("Divisible by 3: " + i);
            } else {
                System.out.println("Odd: " + i);
            }
        }

        while (isActive && sum > 0) {
            sum--;
            if (sum == 5) {
                break;
            }
            if (sum == 3) {
                continue;
            }
        }

        do {
            average = (double) sum / count;
        } while (false);

        switch (initial) {
            case 'A':
            case 'B':
                System.out.println("Starts with A or B");
                break;
            case 'J':
                System.out.println("Starts with J");
                break;
            default:
                System.out.println("Other initial");
        }

        // Operators
        int x = 10;
        int y = 20;
        int result = x + y * 2 - (y / x) % 3;
        boolean comparison = (x > y) || (x != y) && (x <= y);
        x++;
        y--;
        result <<= 2;
        result &= 0xFF;

        // Arrays
        int[] numbers = {1, 2, 3, 4, 5};
        String[][] matrix = new String[2][2];
        matrix[0][0] = "Hello";

        // Enhanced for loop
        for (int num : numbers) {
            System.out.print(num + " ");
        }
        System.out.println();

        // Try-catch-finally
        try {
            int div = 10 / 0;
        } catch (ArithmeticException | NullPointerException e) {
            System.err.println("Error: " + e.getMessage());
        } finally {
            System.out.println("Finally block executed.");
        }

        // Lambda expressions and functional interfaces
        Consumer<String> printer = msg -> System.out.println("Lambda: " + msg);
        printer.accept(input);

        Function<Integer, String> intToString = i -> String.valueOf(i);
        String numStr = intToString.apply(count);

        Callable<String> asyncTask = () -> {
            Thread.sleep(100);
            return "Task Done";
        };

        // Method references
        List<String> names = List.of("Alice", "Bob", "Charlie");
        names.forEach(System.out::println);

        // Generics
        List<T> genericList = new ArrayList<>();
        genericList.add(value);

        // Enums
        DayOfWeek today = DayOfWeek.MONDAY;
        System.out.println("Today is " + today);

        // Annotations
        @Override
        String processed = input.toUpperCase() + "-" + count;

        // Ternary operator
        String status = (count > 10) ? "High" : "Low";

        // instanceof and casting
        Object obj = "Test String";
        if (obj instanceof String) {
            String str = (String) obj;
            System.out.println("Casted string: " + str);
        }

        // Static import
        System.out.println("Max value: " + MAX_VALUE);

        // Varargs
        printMessages("Msg1", "Msg2", "Msg3");

        // Assertions
        assert count > 0 : "Count must be positive";

        // Optional
        Optional<String> optionalName = Optional.ofNullable(name);
        optionalName.ifPresent(n -> System.out.println("Name is: " + n));

        // Streams API
        List<Integer> numbersList = List.of(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
        Set<Integer> evenNumbers = numbersList.stream()
                                            .filter(num -> num % 2 == 0)
                                            .map(num -> num * 2)
                                            .collect(Collectors.toSet());
        System.out.println("Even numbers doubled: " + evenNumbers);

        // Records (Java 16+)
        record Point(int x, int y) {}
        Point p = new Point(10, 20);
        System.out.println("Point: " + p.x() + ", " + p.y());

        // Sealed classes (Java 17+)
        // public sealed interface Shape permits Circle, Rectangle {}
        // public final class Circle implements Shape {}
        // public non-sealed class Rectangle implements Shape {}

        // Pattern Matching for instanceof (Java 16+)
        Object anotherObj = "Hello";
        if (anotherObj instanceof String s) {
            System.out.println("Pattern matched string: " + s.length());
        }

        // Text Blocks (Java 15+)
        String textBlock = """
            This is a
            multi-line
            text block.
            """;
        System.out.println(textBlock);

        // Switch Expressions (Java 14+)
        String dayType = switch (today) {
            case MONDAY, TUESDAY, WEDNESDAY, THURSDAY, FRIDAY -> "Weekday";
            case SATURDAY, SUNDAY -> "Weekend";
        };
        System.out.println("Day type: " + dayType);

        // Var keyword (Java 10+)
        var message = "Inferred type string";
        var countList = new ArrayList<Integer>();
        countList.add(1);
        System.out.println(message + ", list size: " + countList.size());

        return processed;
    }

    private void printMessages(String... messages) {
        for (String msg : messages) {
            System.out.println("Vararg message: " + msg);
        }
    }

    // --- Nested Class ---
    private static class InnerClass {
        private int innerValue;

        public InnerClass(int innerValue) {
            this.innerValue = innerValue;
        }

        public int getInnerValue() {
            return innerValue;
        }
    }

    // --- Interface Implementation ---
    @Override
    public void myMethod() {
        System.out.println("Implementing myMethod");
    }

    // --- Enum Declaration ---
    public enum DayOfWeek {
        MONDAY, TUESDAY, WEDNESDAY, THURSDAY, FRIDAY, SATURDAY, SUNDAY
    }

    // --- Main method for execution ---
    public static void main(String[] args) {
        JavaLexerTest<Integer> test = new JavaLexerTest<>("MainTest", 123);
        try {
            test.processData("Sample", 15);
        } catch (IllegalArgumentException e) {
            System.err.println("Caught exception in main: " + e.getMessage());
        }

        InnerClass inner = new InnerClass(42);
        System.out.println("Inner class value: " + inner.getInnerValue());

        test.myMethod();

        // Accessing static field
        staticList.add("Static Item 1");
        System.out.println("Static list: " + staticList);

        // Null handling with Objects.requireNonNull
        String nonNullString = Objects.requireNonNull(test.name, "Name must not be null");
        System.out.println("Non-null name: " + nonNullString);

        // Array of objects
        JavaLexerTest[] tests = new JavaLexerTest[2];
        tests[0] = new JavaLexerTest("ArrayTest1", 1);
        tests[1] = new JavaLexerTest("ArrayTest2", 2);

        // Type inference with diamond operator
        List<String> inferredList = new ArrayList<>();
        inferredList.add("Inferred");

        // Hex, binary, octal literals
        int hex = 0xFF;
        int binary = 0b1010;
        int octal = 012;
        System.out.println("Hex: " + hex + ", Binary: " + binary + ", Octal: " + octal);

        // Underscores in numeric literals
        long bigNumber = 1_000_000_000L;
        double scientific = 1.23e-5;
        System.out.println("Big number: " + bigNumber + ", Scientific: " + scientific);

        // Unicode escapes
        String unicode = "\u0048\u0065\u006C\u006C\u006F"; // Hello
        System.out.println("Unicode: " + unicode);

        // Raw string literals (if supported by future Java versions)
        // String rawString = `This is a raw string`;

        // Module declarations (Java 9+)
        // module com.example.mymodule {
        //     requires java.base;
        //     exports com.example.lexer;
        // }
    }
}

// --- Interface Declaration ---
interface MyInterface {
    void myMethod();
}

// --- Abstract Class ---
abstract class AbstractClass {
    abstract void abstractMethod();
    void concreteMethod() {
        System.out.println("Concrete method in abstract class");
    }
}

// --- Class extending AbstractClass ---
class ConcreteClass extends AbstractClass {
    @Override
    void abstractMethod() {
        System.out.println("Implementing abstract method");
    }
}

// --- Enum with methods and fields ---
enum Color {
    RED("#FF0000"), GREEN("#00FF00"), BLUE("#0000FF");

    private final String hexCode;

    Color(String hexCode) {
        this.hexCode = hexCode;
    }

    public String getHexCode() {
        return hexCode;
    }

    public static Color fromHex(String hex) {
        for (Color color : Color.values()) {
            if (color.getHexCode().equalsIgnoreCase(hex)) {
                return color;
            }
        }
        throw new IllegalArgumentException("Unknown hex code: " + hex);
    }
}

// --- Annotation Definition ---
@Retention(java.lang.annotation.RetentionPolicy.RUNTIME)
@Target({java.lang.annotation.ElementType.METHOD, java.lang.annotation.ElementType.TYPE})
@interface CustomAnnotation {
    String value() default "";
    int count() default 1;
}

// --- Example of using CustomAnnotation ---
@CustomAnnotation(value = "AnnotatedClass", count = 5)
class AnnotatedClass {
    @CustomAnnotation("AnnotatedMethod")
    public void annotatedMethod() {
        System.out.println("Method with custom annotation");
    }
}

// --- Generic Interface ---
interface GenericInterface<K, V> {
    K getKey();
    V getValue();
}

// --- Class implementing Generic Interface ---
class Pair<K, V> implements GenericInterface<K, V> {
    private K key;
    private V value;

    public Pair(K key, V value) {
        this.key = key;
        this.value = value;
    }

    @Override
    public K getKey() {
        return key;
    }

    @Override
    public V getValue() {
        return value;
    }
}

// --- Static Nested Class ---
class OuterClass {
    private static String outerStaticField = "Outer Static";
    private String outerInstanceField = "Outer Instance";

    static class StaticNestedClass {
        public void display() {
            System.out.println("Accessing outer static field: " + outerStaticField);
            // Cannot access outerInstanceField directly
        }
    }
}

// --- Inner Class ---
class AnotherOuterClass {
    private String outerMessage = "Hello from Outer";

    class InnerClass {
        public void display() {
            System.out.println("Accessing outer instance field: " + outerMessage);
        }
    }
}

// --- Anonymous Inner Class ---
interface Greeter {
    void greet();
}

class AnonymousInnerClassExample {
    public void createGreeter() {
        Greeter englishGreeter = new Greeter() {
            @Override
            public void greet() {
                System.out.println("Hello!");
            }
        };
        englishGreeter.greet();
    }
}

// --- Local Class ---
class LocalClassExample {
    public void createLocalClass() {
        class LocalGreeter implements Greeter {
            private String greeting = "Bonjour!";

            @Override
            public void greet() {
                System.out.println(greeting);
            }
        }
        LocalGreeter localGreeter = new LocalGreeter();
        localGreeter.greet();
    }
}

// --- Final Class ---
final class FinalClass {
    private final String message;

    public FinalClass(String message) {
        this.message = message;
    }

    public String getMessage() {
        return message;
    }
}

// --- Strictfp, Native, Synchronized, Volatile, Transient Keywords ---
class KeywordTest {
    public strictfp double calculate(double a, double b) {
        return a / b;
    }

    public native void nativeMethod();

    public synchronized void syncMethod() {
        // synchronized block
        synchronized (this) {
            System.out.println("Synchronized block");
        }
    }

    private volatile int volatileField;
    private transient String transientField;
}

// --- Module-related keywords (Java 9+) ---
// open module com.example.openmodule {
//     requires transitive java.sql;
//     exports com.example.api to com.example.client;
//     opens com.example.internal;
//     uses com.example.spi.MyService;
//     provides com.example.spi.MyService with com.example.impl.MyServiceImpl;
// }

// --- Record (Java 16+) ---
record User(long id, String username, String email) {
    // Compact constructor
    public User {
        Objects.requireNonNull(username);
        Objects.requireNonNull(email);
    }

    // Custom method
    public String getDomain() {
        return email.substring(email.indexOf("@") + 1);
    }
}

// --- Sealed Interface (Java 17+) ---
sealed interface Vehicle permits Car, Truck {
    String getType();
}

final class Car implements Vehicle {
    @Override
    public String getType() {
        return "Car";
    }
}

non-sealed class Truck implements Vehicle {
    @Override
    public String getType() {
        return "Truck";
    }
}

// --- Pattern Matching for Switch (Java 17+) ---
class SwitchPatternMatching {
    public String process(Object o) {
        return switch (o) {
            case Integer i -> String.format("Integer: %d", i);
            case String s && s.length() > 5 -> String.format("Long String: %s", s);
            case String s -> String.format("Short String: %s", s);
            case null -> "Null Object";
            default -> "Unknown Object";
        };
    }
}

// --- Foreign Function & Memory API (Preview in Java 19+) ---
import java.lang.foreign.*;
import java.lang.invoke.*;
class FFMAPIExample {
    public void callCFunction() throws Throwable {
        Linker linker = Linker.nativeLinker();
        MethodHandle strlen = linker.downcallHandle(
            linker.defaultLookup().find("strlen").get(),
            FunctionDescriptor.of(ValueLayout.JAVA_LONG, ValueLayout.ADDRESS)
        );
        try (MemorySegment str = MemorySegment.ofArray("Hello".getBytes())) {
            long len = (long) strlen.invokeExact(str);
            System.out.println("Length: " + len);
        }
    }
}

// --- Virtual Threads (Preview in Java 19+) ---
import java.util.concurrent.Executors;
class VirtualThreadsExample {
    public void runVirtualThread() {
        try (var executor = Executors.newVirtualThreadPerTaskExecutor()) {
            executor.submit(() -> {
                System.out.println("Running in virtual thread: " + Thread.currentThread());
            });
        }
    }
}

// --- Unnamed Classes and Instance Main Methods (Preview in Java 21+) ---
void main() {
    System.out.println("Hello from unnamed class!");
}

// --- Unnamed Patterns and Variables (Preview in Java 21+) ---
record Pair(int x, int y) {}
void processPair(Object o) {
    if (o instanceof Pair(int x, _)) {
        System.out.println("X value: " + x);
    }
}
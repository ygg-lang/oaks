// Dart test file for lexer testing

// Main function with async support
void main() async {
  print('Hello, Dart!');
  
  // Variable declarations with different types
  var name = 'Dart';
  String version = '3.0';
  int year = 2023;
  double pi = 3.14159;
  bool isAwesome = true;
  
  // Null safety
  String? nullableString;
  late String lateString;
  
  // Lists and Maps
  List<String> fruits = ['apple', 'banana', 'orange'];
  Map<String, int> scores = {
    'Alice': 95,
    'Bob': 87,
    'Charlie': 92,
  };
  
  // Functions
  int add(int a, int b) {
    return a + b;
  }
  
  // Arrow function
  int multiply(int x, int y) => x * y;
  
  // Anonymous function
  var greet = (String name) {
    print('Hello, $name!');
  };
  
  // Class definition
  class Person {
    String name;
    int age;
    
    // Constructor
    Person(this.name, this.age);
    
    // Named constructor
    Person.guest() : name = 'Guest', age = 0;
    
    // Method
    void introduce() {
      print('Hi, I\'m $name and I\'m $age years old.');
    }
    
    // Getter
    String get description => '$name ($age)';
  }
  
  // Inheritance
  class Student extends Person {
    String school;
    
    Student(String name, int age, this.school) : super(name, age);
    
    @override
    void introduce() {
      super.introduce();
      print('I study at $school.');
    }
  }
  
  // Mixin
  mixin Logger {
    void log(String message) {
      print('[${DateTime.now()}] $message');
    }
  }
  
  class MyClass with Logger {
    void doSomething() {
      log('Doing something important');
    }
  }
  
  // Abstract class
  abstract class Animal {
    String name;
    
    Animal(this.name);
    
    void makeSound(); // Abstract method
  }
  
  // Interface (implicit in Dart)
  class Dog implements Animal {
    @override
    String name;
    
    Dog(this.name);
    
    @override
    void makeSound() {
      print('$name barks!');
    }
  }
  
  // Extension methods
  extension StringExtension on String {
    String capitalize() {
      return '${this[0].toUpperCase()}${substring(1)}';
    }
  }
  
  // Enum
  enum Color { red, green, blue }
  
  // Async/await
  Future<String> fetchData() async {
    await Future.delayed(Duration(seconds: 1));
    return 'Data fetched successfully';
  }
  
  // Stream
  Stream<int> countStream() async* {
    for (int i = 1; i <= 5; i++) {
      await Future.delayed(Duration(seconds: 1));
      yield i;
    }
  }
  
  // Generics
  class Box<T> {
    T value;
    
    Box(this.value);
    
    T getValue() => value;
  }
  
  // Cascade notation
  var person = Person('Alice', 30)
    ..name = 'Alice Smith'
    ..age = 31;
  
  // Conditional expressions
  var status = isAwesome ? 'Awesome' : 'Not awesome';
  var displayName = name ?? 'Default Name';
  
  // Exception handling
  try {
    var result = 10 ~/ 0;
  } catch (e, stackTrace) {
    print('Error: $e');
    print('Stack trace: $stackTrace');
  } finally {
    print('Cleanup code');
  }
  
  // Assert
  assert(pi > 3);
  
  // Const and final
  const double gravity = 9.81;
  final DateTime now = DateTime.now();
  
  // Library import with alias
  import 'dart:math' as math;
  
  // Part and part of
  part 'some_file.dart';
  part of 'main_library.dart';
  
  // Typedef
  typedef IntOperation = int Function(int a, int b);
  
  // Callable class
  class Multiplier {
    int factor;
    
    Multiplier(this.factor);
    
    int call(int value) => value * factor;
  }
}
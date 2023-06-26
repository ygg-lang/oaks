use oak_csharp::{CSharpRoot, parse};

#[test]
fn test_basic_syntax() {
    let code = r#"
        using System;
        
        namespace Test {
            public class Program {
                public static void Main(string[] args) {
                    Console.WriteLine("Hello, World!");
                }
            }
        }
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse basic C# code");

    let ast = result.unwrap();
    assert!(!ast.items.is_empty(), "AST should not be empty");
}

#[test]
fn test_expressions() {
    let code = r#"
        public class Test {
            public int Calculate() {
                int a = 10;
                int b = 20;
                int c = a + b * 2;
                return c;
            }
        }
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse expressions");
}

#[test]
fn test_control_flow() {
    let code = r#"
        public class Test {
            public string GetGrade(int score) {
                if (score >= 90) {
                    return "A";
                } else if (score >= 80) {
                    return "B";
                } else {
                    return "C";
                }
            }
        }
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse control flow");
}

#[test]
fn test_loops() {
    let code = r#"
        public class Test {
            public int Sum(int n) {
                int sum = 0;
                for (int i = 1; i <= n; i++) {
                    sum += i;
                }
                return sum;
            }
        }
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse loops");
}

#[test]
fn test_switch_statement() {
    let code = r#"
        public class Test {
            public string GetDayName(int day) {
                switch (day) {
                    case 1:
                        return "Monday";
                    case 2:
                        return "Tuesday";
                    default:
                        return "Unknown";
                }
            }
        }
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse switch statement");
}

#[test]
fn test_try_catch() {
    let code = r#"
        public class Test {
            public int Divide(int a, int b) {
                try {
                    return a / b;
                } catch (DivideByZeroException ex) {
                    Console.WriteLine("Cannot divide by zero");
                    return 0;
                } finally {
                    Console.WriteLine("Operation completed");
                }
            }
        }
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse try-catch");
}

#[test]
fn test_lambda_expressions() {
    let code = r#"
        public class Test {
            public void TestLambda() {
                Func<int, int> square = x => x * x;
                int result = square(5);
            }
        }
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse lambda expressions");
}

#[test]
fn test_async_await() {
    let code = r#"
        public class Test {
            public async Task<string> GetDataAsync() {
                await Task.Delay(1000);
                return "Data";
            }
        }
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse async-await");
}

#[test]
fn test_object_creation() {
    let code = r#"
        public class Test {
            public void CreateObjects() {
                var person = new Person { Name = "John", Age = 30 };
                var numbers = new List<int> { 1, 2, 3, 4, 5 };
            }
        }
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse object creation");
}

#[test]
fn test_generics() {
    let code = r#"
        public class Test<T> {
            public T Value { get; set; }
            
            public void SetValue(T value) {
                Value = value;
            }
        }
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse generics");
}

#[test]
fn test_properties() {
    let code = r#"
        public class Test {
            private string _name;
            
            public string Name {
                get { return _name; }
                set { _name = value; }
            }
            
            public int Age { get; set; }
        }
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse properties");
}

#[test]
fn test_indexers() {
    let code = r#"
        public class Test {
            private int[] _items = new int[10];
            
            public int this[int index] {
                get { return _items[index]; }
                set { _items[index] = value; }
            }
        }
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse indexers");
}

#[test]
fn test_events() {
    let code = r#"
        public class Test {
            public event EventHandler MyEvent;
            
            protected virtual void OnMyEvent(EventArgs e) {
                MyEvent?.Invoke(this, e);
            }
        }
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse events");
}

#[test]
fn test_delegates() {
    let code = r#"
        public delegate int Calculator(int a, int b);
        
        public class Test {
            public static int Add(int a, int b) {
                return a + b;
            }
        }
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse delegates");
}

#[test]
fn test_enums() {
    let code = r#"
        public enum Color {
            Red,
            Green,
            Blue
        }
        
        public class Test {
            public Color MyColor { get; set; }
        }
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse enums");
}

#[test]
fn test_structs() {
    let code = r#"
        public struct Point {
            public int X { get; set; }
            public int Y { get; set; }
            
            public Point(int x, int y) {
                X = x;
                Y = y;
            }
        }
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse structs");
}

#[test]
fn test_interfaces() {
    let code = r#"
        public interface IAnimal {
            void Speak();
        }
        
        public class Dog : IAnimal {
            public void Speak() {
                Console.WriteLine("Woof!");
            }
        }
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse interfaces");
}

#[test]
fn test_records() {
    let code = r#"
        public record Person(string Name, int Age);
        
        public class Test {
            public void TestRecord() {
                var person = new Person("John", 30);
            }
        }
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse records");
}

#[test]
fn test_namespaces() {
    let code = r#"
        namespace MyCompany.Project.Core {
            public class Test {
                public void DoSomething() {}
            }
        }
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse namespaces");
}

#[test]
fn test_using_directives() {
    let code = r#"
        using System;
        using System.Collections.Generic;
        using static System.Math;
        
        public class Test {
            public double Calculate() {
                return Sqrt(16);
            }
        }
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse using directives");
}

#[test]
fn test_complex_code() {
    let code = r#"
        using System;
        using System.Collections.Generic;
        using System.Threading.Tasks;
        
        namespace ComplexExample {
            public class Program {
                public static async Task Main(string[] args) {
                    var service = new DataService();
                    var data = await service.GetDataAsync();
                    Console.WriteLine(data);
                }
            }
            
            public class DataService {
                public async Task<string> GetDataAsync() {
                    await Task.Delay(1000);
                    return "Hello from DataService";
                }
            }
        }
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse complex code");
}

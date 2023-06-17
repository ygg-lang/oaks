using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace TestApplication
{
    // Class definition with properties
    public class Person
    {
        public string Name { get; set; }
        public int Age { get; set; }
        
        // Constructor
        public Person(string name, int age)
        {
            Name = name;
            Age = age;
        }
        
        // Method
        public virtual string GetDescription()
        {
            return $"Person: {Name}, Age: {Age}";
        }
    }
    
    // Interface definition
    public interface IEmployee
    {
        decimal Salary { get; set; }
        void Work();
    }
    
    // Class inheritance and interface implementation
    public class Employee : Person, IEmployee
    {
        public decimal Salary { get; set; }
        public string Department { get; set; }
        
        public Employee(string name, int age, decimal salary, string department) 
            : base(name, age)
        {
            Salary = salary;
            Department = department;
        }
        
        public override string GetDescription()
        {
            return $"Employee: {Name}, Age: {Age}, Salary: {Salary:C}, Department: {Department}";
        }
        
        public void Work()
        {
            Console.WriteLine($"{Name} is working in {Department}");
        }
    }
    
    // Generic class
    public class Repository<T> where T : class
    {
        private List<T> items = new List<T>();
        
        public void Add(T item)
        {
            items.Add(item);
        }
        
        public T Get(int index)
        {
            return items[index];
        }
        
        public IEnumerable<T> GetAll()
        {
            return items;
        }
    }
    
    // Enum definition
    public enum Status
    {
        Active,
        Inactive,
        Pending
    }
    
    // Extension method
    public static class StringExtensions
    {
        public static bool IsNullOrEmpty(this string value)
        {
            return string.IsNullOrEmpty(value);
        }
    }
    
    // Async method
    public static async Task<string> GetDataAsync()
    {
        await Task.Delay(1000);
        return "Data retrieved";
    }
    
    // Main program
    class Program
    {
        static async Task Main(string[] args)
        {
            // Object creation
            var person = new Person("Alice", 30);
            var employee = new Employee("Bob", 35, 50000m, "IT");
            
            // LINQ usage
            var numbers = new[] { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 };
            var evenNumbers = numbers.Where(n => n % 2 == 0).ToList();
            
            // Pattern matching with switch expression
            var status = GetStatus(employee);
            Console.WriteLine($"Status: {status}");
            
            // Using extension method
            var testString = "";
            if (testString.IsNullOrEmpty())
            {
                Console.WriteLine("String is empty");
            }
            
            // Async/await
            var data = await GetDataAsync();
            Console.WriteLine(data);
        }
        
        // Switch expression
        static string GetStatus(Person person) => person switch
        {
            Employee e when e.Salary > 40000 => "High-paid employee",
            Employee e => "Regular employee",
            Person p when p.Age > 65 => "Senior person",
            _ => "Regular person"
        };
    }
}
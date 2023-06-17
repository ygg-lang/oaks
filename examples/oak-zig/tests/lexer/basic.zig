// Zig test file
const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Constants
const PI: f64 = 3.14159265359;
const MAX_SIZE: usize = 1000;

// Structs
const Point = struct {
    x: f64,
    y: f64,
    
    fn distance(self: Point, other: Point) f64 {
        const dx = self.x - other.x;
        const dy = self.y - other.y;
        return @sqrt(dx * dx + dy * dy);
    }
    
    fn init(x: f64, y: f64) Point {
        return Point{ .x = x, .y = y };
    }
};

// Enums
const Color = enum {
    red,
    green,
    blue,
    
    fn toString(self: Color) []const u8 {
        return switch (self) {
            .red => "Red",
            .green => "Green",
            .blue => "Blue",
        };
    }
};

// Union
const Value = union(enum) {
    integer: i32,
    float: f64,
    string: []const u8,
    
    fn print_value(self: Value) void {
        switch (self) {
            .integer => |val| print("Integer: {}\n", .{val}),
            .float => |val| print("Float: {d}\n", .{val}),
            .string => |val| print("String: {s}\n", .{val}),
        }
    }
};

// Functions
fn factorial(n: u32) u32 {
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}

fn fibonacci(n: u32) u32 {
    if (n <= 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}

// Generic function
fn max(comptime T: type, a: T, b: T) T {
    return if (a > b) a else b;
}

// Error handling
const MathError = error{
    DivisionByZero,
    NegativeSquareRoot,
};

fn divide(a: f64, b: f64) MathError!f64 {
    if (b == 0) return MathError.DivisionByZero;
    return a / b;
}

fn sqrt_positive(x: f64) MathError!f64 {
    if (x < 0) return MathError.NegativeSquareRoot;
    return @sqrt(x);
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Basic operations
    print("Hello, Zig!\n");
    
    // Variables
    var count: i32 = 0;
    const name = "Zig";
    
    // Arrays
    const numbers = [_]i32{ 1, 2, 3, 4, 5 };
    var sum: i32 = 0;
    for (numbers) |num| {
        sum += num;
    }
    print("Sum: {}\n", .{sum});
    
    // Dynamic array
    var list = ArrayList(i32).init(allocator);
    defer list.deinit();
    
    try list.append(10);
    try list.append(20);
    try list.append(30);
    
    print("List items: ");
    for (list.items) |item| {
        print("{} ", .{item});
    }
    print("\n");
    
    // Structs
    const p1 = Point.init(0, 0);
    const p2 = Point.init(3, 4);
    const dist = p1.distance(p2);
    print("Distance: {d}\n", .{dist});
    
    // Enums
    const color = Color.red;
    print("Color: {s}\n", .{color.toString()});
    
    // Unions
    const values = [_]Value{
        Value{ .integer = 42 },
        Value{ .float = 3.14 },
        Value{ .string = "Hello" },
    };
    
    for (values) |val| {
        val.print_value();
    }
    
    // Error handling
    const result = divide(10, 2) catch |err| {
        print("Error: {}\n", .{err});
        return;
    };
    print("Division result: {d}\n", .{result});
    
    // Loops
    var i: u32 = 0;
    while (i < 5) : (i += 1) {
        print("Factorial of {}: {}\n", .{ i, factorial(i) });
    }
    
    // Conditionals
    const x = 10;
    if (x > 5) {
        print("x is greater than 5\n");
    } else {
        print("x is not greater than 5\n");
    }
    
    // Generic function
    const max_int = max(i32, 10, 20);
    const max_float = max(f64, 3.14, 2.71);
    print("Max int: {}, Max float: {d}\n", .{ max_int, max_float });
}
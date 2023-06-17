#include <iostream>
#include <vector>
#include <memory>
#include <string>
#include <algorithm>

// Template class
template<typename T>
class Container {
private:
    std::vector<T> data;
    
public:
    void add(const T& item) {
        data.push_back(item);
    }
    
    T get(size_t index) const {
        return data.at(index);
    }
    
    size_t size() const {
        return data.size();
    }
};

// Abstract base class
class Shape {
public:
    virtual ~Shape() = default;
    virtual double area() const = 0;
    virtual std::string name() const = 0;
};

// Concrete class with constructor initialization list
class Rectangle : public Shape {
private:
    double width, height;
    
public:
    Rectangle(double w, double h) : width(w), height(h) {}
    
    double area() const override {
        return width * height;
    }
    
    std::string name() const override {
        return "Rectangle";
    }
};

// Smart pointers and modern C++ features
void modern_cpp_features() {
    // Auto keyword
    auto rectangle = std::make_unique<Rectangle>(10.0, 5.0);
    
    // Range-based for loop
    std::vector<int> numbers = {1, 2, 3, 4, 5};
    for (const auto& num : numbers) {
        std::cout << num << " ";
    }
    std::cout << std::endl;
    
    // Lambda function
    auto lambda = [](int x) { return x * x; };
    std::cout << "Square of 5: " << lambda(5) << std::endl;
    
    // Move semantics
    std::string str1 = "Hello";
    std::string str2 = std::move(str1);
}

// Namespace
namespace Math {
    constexpr double PI = 3.14159265359;
    
    template<typename T>
    T max(T a, T b) {
        return (a > b) ? a : b;
    }
}

// Enum class (strongly typed enum)
enum class Color {
    Red,
    Green,
    Blue
};

// Main function
int main() {
    Container<int> intContainer;
    intContainer.add(42);
    intContainer.add(17);
    
    std::cout << "Container size: " << intContainer.size() << std::endl;
    
    modern_cpp_features();
    
    return 0;
}
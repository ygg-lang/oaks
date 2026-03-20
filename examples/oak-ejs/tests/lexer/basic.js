// JavaScript test file
'use strict';

// Variables and constants
const PI = 3.14159;
let count = 0;
var name = "JavaScript";

// Functions
function greet(person) {
    return `Hello, ${person}!`;
}

const add = (a, b) => a + b;

// Arrow function with block
const multiply = (a, b) => {
    const result = a * b;
    return result;
};

// Classes
class Person {
    constructor(name, age) {
        this.name = name;
        this.age = age;
    }
    
    introduce() {
        console.log(`Hi, I'm ${this.name} and I'm ${this.age} years old.`);
    }
    
    static createAdult(name) {
        return new Person(name, 18);
    }
}

// Inheritance
class Student extends Person {
    constructor(name, age, grade) {
        super(name, age);
        this.grade = grade;
    }
    
    study() {
        console.log(`${this.name} is studying.`);
    }
}

// Objects
const config = {
    debug: true,
    timeout: 5000,
    retries: 3,
    
    // Method shorthand
    log(message) {
        if (this.debug) {
            console.log(message);
        }
    }
};

// Arrays and array methods
const numbers = [1, 2, 3, 4, 5];
const squares = numbers.map(n => n * n);
const evens = numbers.filter(n => n % 2 === 0);
const sum = numbers.reduce((acc, n) => acc + n, 0);

// Destructuring
const [first, second, ...rest] = numbers;
const {debug, timeout} = config;

// Template literals
const message = `
    Welcome to ${name}!
    Current count: ${count}
    Sum of numbers: ${sum}
`;

// Promises
function fetchData() {
    return new Promise((resolve, reject) => {
        setTimeout(() => {
            if (Math.random() > 0.5) {
                resolve("Data fetched successfully");
            } else {
                reject(new Error("Failed to fetch data"));
            }
        }, 1000);
    });
}

// Async/await
async function processData() {
    try {
        const data = await fetchData();
        console.log(data);
        return data;
    } catch (error) {
        console.error("Error:", error.message);
        throw error;
    }
}

// Modules (ES6)
export { Person, Student, add, multiply };
export default config;

// Regular expressions
const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
const phoneRegex = /^\+?[\d\s\-\(\)]+$/;

// Control flow
for (let i = 0; i < numbers.length; i++) {
    if (numbers[i] % 2 === 0) {
        console.log(`${numbers[i]} is even`);
    } else {
        console.log(`${numbers[i]} is odd`);
    }
}

// For...of loop
for (const number of numbers) {
    console.log(number);
}

// For...in loop
for (const key in config) {
    console.log(`${key}: ${config[key]}`);
}

// Switch statement
function getDay(dayNumber) {
    switch (dayNumber) {
        case 0:
            return "Sunday";
        case 1:
            return "Monday";
        case 2:
            return "Tuesday";
        case 3:
            return "Wednesday";
        case 4:
            return "Thursday";
        case 5:
            return "Friday";
        case 6:
            return "Saturday";
        default:
            return "Invalid day";
    }
}

// Error handling
try {
    const result = JSON.parse('{"invalid": json}');
} catch (error) {
    console.error("JSON parsing failed:", error.message);
} finally {
    console.log("Cleanup completed");
}
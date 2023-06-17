// TypeScript test file

// Type definitions
interface User {
    id: number;
    name: string;
    email: string;
    isActive: boolean;
    roles?: string[];
}

type Status = 'pending' | 'approved' | 'rejected';

// Generic interface
interface Repository<T> {
    findById(id: number): Promise<T | null>;
    save(entity: T): Promise<T>;
    delete(id: number): Promise<void>;
}

// Class with types
class UserService implements Repository<User> {
    private users: User[] = [];
    
    constructor(private readonly apiUrl: string) {}
    
    async findById(id: number): Promise<User | null> {
        return this.users.find(user => user.id === id) || null;
    }
    
    async save(user: User): Promise<User> {
        const existingIndex = this.users.findIndex(u => u.id === user.id);
        if (existingIndex >= 0) {
            this.users[existingIndex] = user;
        } else {
            this.users.push(user);
        }
        return user;
    }
    
    async delete(id: number): Promise<void> {
        this.users = this.users.filter(user => user.id !== id);
    }
    
    // Generic method
    async findByField<K extends keyof User>(field: K, value: User[K]): Promise<User[]> {
        return this.users.filter(user => user[field] === value);
    }
}

// Abstract class
abstract class Shape {
    abstract area(): number;
    abstract perimeter(): number;
    
    describe(): string {
        return `Area: ${this.area()}, Perimeter: ${this.perimeter()}`;
    }
}

class Circle extends Shape {
    constructor(private radius: number) {
        super();
    }
    
    area(): number {
        return Math.PI * this.radius ** 2;
    }
    
    perimeter(): number {
        return 2 * Math.PI * this.radius;
    }
}

// Enum
enum Color {
    Red = "red",
    Green = "green",
    Blue = "blue"
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

// Union types
type Theme = 'light' | 'dark';
type Size = 'small' | 'medium' | 'large';

// Intersection types
type Styled = {
    color: Color;
    size: Size;
};

type Themed = {
    theme: Theme;
};

type StyledComponent = Styled & Themed;

// Function types
type EventHandler<T> = (event: T) => void;
type Validator<T> = (value: T) => boolean;

// Generic functions
function identity<T>(arg: T): T {
    return arg;
}

function map<T, U>(array: T[], fn: (item: T) => U): U[] {
    return array.map(fn);
}

// Utility types
type PartialUser = Partial<User>;
type RequiredUser = Required<User>;
type UserEmail = Pick<User, 'email'>;
type UserWithoutId = Omit<User, 'id'>;

// Conditional types
type NonNullable<T> = T extends null | undefined ? never : T;

// Mapped types
type ReadonlyUser = {
    readonly [K in keyof User]: User[K];
};

// Decorators
function logged(target: any, propertyName: string, descriptor: PropertyDescriptor) {
    const method = descriptor.value;
    descriptor.value = function (...args: any[]) {
        console.log(`Calling ${propertyName} with args:`, args);
        const result = method.apply(this, args);
        console.log(`${propertyName} returned:`, result);
        return result;
    };
}

class Calculator {
    @logged
    add(a: number, b: number): number {
        return a + b;
    }
    
    @logged
    multiply(a: number, b: number): number {
        return a * b;
    }
}

// Namespace
namespace Utilities {
    export function formatDate(date: Date): string {
        return date.toISOString().split('T')[0];
    }
    
    export function capitalize(str: string): string {
        return str.charAt(0).toUpperCase() + str.slice(1);
    }
}

// Module augmentation
declare global {
    interface Array<T> {
        first(): T | undefined;
        last(): T | undefined;
    }
}

Array.prototype.first = function<T>(this: T[]): T | undefined {
    return this[0];
};

Array.prototype.last = function<T>(this: T[]): T | undefined {
    return this[this.length - 1];
};

// Usage examples
const userService = new UserService('https://api.example.com');

const user: User = {
    id: 1,
    name: 'John Doe',
    email: 'john@example.com',
    isActive: true,
    roles: ['admin', 'user']
};

const circle = new Circle(5);
console.log(circle.describe());

const styledComponent: StyledComponent = {
    color: Color.Red,
    size: 'large',
    theme: 'dark'
};

// Type guards
function isUser(obj: any): obj is User {
    return obj && typeof obj.id === 'number' && typeof obj.name === 'string';
}

// Async/await with types
async function fetchUser(id: number): Promise<User | null> {
    try {
        const response = await fetch(`/api/users/${id}`);
        const data = await response.json();
        return isUser(data) ? data : null;
    } catch (error) {
        console.error('Failed to fetch user:', error);
        return null;
    }
}

export { User, UserService, Shape, Circle, Color, Direction, userService };
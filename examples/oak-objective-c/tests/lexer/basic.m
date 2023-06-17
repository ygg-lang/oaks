// Objective-C Test File - Comprehensive Syntax Coverage
// This file tests various Objective-C syntax elements for lexer testing

#import <Foundation/Foundation.h>
#import <UIKit/UIKit.h>
#import <CoreData/CoreData.h>
#import <QuartzCore/QuartzCore.h>

// Forward declarations
@class Person;
@class Vehicle;
@protocol Drawable;
@protocol Serializable;

// Protocol definitions
@protocol Drawable <NSObject>
@required
- (void)draw;
- (void)drawAtPoint:(CGPoint)point;

@optional
- (void)drawWithColor:(UIColor *)color;
- (CGSize)size;
@end

@protocol Serializable <NSObject>
- (NSData *)serialize;
- (BOOL)deserializeFromData:(NSData *)data;
@end

@protocol Copyable <NSObject>
- (id)copyWithZone:(NSZone *)zone;
@end

// Category declaration
@interface NSString (Utilities)
- (BOOL)isValidEmail;
- (NSString *)reverseString;
- (NSInteger)wordCount;
@end

// Enumeration definitions
typedef NS_ENUM(NSInteger, Direction) {
    DirectionNorth = 0,
    DirectionSouth,
    DirectionEast,
    DirectionWest
};

typedef NS_OPTIONS(NSUInteger, FilePermissions) {
    FilePermissionRead    = 1 << 0,
    FilePermissionWrite   = 1 << 1,
    FilePermissionExecute = 1 << 2
};

typedef NS_ENUM(NSInteger, LogLevel) {
    LogLevelTrace = 0,
    LogLevelDebug = 1,
    LogLevelInfo  = 2,
    LogLevelWarn  = 3,
    LogLevelError = 4,
    LogLevelFatal = 5
};

// Structure definitions
typedef struct {
    CGFloat x;
    CGFloat y;
    CGFloat z;
} Point3D;

typedef struct {
    Point3D origin;
    Point3D size;
} BoundingBox;

// Block type definitions
typedef void (^CompletionBlock)(BOOL success, NSError *error);
typedef NSString * (^StringTransformBlock)(NSString *input);
typedef NSInteger (^ComparisonBlock)(id obj1, id obj2);

// Class interface - Base class
@interface Person : NSObject <NSCopying, NSCoding, Serializable>

// Properties with different attributes
@property (nonatomic, strong) NSString *firstName;
@property (nonatomic, strong) NSString *lastName;
@property (nonatomic, assign) NSInteger age;
@property (nonatomic, strong) NSDate *birthDate;
@property (nonatomic, strong) NSMutableArray<NSString *> *hobbies;
@property (nonatomic, weak) Person *spouse;
@property (nonatomic, strong, readonly) NSString *fullName;
@property (nonatomic, assign, getter=isActive) BOOL active;
@property (nonatomic, copy) NSString *email;
@property (nonatomic, strong, nullable) NSString *phoneNumber;

// Class methods
+ (instancetype)personWithFirstName:(NSString *)firstName lastName:(NSString *)lastName;
+ (NSArray<Person *> *)personsFromJSONArray:(NSArray *)jsonArray;
+ (void)setDefaultAge:(NSInteger)age;

// Instance methods
- (instancetype)initWithFirstName:(NSString *)firstName 
                         lastName:(NSString *)lastName 
                              age:(NSInteger)age;
- (instancetype)initWithDictionary:(NSDictionary *)dictionary;

- (void)celebrateBirthday;
- (void)addHobby:(NSString *)hobby;
- (void)removeHobby:(NSString *)hobby;
- (BOOL)hasHobby:(NSString *)hobby;

- (void)performActionWithCompletion:(CompletionBlock)completion;
- (void)processDataInBackground:(NSData *)data 
                     completion:(void (^)(id result, NSError *error))completion;

// Method with multiple parameters
- (void)updatePersonWithFirstName:(NSString *)firstName
                          lastName:(NSString *)lastName
                               age:(NSInteger)age
                             email:(NSString *)email
                       phoneNumber:(nullable NSString *)phoneNumber;

@end

// Class interface - Inheritance
@interface Employee : Person

@property (nonatomic, strong) NSString *employeeId;
@property (nonatomic, strong) NSString *department;
@property (nonatomic, assign) CGFloat salary;
@property (nonatomic, strong) NSDate *hireDate;
@property (nonatomic, weak) Employee *manager;
@property (nonatomic, strong) NSMutableSet<Employee *> *subordinates;

- (instancetype)initWithFirstName:(NSString *)firstName
                         lastName:(NSString *)lastName
                              age:(NSInteger)age
                       employeeId:(NSString *)employeeId
                       department:(NSString *)department;

- (void)promoteToPosition:(NSString *)position withSalaryIncrease:(CGFloat)increase;
- (void)addSubordinate:(Employee *)employee;
- (void)removeSubordinate:(Employee *)employee;
- (NSArray<Employee *> *)allSubordinates;

@end

// Class interface with generics
@interface Container<ObjectType> : NSObject

@property (nonatomic, strong) NSMutableArray<ObjectType> *items;
@property (nonatomic, assign, readonly) NSUInteger count;

- (void)addItem:(ObjectType)item;
- (void)removeItem:(ObjectType)item;
- (ObjectType)itemAtIndex:(NSUInteger)index;
- (void)enumerateItemsUsingBlock:(void (^)(ObjectType item, NSUInteger idx, BOOL *stop))block;

@end

// Class interface implementing protocols
@interface Shape : NSObject <Drawable, Copyable>

@property (nonatomic, assign) CGPoint center;
@property (nonatomic, strong) UIColor *fillColor;
@property (nonatomic, strong) UIColor *strokeColor;
@property (nonatomic, assign) CGFloat strokeWidth;

- (instancetype)initWithCenter:(CGPoint)center;
- (CGFloat)area;
- (CGFloat)perimeter;

@end

@interface Circle : Shape
@property (nonatomic, assign) CGFloat radius;
- (instancetype)initWithCenter:(CGPoint)center radius:(CGFloat)radius;
@end

@interface Rectangle : Shape
@property (nonatomic, assign) CGSize size;
- (instancetype)initWithCenter:(CGPoint)center size:(CGSize)size;
@end

// Implementation files

@implementation Person {
    // Private instance variables
    NSMutableDictionary *_metadata;
    NSInteger _privateCounter;
}

// Synthesize properties (optional in modern Objective-C)
@synthesize firstName = _firstName;
@synthesize lastName = _lastName;

// Dynamic properties
@dynamic fullName;

// Class variables
static NSInteger defaultAge = 18;
static NSMutableArray *allPersons = nil;

// Class methods implementation
+ (void)initialize {
    if (self == [Person class]) {
        allPersons = [[NSMutableArray alloc] init];
    }
}

+ (instancetype)personWithFirstName:(NSString *)firstName lastName:(NSString *)lastName {
    return [[self alloc] initWithFirstName:firstName lastName:lastName age:defaultAge];
}

+ (NSArray<Person *> *)personsFromJSONArray:(NSArray *)jsonArray {
    NSMutableArray *persons = [NSMutableArray array];
    
    for (NSDictionary *dict in jsonArray) {
        if ([dict isKindOfClass:[NSDictionary class]]) {
            Person *person = [[Person alloc] initWithDictionary:dict];
            if (person) {
                [persons addObject:person];
            }
        }
    }
    
    return [persons copy];
}

+ (void)setDefaultAge:(NSInteger)age {
    defaultAge = age;
}

// Instance methods implementation
- (instancetype)init {
    return [self initWithFirstName:@"" lastName:@"" age:defaultAge];
}

- (instancetype)initWithFirstName:(NSString *)firstName 
                         lastName:(NSString *)lastName 
                              age:(NSInteger)age {
    self = [super init];
    if (self) {
        _firstName = [firstName copy];
        _lastName = [lastName copy];
        _age = age;
        _birthDate = [NSDate date];
        _hobbies = [NSMutableArray array];
        _metadata = [NSMutableDictionary dictionary];
        _active = YES;
        
        [allPersons addObject:self];
    }
    return self;
}

- (instancetype)initWithDictionary:(NSDictionary *)dictionary {
    NSString *firstName = dictionary[@"firstName"];
    NSString *lastName = dictionary[@"lastName"];
    NSNumber *ageNumber = dictionary[@"age"];
    
    if (!firstName || !lastName) {
        return nil;
    }
    
    NSInteger age = ageNumber ? [ageNumber integerValue] : defaultAge;
    
    self = [self initWithFirstName:firstName lastName:lastName age:age];
    if (self) {
        self.email = dictionary[@"email"];
        self.phoneNumber = dictionary[@"phoneNumber"];
        
        NSArray *hobbiesArray = dictionary[@"hobbies"];
        if ([hobbiesArray isKindOfClass:[NSArray class]]) {
            [self.hobbies addObjectsFromArray:hobbiesArray];
        }
    }
    
    return self;
}

- (void)dealloc {
    [allPersons removeObject:self];
    NSLog(@"Person %@ %@ deallocated", self.firstName, self.lastName);
}

// Property getters and setters
- (NSString *)fullName {
    return [NSString stringWithFormat:@"%@ %@", self.firstName, self.lastName];
}

- (void)setAge:(NSInteger)age {
    if (age >= 0 && age <= 150) {
        _age = age;
        
        // Update birth date based on age
        NSCalendar *calendar = [NSCalendar currentCalendar];
        NSDateComponents *components = [[NSDateComponents alloc] init];
        components.year = -age;
        self.birthDate = [calendar dateByAddingComponents:components toDate:[NSDate date] options:0];
    }
}

// Instance methods
- (void)celebrateBirthday {
    self.age++;
    NSLog(@"Happy Birthday %@! You are now %ld years old.", self.fullName, (long)self.age);
    
    // Post notification
    [[NSNotificationCenter defaultCenter] postNotificationName:@"PersonBirthdayNotification"
                                                        object:self
                                                      userInfo:@{@"newAge": @(self.age)}];
}

- (void)addHobby:(NSString *)hobby {
    if (hobby && ![self.hobbies containsObject:hobby]) {
        [self.hobbies addObject:hobby];
    }
}

- (void)removeHobby:(NSString *)hobby {
    [self.hobbies removeObject:hobby];
}

- (BOOL)hasHobby:(NSString *)hobby {
    return [self.hobbies containsObject:hobby];
}

- (void)performActionWithCompletion:(CompletionBlock)completion {
    dispatch_async(dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_DEFAULT, 0), ^{
        // Simulate some work
        [NSThread sleepForTimeInterval:1.0];
        
        BOOL success = arc4random_uniform(10) > 2; // 80% success rate
        NSError *error = nil;
        
        if (!success) {
            error = [NSError errorWithDomain:@"PersonErrorDomain"
                                        code:1001
                                    userInfo:@{NSLocalizedDescriptionKey: @"Action failed"}];
        }
        
        dispatch_async(dispatch_get_main_queue(), ^{
            if (completion) {
                completion(success, error);
            }
        });
    });
}

- (void)processDataInBackground:(NSData *)data 
                     completion:(void (^)(id result, NSError *error))completion {
    dispatch_queue_t processingQueue = dispatch_queue_create("com.example.processing", DISPATCH_QUEUE_CONCURRENT);
    
    dispatch_async(processingQueue, ^{
        @try {
            // Simulate data processing
            NSError *error = nil;
            id result = [NSJSONSerialization JSONObjectWithData:data options:0 error:&error];
            
            dispatch_async(dispatch_get_main_queue(), ^{
                if (completion) {
                    completion(result, error);
                }
            });
        }
        @catch (NSException *exception) {
            NSError *error = [NSError errorWithDomain:@"ProcessingErrorDomain"
                                                 code:2001
                                             userInfo:@{NSLocalizedDescriptionKey: exception.reason}];
            
            dispatch_async(dispatch_get_main_queue(), ^{
                if (completion) {
                    completion(nil, error);
                }
            });
        }
    });
}

- (void)updatePersonWithFirstName:(NSString *)firstName
                          lastName:(NSString *)lastName
                               age:(NSInteger)age
                             email:(NSString *)email
                       phoneNumber:(nullable NSString *)phoneNumber {
    self.firstName = firstName;
    self.lastName = lastName;
    self.age = age;
    self.email = email;
    self.phoneNumber = phoneNumber;
}

// Protocol implementations
- (id)copyWithZone:(NSZone *)zone {
    Person *copy = [[[self class] allocWithZone:zone] initWithFirstName:self.firstName
                                                               lastName:self.lastName
                                                                    age:self.age];
    copy.email = self.email;
    copy.phoneNumber = self.phoneNumber;
    copy.active = self.active;
    [copy.hobbies addObjectsFromArray:self.hobbies];
    
    return copy;
}

- (void)encodeWithCoder:(NSCoder *)coder {
    [coder encodeObject:self.firstName forKey:@"firstName"];
    [coder encodeObject:self.lastName forKey:@"lastName"];
    [coder encodeInteger:self.age forKey:@"age"];
    [coder encodeObject:self.birthDate forKey:@"birthDate"];
    [coder encodeObject:self.hobbies forKey:@"hobbies"];
    [coder encodeObject:self.email forKey:@"email"];
    [coder encodeObject:self.phoneNumber forKey:@"phoneNumber"];
    [coder encodeBool:self.active forKey:@"active"];
}

- (instancetype)initWithCoder:(NSCoder *)coder {
    NSString *firstName = [coder decodeObjectForKey:@"firstName"];
    NSString *lastName = [coder decodeObjectForKey:@"lastName"];
    NSInteger age = [coder decodeIntegerForKey:@"age"];
    
    self = [self initWithFirstName:firstName lastName:lastName age:age];
    if (self) {
        self.birthDate = [coder decodeObjectForKey:@"birthDate"];
        self.hobbies = [coder decodeObjectForKey:@"hobbies"];
        self.email = [coder decodeObjectForKey:@"email"];
        self.phoneNumber = [coder decodeObjectForKey:@"phoneNumber"];
        self.active = [coder decodeBoolForKey:@"active"];
    }
    
    return self;
}

- (NSData *)serialize {
    return [NSKeyedArchiver archivedDataWithRootObject:self requiringSecureCoding:NO error:nil];
}

- (BOOL)deserializeFromData:(NSData *)data {
    NSError *error = nil;
    Person *person = [NSKeyedUnarchiver unarchivedObjectOfClass:[Person class] fromData:data error:&error];
    
    if (person && !error) {
        self.firstName = person.firstName;
        self.lastName = person.lastName;
        self.age = person.age;
        self.birthDate = person.birthDate;
        self.hobbies = person.hobbies;
        self.email = person.email;
        self.phoneNumber = person.phoneNumber;
        self.active = person.active;
        return YES;
    }
    
    return NO;
}

// Description method
- (NSString *)description {
    return [NSString stringWithFormat:@"<%@: %p> %@ (age: %ld, email: %@, hobbies: %@)",
            NSStringFromClass([self class]), self, self.fullName, (long)self.age, self.email, self.hobbies];
}

@end

// Employee implementation
@implementation Employee

- (instancetype)initWithFirstName:(NSString *)firstName
                         lastName:(NSString *)lastName
                              age:(NSInteger)age
                       employeeId:(NSString *)employeeId
                       department:(NSString *)department {
    self = [super initWithFirstName:firstName lastName:lastName age:age];
    if (self) {
        _employeeId = [employeeId copy];
        _department = [department copy];
        _hireDate = [NSDate date];
        _subordinates = [NSMutableSet set];
    }
    return self;
}

- (void)promoteToPosition:(NSString *)position withSalaryIncrease:(CGFloat)increase {
    self.salary += increase;
    NSLog(@"%@ promoted to %@ with salary increase of $%.2f", self.fullName, position, increase);
}

- (void)addSubordinate:(Employee *)employee {
    if (employee && employee != self) {
        [self.subordinates addObject:employee];
        employee.manager = self;
    }
}

- (void)removeSubordinate:(Employee *)employee {
    [self.subordinates removeObject:employee];
    employee.manager = nil;
}

- (NSArray<Employee *> *)allSubordinates {
    return [self.subordinates allObjects];
}

- (NSString *)description {
    return [NSString stringWithFormat:@"<%@: %p> %@ (ID: %@, Dept: %@, Salary: $%.2f)",
            NSStringFromClass([self class]), self, self.fullName, self.employeeId, self.department, self.salary];
}

@end

// Container implementation
@implementation Container

- (instancetype)init {
    self = [super init];
    if (self) {
        _items = [NSMutableArray array];
    }
    return self;
}

- (NSUInteger)count {
    return self.items.count;
}

- (void)addItem:(id)item {
    if (item) {
        [self.items addObject:item];
    }
}

- (void)removeItem:(id)item {
    [self.items removeObject:item];
}

- (id)itemAtIndex:(NSUInteger)index {
    if (index < self.items.count) {
        return self.items[index];
    }
    return nil;
}

- (void)enumerateItemsUsingBlock:(void (^)(id item, NSUInteger idx, BOOL *stop))block {
    [self.items enumerateObjectsUsingBlock:block];
}

@end

// Shape implementation
@implementation Shape

- (instancetype)initWithCenter:(CGPoint)center {
    self = [super init];
    if (self) {
        _center = center;
        _fillColor = [UIColor blueColor];
        _strokeColor = [UIColor blackColor];
        _strokeWidth = 1.0;
    }
    return self;
}

- (void)draw {
    // Base implementation - to be overridden
    NSLog(@"Drawing shape at center (%.2f, %.2f)", self.center.x, self.center.y);
}

- (void)drawAtPoint:(CGPoint)point {
    CGPoint originalCenter = self.center;
    self.center = point;
    [self draw];
    self.center = originalCenter;
}

- (void)drawWithColor:(UIColor *)color {
    UIColor *originalColor = self.fillColor;
    self.fillColor = color;
    [self draw];
    self.fillColor = originalColor;
}

- (CGSize)size {
    // Base implementation - to be overridden
    return CGSizeZero;
}

- (CGFloat)area {
    // Base implementation - to be overridden
    return 0.0;
}

- (CGFloat)perimeter {
    // Base implementation - to be overridden
    return 0.0;
}

- (id)copyWithZone:(NSZone *)zone {
    Shape *copy = [[[self class] allocWithZone:zone] initWithCenter:self.center];
    copy.fillColor = self.fillColor;
    copy.strokeColor = self.strokeColor;
    copy.strokeWidth = self.strokeWidth;
    return copy;
}

@end

// Circle implementation
@implementation Circle

- (instancetype)initWithCenter:(CGPoint)center radius:(CGFloat)radius {
    self = [super initWithCenter:center];
    if (self) {
        _radius = radius;
    }
    return self;
}

- (void)draw {
    NSLog(@"Drawing circle at (%.2f, %.2f) with radius %.2f", 
          self.center.x, self.center.y, self.radius);
}

- (CGSize)size {
    CGFloat diameter = self.radius * 2;
    return CGSizeMake(diameter, diameter);
}

- (CGFloat)area {
    return M_PI * self.radius * self.radius;
}

- (CGFloat)perimeter {
    return 2 * M_PI * self.radius;
}

@end

// Rectangle implementation
@implementation Rectangle

- (instancetype)initWithCenter:(CGPoint)center size:(CGSize)size {
    self = [super initWithCenter:center];
    if (self) {
        _size = size;
    }
    return self;
}

- (void)draw {
    NSLog(@"Drawing rectangle at (%.2f, %.2f) with size (%.2f, %.2f)", 
          self.center.x, self.center.y, self.size.width, self.size.height);
}

- (CGFloat)area {
    return self.size.width * self.size.height;
}

- (CGFloat)perimeter {
    return 2 * (self.size.width + self.size.height);
}

@end

// Category implementation
@implementation NSString (Utilities)

- (BOOL)isValidEmail {
    NSString *emailRegex = @"[A-Z0-9a-z._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,}";
    NSPredicate *emailPredicate = [NSPredicate predicateWithFormat:@"SELF MATCHES %@", emailRegex];
    return [emailPredicate evaluateWithObject:self];
}

- (NSString *)reverseString {
    NSMutableString *reversed = [NSMutableString string];
    NSInteger length = self.length;
    
    for (NSInteger i = length - 1; i >= 0; i--) {
        [reversed appendString:[self substringWithRange:NSMakeRange(i, 1)]];
    }
    
    return [reversed copy];
}

- (NSInteger)wordCount {
    NSArray *words = [self componentsSeparatedByCharactersInSet:[NSCharacterSet whitespaceAndNewlineCharacterSet]];
    NSArray *filteredWords = [words filteredArrayUsingPredicate:[NSPredicate predicateWithFormat:@"length > 0"]];
    return filteredWords.count;
}

@end

// Main function demonstrating usage
int main(int argc, char *argv[]) {
    @autoreleasepool {
        NSLog(@"=== Objective-C Comprehensive Test ===");
        
        // Create persons
        Person *john = [Person personWithFirstName:@"John" lastName:@"Doe"];
        john.age = 30;
        john.email = @"john.doe@example.com";
        [john addHobby:@"Reading"];
        [john addHobby:@"Swimming"];
        [john addHobby:@"Cooking"];
        
        Person *jane = [[Person alloc] initWithFirstName:@"Jane" lastName:@"Smith" age:28];
        jane.email = @"jane.smith@example.com";
        jane.phoneNumber = @"+1-555-0123";
        [jane addHobby:@"Photography"];
        [jane addHobby:@"Hiking"];
        
        // Set up relationship
        john.spouse = jane;
        jane.spouse = john;
        
        NSLog(@"Created persons: %@, %@", john, jane);
        
        // Create employees
        Employee *alice = [[Employee alloc] initWithFirstName:@"Alice"
                                                     lastName:@"Johnson"
                                                          age:35
                                                   employeeId:@"EMP001"
                                                   department:@"Engineering"];
        alice.salary = 85000.0;
        alice.email = @"alice.johnson@company.com";
        
        Employee *bob = [[Employee alloc] initWithFirstName:@"Bob"
                                                   lastName:@"Wilson"
                                                        age:32
                                                 employeeId:@"EMP002"
                                                 department:@"Engineering"];
        bob.salary = 75000.0;
        bob.email = @"bob.wilson@company.com";
        
        [alice addSubordinate:bob];
        
        NSLog(@"Created employees: %@, %@", alice, bob);
        
        // Test containers
        Container<Person *> *personContainer = [[Container alloc] init];
        [personContainer addItem:john];
        [personContainer addItem:jane];
        [personContainer addItem:alice];
        [personContainer addItem:bob];
        
        NSLog(@"Container has %lu persons", (unsigned long)personContainer.count);
        
        [personContainer enumerateItemsUsingBlock:^(Person *person, NSUInteger idx, BOOL *stop) {
            NSLog(@"Person %lu: %@", (unsigned long)idx, person.fullName);
        }];
        
        // Test shapes
        Circle *circle = [[Circle alloc] initWithCenter:CGPointMake(100, 100) radius:50];
        Rectangle *rectangle = [[Rectangle alloc] initWithCenter:CGPointMake(200, 200) 
                                                            size:CGSizeMake(80, 60)];
        
        NSArray<Shape *> *shapes = @[circle, rectangle];
        
        for (Shape *shape in shapes) {
            [shape draw];
            NSLog(@"Area: %.2f, Perimeter: %.2f", [shape area], [shape perimeter]);
        }
        
        // Test blocks and closures
        CompletionBlock completion = ^(BOOL success, NSError *error) {
            if (success) {
                NSLog(@"Operation completed successfully");
            } else {
                NSLog(@"Operation failed: %@", error.localizedDescription);
            }
        };
        
        [john performActionWithCompletion:completion];
        
        // Test string transformations
        StringTransformBlock upperCaseTransform = ^NSString *(NSString *input) {
            return [input uppercaseString];
        };
        
        StringTransformBlock reverseTransform = ^NSString *(NSString *input) {
            return [input reverseString];
        };
        
        NSString *testString = @"Hello, Objective-C!";
        NSLog(@"Original: %@", testString);
        NSLog(@"Uppercase: %@", upperCaseTransform(testString));
        NSLog(@"Reversed: %@", reverseTransform(testString));
        NSLog(@"Word count: %ld", (long)[testString wordCount]);
        NSLog(@"Is valid email: %@", [testString isValidEmail] ? @"YES" : @"NO");
        
        // Test email validation
        NSArray *emails = @[@"test@example.com", @"invalid-email", @"user@domain.org"];
        for (NSString *email in emails) {
            NSLog(@"'%@' is %@ email", email, [email isValidEmail] ? @"a valid" : @"an invalid");
        }
        
        // Test enumeration
        Direction currentDirection = DirectionNorth;
        NSLog(@"Current direction: %ld", (long)currentDirection);
        
        FilePermissions permissions = FilePermissionRead | FilePermissionWrite;
        NSLog(@"File permissions: %lu", (unsigned long)permissions);
        
        // Test data processing
        NSDictionary *sampleData = @{
            @"name": @"Sample Data",
            @"values": @[@1, @2, @3, @4, @5],
            @"metadata": @{
                @"created": [NSDate date],
                @"version": @"1.0"
            }
        };
        
        NSError *jsonError = nil;
        NSData *jsonData = [NSJSONSerialization dataWithJSONObject:sampleData 
                                                           options:NSJSONWritingPrettyPrinted 
                                                             error:&jsonError];
        
        if (jsonData && !jsonError) {
            [john processDataInBackground:jsonData completion:^(id result, NSError *error) {
                if (result) {
                    NSLog(@"Data processing result: %@", result);
                } else {
                    NSLog(@"Data processing error: %@", error.localizedDescription);
                }
            }];
        }
        
        // Test copying and serialization
        Person *johnCopy = [john copy];
        NSLog(@"Original: %@", john);
        NSLog(@"Copy: %@", johnCopy);
        
        NSData *serializedData = [john serialize];
        Person *deserializedPerson = [[Person alloc] init];
        BOOL deserializationSuccess = [deserializedPerson deserializeFromData:serializedData];
        NSLog(@"Deserialization %@: %@", deserializationSuccess ? @"succeeded" : @"failed", deserializedPerson);
        
        // Test birthday celebration
        [john celebrateBirthday];
        
        // Test advanced features
        NSLog(@"=== Advanced Features ===");
        
        // Key-Value Observing (KVO)
        [john addObserver:alice 
               forKeyPath:@"age" 
                  options:NSKeyValueObservingOptionNew | NSKeyValueObservingOptionOld 
                  context:NULL];
        
        john.age = 31; // This will trigger KVO notification
        
        [john removeObserver:alice forKeyPath:@"age"];
        
        // Key-Value Coding (KVC)
        [john setValue:@"john.doe.updated@example.com" forKey:@"email"];
        NSString *retrievedEmail = [john valueForKey:@"email"];
        NSLog(@"Retrieved email via KVC: %@", retrievedEmail);
        
        // Collection operators
        NSArray<Person *> *allPersons = @[john, jane, alice, bob];
        NSNumber *averageAge = [allPersons valueForKeyPath:@"@avg.age"];
        NSNumber *maxAge = [allPersons valueForKeyPath:@"@max.age"];
        NSNumber *minAge = [allPersons valueForKeyPath:@"@min.age"];
        NSNumber *totalAge = [allPersons valueForKeyPath:@"@sum.age"];
        
        NSLog(@"Average age: %.1f", [averageAge doubleValue]);
        NSLog(@"Max age: %ld", (long)[maxAge integerValue]);
        NSLog(@"Min age: %ld", (long)[minAge integerValue]);
        NSLog(@"Total age: %ld", (long)[totalAge integerValue]);
        
        // Predicates and filtering
        NSPredicate *agePredicate = [NSPredicate predicateWithFormat:@"age > 30"];
        NSArray *olderPersons = [allPersons filteredArrayUsingPredicate:agePredicate];
        NSLog(@"Persons older than 30: %@", [olderPersons valueForKey:@"fullName"]);
        
        // Sorting
        NSSortDescriptor *ageSortDescriptor = [NSSortDescriptor sortDescriptorWithKey:@"age" ascending:YES];
        NSSortDescriptor *nameSortDescriptor = [NSSortDescriptor sortDescriptorWithKey:@"lastName" ascending:YES];
        NSArray *sortedPersons = [allPersons sortedArrayUsingDescriptors:@[ageSortDescriptor, nameSortDescriptor]];
        
        NSLog(@"Sorted persons by age then name:");
        for (Person *person in sortedPersons) {
            NSLog(@"  %@ (age %ld)", person.fullName, (long)person.age);
        }
        
        NSLog(@"=== Test completed ===");
        
        // Wait for async operations to complete
        [[NSRunLoop currentRunLoop] runUntilDate:[NSDate dateWithTimeIntervalSinceNow:2.0]];
    }
    
    return 0;
}
// Comprehensive ActionScript test file for lexer testing

package com.example.lexer {
    import flash.display.Sprite;
    import flash.events.Event;
    import flash.net.URLRequest;
    import flash.utils.Dictionary;

    /**
     * This is a multi-line comment for the LexerTest class.
     * It covers various ActionScript syntax features.
     */
    public class LexerTest extends Sprite implements ITestInterface {
        // Constants
        public static const MAX_VALUE:int = 100;
        private const APP_NAME:String = "LexerApp";

        // Variables
        public var publicVar:Number = 3.14;
        private var _privateString:String = "Hello, ActionScript!";
        protected var protectedBoolean:Boolean = true;
        internal var internalArray:Array = [1, 2, 3];
        
        // Untyped variable
        var untypedVar = "dynamic";

        // XML literal
        private var xmlData:XML = <data>
                                    <item id="1">Value 1</item>
                                    <item id="2">Value 2</item>
                                  </data>;

        // Regular Expression literal
        private var regex:RegExp = /\btest\b/gi;

        // Dictionary
        private var myDictionary:Dictionary = new Dictionary();

        // Constructor
        public function LexerTest() {
            super();
            trace("LexerTest instance created for " + APP_NAME);
            setupTests();
        }

        // Interface method implementation
        public function runTest():void {
            trace("Running tests...");
            testVariablesAndTypes();
            testOperators();
            testControlFlow();
            testFunctions();
            testClassesAndObjects();
            testEvents();
            testXML();
            testErrorHandling();
            testNamespaces();
            testMetadata();
            trace("Tests completed.");
        }

        // Private method
        private function setupTests():void {
            myDictionary["key1"] = "value1";
            myDictionary["key2"] = "value2";
            
            // Access XML data
            for each (var item:XML in xmlData..item) {
                trace("XML Item ID: " + item.@id + ", Value: " + item.text());
            }
        }

        // Test variables and types
        private function testVariablesAndTypes():void {
            var localInt:int = 123;
            var localUint:uint = 456;
            var localNumber:Number = 7.89;
            var localBoolean:Boolean = false;
            var localString:String = 'single-quoted string';
            var localArray:Array = new Array("a", "b", "c");
            var localObject:Object = {key: "value", num: 10};

            trace("localInt: " + localInt);
            trace("localUint: " + localUint);
            trace("localNumber: " + localNumber);
            trace("localBoolean: " + localBoolean);
            trace("localString: " + localString);
            trace("localArray: " + localArray);
            trace("localObject: " + localObject.key + ", " + localObject.num);
            trace("Private string: " + _privateString);
            trace("Public var: " + publicVar);
        }

        // Test operators
        private function testOperators():void {
            var a:int = 10;
            var b:int = 5;
            var c:Boolean = true;
            var d:Boolean = false;

            // Arithmetic
            trace("a + b = " + (a + b));
            trace("a - b = " + (a - b));
            trace("a * b = " + (a * b));
            trace("a / b = " + (a / b));
            trace("a % b = " + (a % b));

            // Assignment
            a += b; trace("a += b: " + a);
            a -= b; trace("a -= b: " + a);

            // Comparison
            trace("a == b: " + (a == b));
            trace("a != b: " + (a != b));
            trace("a > b: " + (a > b));
            trace("a <= b: " + (a <= b));

            // Logical
            trace("c && d: " + (c && d));
            trace("c || d: " + (c || d));
            trace("!c: " + (!c));

            // Bitwise
            trace("a & b: " + (a & b));
            trace("a | b: " + (a | b));
            trace("a ^ b: " + (a ^ b));
            trace("~a: " + (~a));
            trace("a << 1: " + (a << 1));
            trace("a >> 1: " + (a >> 1));
            trace("a >>> 1: " + (a >>> 1));

            // Ternary
            var max:int = (a > b) ? a : b;
            trace("Max of a and b: " + max);
        }

        // Test control flow
        private function testControlFlow():void {
            var x:int = 10;
            if (x > 5) {
                trace("x is greater than 5");
            } else if (x == 5) {
                trace("x is 5");
            } else {
                trace("x is less than 5");
            }

            var day:String = "Monday";
            switch (day) {
                case "Monday":
                    trace("It's Monday.");
                    break;
                case "Tuesday":
                    trace("It's Tuesday.");
                    break;
                default:
                    trace("It's another day.");
            }

            for (var i:int = 0; i < 3; i++) {
                trace("For loop iteration: " + i);
            }

            var count:int = 0;
            while (count < 2) {
                trace("While loop iteration: " + count);
                count++;
            }

            var doCount:int = 0;
            do {
                trace("Do-while loop iteration: " + doCount);
                doCount++;
            } while (doCount < 1);

            var arr:Array = ["apple", "banana", "cherry"];
            for each (var fruit:String in arr) {
                trace("Fruit: " + fruit);
            }
            
            for (var prop:String in myDictionary) {
                trace("Dictionary key: " + prop + ", value: " + myDictionary[prop]);
            }
        }

        // Test functions
        private function testFunctions():void {
            function greet(name:String):String {
                return "Hello, " + name + "!";
            }
            trace(greet("Alice"));

            var multiply:Function = function(a:Number, b:Number):Number {
                return a * b;
            };
            trace("5 * 3 = " + multiply(5, 3));

            function calculate(num1:Number, num2:Number, operation:Function):Number {
                return operation(num1, num2);
            }
            trace("Calculated: " + calculate(10, 2, function(x:Number, y:Number):Number { return x / y; }));

            function defaultParams(a:int, b:int = 5):int {
                return a + b;
            }
            trace("Default params: " + defaultParams(10));
            trace("Default params with override: " + defaultParams(10, 10));

            function varArgs(...args):void {
                for each (var arg:* in args) {
                    trace("Var arg: " + arg);
                }
            }
            varArgs(1, "two", true);
        }

        // Test classes and objects
        private function testClassesAndObjects():void {
            var myCar:Car = new Car("Toyota", "Camry");
            trace("My car: " + myCar.getDetails());
            myCar.start();

            var myElectricCar:ElectricCar = new ElectricCar("Tesla", "Model S", 100);
            trace("My electric car: " + myElectricCar.getDetails());
            myElectricCar.charge();
            myElectricCar.start();
        }

        // Test events
        private function testEvents():void {
            this.addEventListener(Event.ADDED_TO_STAGE, onAddedToStage);
            // Simulate dispatching an event
            dispatchEvent(new Event("CUSTOM_EVENT"));
        }

        private function onAddedToStage(event:Event):void {
            trace("Added to stage event received.");
            this.removeEventListener(Event.ADDED_TO_STAGE, onAddedToStage);
            this.addEventListener("CUSTOM_EVENT", onCustomEvent);
        }

        private function onCustomEvent(event:Event):void {
            trace("Custom event received: " + event.type);
        }

        // Test XML (E4X)
        private function testXML():void {
            var catalog:XML = <catalog>
                                <book id="bk101">
                                    <author>Gambardella, Matthew</author>
                                    <title>XML Developer's Guide</title>
                                    <genre>Computer</genre>
                                    <price>44.95</price>
                                    <publish_date>2000-10-01</publish_date>
                                    <description>An in-depth look at creating applications with XML.</description>
                                </book>
                                <book id="bk102">
                                    <author>Ralls, Kim</author>
                                    <title>Midnight Rain</title>
                                    <genre>Fantasy</genre>
                                    <price>5.95</price>
                                    <publish_date>2000-12-16</publish_date>
                                    <description>A former architect battles an evil sorceress in the year 2000.</description>
                                </book>
                              </catalog>;

            trace("First book title: " + catalog.book[0].title);
            trace("Books with price > 10: ");
            for each (var book:XML in catalog..book.(price > 10)) {
                trace(" - " + book.title + " (" + book.price + ")");
            }
            
            // XMLList
            var titles:XMLList = catalog..title;
            trace("All titles: " + titles);
        }

        // Test error handling
        private function testErrorHandling():void {
            try {
                var num:int = int("abc"); // This will throw an error
                trace("This line will not be reached.");
            } catch (e:Error) {
                trace("Caught an error: " + e.message);
            } finally {
                trace("Finally block executed.");
            }

            try {
                throw new Error("Custom error occurred!");
            } catch (e:Error) {
                trace("Caught custom error: " + e.message);
            }
        }

        // Test namespaces
        private namespace custom_ns = "http://www.example.com/custom";
        use namespace custom_ns;

        private function testNamespaces():void {
            custom_ns var namespacedVar:String = "Namespaced Value";
            trace("Namespaced variable: " + namespacedVar);
        }

        // Test metadata
        [Deprecated(replacement="newFunction")]
        public function oldFunction():void {
            trace("This is an old function.");
        }

        [Event(name="CUSTOM_EVENT", type="flash.events.Event")]
        public function newFunction():void {
            trace("This is a new function.");
        }

        public function testMetadata():void {
            oldFunction();
            newFunction();
        }
    }
}

// Interface definition
interface ITestInterface {
    function runTest():void;
}

// Base class
class Car {
    public var make:String;
    public var model:String;

    public function Car(make:String, model:String) {
        this.make = make;
        this.model = model;
    }

    public function getDetails():String {
        return make + " " + model;
    }

    public function start():void {
        trace(make + " " + model + " started.");
    }
}

// Subclass with inheritance
class ElectricCar extends Car {
    public var batteryCapacity:int;

    public function ElectricCar(make:String, model:String, capacity:int) {
        super(make, model);
        this.batteryCapacity = capacity;
    }

    public function charge():void {
        trace(make + " " + model + " is charging. Capacity: " + batteryCapacity + " kWh.");
    }

    // Override method
    override public function start():void {
        trace(make + " " + model + " started silently (electric).");
    }
}
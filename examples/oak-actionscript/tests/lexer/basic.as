// ActionScript test file
package com.example {
    public class HelloWorld {
        public function HelloWorld() {
            trace("Hello, World!");
        }
        
        public function addNumbers(a:Number, b:Number):Number {
            return a + b;
        }
        
        private var _name:String = "ActionScript";
        
        public function get name():String {
            return _name;
        }
        
        public function set name(value:String):void {
            _name = value;
        }
    }
}
// Vala Comprehensive Lexer Test
using GLib;
using Gtk;

namespace Demo {

    // Delegate
    public delegate void NotificationHandler(string message);

    // Error Domain
    public errordomain IOError {
        FILE_NOT_FOUND,
        PERMISSION_DENIED
    }

    public class Person : Object {
        // Properties
        public string name { get; set; default = "Unknown"; }
        public int age { get; set; construct; }
        
        // Signal
        public signal void mood_changed(string new_mood);

        public Person(string name, int age) {
            Object(name: name, age: age);
        }

        // Method with error handling
        public void load_data(string filename) throws IOError {
            if (!FileUtils.test(filename, FileTest.EXISTS)) {
                throw new IOError.FILE_NOT_FOUND("File not found: %s".printf(filename));
            }
            // ...
        }

        // Async method
        public async void perform_task() {
            print("Starting task...\n");
            // yield;
            print("Task finished.\n");
        }
    }

    public interface Drawable {
        public abstract void draw();
    }

    // Generics
    public class Wrapper<G> : Object {
        private G data;
        
        public void set_data(G data) {
            this.data = data;
        }
        
        public G get_data() {
            return this.data;
        }
    }

    public struct Point {
        public double x;
        public double y;
    }

    public enum Color {
        RED,
        GREEN,
        BLUE
    }

    public static int main(string[] args) {
        var p = new Person("Alice", 30);
        
        // Lambda
        p.mood_changed.connect((mood) => {
            print("Mood changed to: %s\n", mood);
        });

        // Try-Catch
        try {
            p.load_data("data.txt");
        } catch (IOError e) {
            print("Error: %s\n", e.message);
        }

        // Collections
        var list = new List<string>();
        list.append("Item 1");
        list.append("Item 2");

        foreach (string item in list) {
            print("%s\n", item);
        }
        
        // Nullable types
        string? nullable_str = null;
        if (nullable_str == null) {
            print("It is null\n");
        }

        return 0;
    }
}

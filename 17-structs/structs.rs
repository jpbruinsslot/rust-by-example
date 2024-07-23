// Title: Structs
// Language: Rust
//
// In Rust we're able to define structs, which are a way to group related data
// together. Associated functions to structs are used to specify
// behavior for the structs. There are three types of structs in Rust:
// named-field structs, tuple structs, and unit structs.

#![allow(unused)]
fn main() {

    // **Unit-like** structs have no fields and are useful for implementing a
    // trait on a type without storing any data in the type itself.
    struct UnitLike;

    // **Tuple-like** structs have unnamed fields and are useful for giving a
    // tuple a name, making it a distinct type from other tuples where naming
    // the fields would be redundant.
    struct Color(u8, u8, u8);
    let black = Color(0, 0, 0);

    // Defining a **Named-Field** struct, with fields and methods. The naming
    // convention for structs is `CamelCase`.
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // To define associated functions for a struct, use the `impl` keyword
    // followed by the struct's name and a block containing the functions.
    // These are called associated functions and follow the `snake_case` naming
    // convention.
    impl Rectangle {
        // Associated functions that don't take a struct instance as a parameter
        // are called *type-associated* functions, commonly used as
        // constructors or utility functions.
        fn new(width: u32, height: u32) -> Rectangle {
            Rectangle { width, height }
        }

        // Associated functions that take self as the first parameter are
        // called methods. The `self` argument represents the instance of the
        // struct the method is called on. It doesn't require a type annotation
        // since Rust infers it. Using `self` means the method takes ownership
        // of the instance, consuming it when called.
        fn area(self) -> u32 {
            self.width * self.height
        }

        // In this example, the method borrows the instance by using `&self`,
        // so the instance is not consumed when the method is called. Use this
        // when a method only needs to read, not modify, the instance.
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width 
                && self.height > other.height
        }

        // In this example, the method borrows the instance by using `&mut self`,
        // allowing the method to modify the instance without consuming it.
        // Use this when a method needs to make changes to the instance.
        fn increase_size(&mut self, width: u32, height: u32) {
            self.width += width;
            self.height += height;
        }

        // We can also return `Self` from a method, which is useful for chaining
        // method calls.
        fn set_width(mut self, width: u32) -> Self {
            self.width = width;
            self
        }
    }

    // Creating an instance of the `Rectangle` struct
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Creating an instance of the Rectangle struct using the associated
    // function `new`.
    let rect2 = Rectangle::new(30, 50);

    // Create a struct by using struct update syntax to use the fields of another
    // instance of the same struct.
    let rect3 = Rectangle { width: 30, ..rect1 };

    // Accessing fields of a struct, using the dot notation
    println!("The area of the rect1 is {}", rect1.area());

    // Structs are private by default and only visible within the module they
    // are declared in. Use the `pub` keyword to make a struct public. The same
    // applies to its fields, which are also private by default. Other modules
    // can use the struct and its public associated functions but cannot access
    // private fields by name or use struct expressions to create new values.
    pub struct Person {
        pub name: String,
        age: u8,
    }

    // Rust allows multiple `impl` blocks for a struct, which is useful for
    // separating methods into different files for large structs. Here we
    // define a separate `impl` block for the `Rectangle` struct that we
    // defined earlier.
    impl Rectangle {
        fn print(&self) {
            println!(
                "Rectangle: {} x {}", self.width, self.height,
            );
        }
    }

    // We can set default values for struct fields using the `Default` trait.
    #[derive(Default)]
    struct Point {
        x: i32,
        y: i32,
    }

    // We can instantiate a struct with default values using the `Default` trait.
    let point = Point::default();

    // We can also override some of the default values, when instantiating the
    // struct.
    let other_point = Point {
        x: 10,
        ..Point::default()
    };

    // We can also implement the `Default` trait for our own structs.
    // This is useful when we want to provide a custom default values.
    impl Default for Rectangle {
        fn default() -> Rectangle {
            Rectangle {
                width: 10,
                height: 10,
            }
        }
    }
}

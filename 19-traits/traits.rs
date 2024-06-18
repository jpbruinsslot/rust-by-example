// Title: Traits
// Language: Rust
//
// Traits are Rust's way of defining shared behavior between types. They are
// similar to interfaces or abstract base classes in other languages. A trait
// can be implemented for any type, and it can be used to define a set of
// methods that a type must have in order to be considered a member of that
// trait. This allows you to define shared behavior between types that are not
// related by inheritance.

fn main() {
    // Define a trait named `Shape` with a method signature named `area`.
    trait Shape {
        fn area(&self) -> f64;
    }

    // Define two structs, `Circle` and `Rectangle`, that implement the `Shape`
    // trait. Each struct has its own implementation of the `area` method.
    struct Circle {
        radius: f64,
    }

    impl Circle {
        fn new(radius: f64) -> Circle {
            Circle { radius }
        }
    }

    // Implement the `Shape` trait for the `Circle` struct.
    impl Shape for Circle {
        fn area(&self) -> f64 {
            3.4 * self.radius * self.radius
        }
    }

    struct Rectangle {
        width: f64,
        height: f64,
    }

    impl Rectangle {
        fn new(width: f64, height: f64) -> Rectangle {
            Rectangle { width, height }
        }
    }

    // Implement the `Shape` trait for the `Rectangle` struct.
    impl Shape for Rectangle {
        fn area(&self) -> f64 {
            self.width * self.height
        }
    }

    // Create instances of `Circle` and `Rectangle` and call their `area` methods.
    let circle = Circle::new(5.0);
    let rectangle = Rectangle::new(10.0, 20.0);

    // Define a function that takes a reference to a `Shape` trait object and
    // calls its `area` method. This function can be used to print the area of
    // any type that implements the `Shape` trait. `&dyn Shape` is a reference
    // to a trait object that represents any type that implements the `Shape`
    // trait.
    fn print_area(shape: &dyn Shape) {
        println!("Area: {}", shape.area());
    }

    print_area(&circle);
    print_area(&rectangle);
}

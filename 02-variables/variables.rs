// Title: Variables
// Language: rust
//
// In Rust, variables provide explicit type information, aiding in type safety
// checks by the compiler. Here, we demonstrate variable mutability,
// reassignment, type inference, and explicit type declaration. Additionally,
// Rust allows variable shadowing, enabling the redeclaration of a variable
// with a different type.

fn main() {

    // By default, variables are immutable
    let x = 5;
    println!("The value of x is: {}", x);

    // To make a variable mutable, use the `mut` keyword
    let mut y = 5;
    println!("The value of y is: {}", y);

    // Variables can be reassigned
    y = 6;
    println!("The value of y is: {}", y);

    // Types can be inferred
    let z = 5; // z: i32
    println!("The value of z is: {}", z);

    // Or types can be explicitly declared
    let a: i32 = 5;
    println!("The value of a is: {}", a);

    // Variables can be [shadowed](), the variable `a` is redeclared
    // with a different type, and the value is reassigned.
    let a = "hello, world!";
    println!("The value of a is: {}", a);
}

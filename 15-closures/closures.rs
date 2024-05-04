// Title: Closures
// Language: Rust
//
// Closures are anonymous functions that can capture variables from their
// surrounding environment. Closures can be stored as variables and passed
// around, they can also be used  as arguments to functions. They are declared
// using the `||` syntax.

fn main() {

    // Closures are declared using the `||` syntax. Between those pipes
    // a list of arguments can be passed, and is followed by an expression.
    let addition_v1 = |a: i32, b: i32| -> i32 { a + b };

    // Closures are called just like functions.
    println!("Adding two numbers: {}", addition_v1(1, 2));

    // The return type can be inferred by the compiler, so it can be omitted.
    // Since it is a single expression, the semicolon can be omitted as well.
    let addition_v2 = |c: i32, d: i32| c + d;
    println!("Adding two numbers: {}", addition_v2(2, 3));

    // Arguments and return types are inferred
    let addition_v1 = |e, f| e + f;
    println!("Adding two numbers: {}", addition_v1(3, 4));

    // Closures can capture variables from their surrounding environment. In
    // this case variables in the main function are captured by the closure.
    let x = 1;
    let capture_x = |y| x + y;
    println!("Capturing x: {}", capture_x(2));

    // Closures can also be passed as arguments to functions. This function
    // takes a closure and an integer, and calls the closure with the integer.
    // It implements the `Fn` trait.
    fn call_with_one<F>(closure: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        closure(1)
    }

    let answer = call_with_one(|x| x + 2);
    println!("Answer: {}", answer);
}

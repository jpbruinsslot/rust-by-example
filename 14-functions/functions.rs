// Title: Functions
// Language: rust
//
// Functions are declared using the `fn` keyword. The returned value is
// specified using the `->` arrow. The last expression in the function is the
// return value. Function names are in `snake_case`.

fn main() {

    // Functions are declared using the `fn` keyword. This function takes
    // two ints and returns an int.
    fn add(x: i32, y: i32) -> i32 {
        return x + y;
    }

    let result = add(1, 2);
    println!("Adding two numbers: {}", result);

    // A function with an implicit return, no semicolon is needed.
    fn subtract(x: i32, y: i32) -> i32 {
        x - y
    }

    let result = subtract(2, 1);
    println!("Subtracting two numbers: {}", result);

    // Functions can return multiple values, and this is done using tuples.
    fn swap(x: i32, y: i32) -> (i32, i32) {
        return (y, x);
    }

    let result = swap(1, 2);
    println!("Swapping two numbers: {} {}", result.0, result.1);

    // When functions don't return a value they return an empty tuple `()`
    // known as a *unit* type.
    fn no_return() {}

    let result = no_return();
    println!("No return value: {:?}", result);
}

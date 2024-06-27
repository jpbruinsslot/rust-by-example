// Title: Constants
// Language: rust
//
// Constants are immutable values that are bound to a name and are not allowed
// to change throughout the execution of the program. Constants can be declared
// in any scope, including the global scope, which makes them useful for values
// that many parts of code need to know about.

// Constants are declared using the const keyword. Naming convention for
// constant's is to all upper case characters and underscores for spaces.
const GLOBAL_CONSTANT: u32 = 100_000;

fn main() {
    println!("{}", GLOBAL_CONSTANT);

    // integer
    const ONE: u32 = 1;
    println!("{}", ONE);

    // floating-point number
    const PI: f32 = 3.14159;
    println!("{}", PI);

    // boolean
    const TRUE: bool = true;
    println!("{}", TRUE);

    // character
    const CRAB: char = '🦀';
    println!("{}", CRAB);

    // Tuples, if all members are types that are valid for constants
    const TUPLE: (u32, f32, bool, char) = (ONE, PI, TRUE, CRAB);
    println!("{:?}", TUPLE);

    // Arrays, if all members are types that are valid for constants
    const ARRAY: [u32; 3] = [ONE, ONE, ONE];
    println!("{:?}", ARRAY);

    // The compiler is able to evaluate (limited) constant expressions at
    // compile time and insert the resulting value into the code where the
    // constant is used. This can be helpful for numeric values that you don't
    // want to type out completely.
    const SECONDS_IN_A_DAY: u32 = 60 * 60 * 24;
    println!("{}", SECONDS_IN_A_DAY);
}

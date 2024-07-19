// Title: Error Handling
// Language: Rust
//
// Rust provides a concise and expressive error-handling mechanism using the
// `Result<T, E>` enum. It has two variants: `Ok(T)` for success and `Err(E)` for
// errors. `T` is the type of the success value, and `E` is the type of the error.
// In addition, Rust also provides the `Option` type, which
// is an enum that represents the presence or absence of a value. The `Option`
// type has two variants: `Some`, which contains a value, and `None`, which
// represents the absence of a value.

use std::error;
use std::fmt;
use std::io;
use std::num::ParseIntError;

fn main() -> Result<(), io::Error> {

    // Define a function that returns a `Result` type with a success variant
    // containing a string or an error variant containing an `io::Error`.
    fn divide(x: i32, y: i32) -> Result<String, io::Error> {
        if y == 0 {
            return Err(
                io::Error::new(
                    io::ErrorKind::Other, "Division by zero"
                )
            );
        }
        Ok(format!("{}", x / y))
    }

    // The `Option` type has two variants: `Some`, which contains a value, and
    // `None`, which represents the absence of a value.
    fn divide_option(x: i32, y: i32) -> Option<i32> {
        if y == 0 {
            return None;
        }
        Some(x / y)
    }

    // One way to handle errors is to use a `match` expression. Here we call
    // the `divide` function with different values and pattern match on the
    // result to handle the success and error cases.
    match divide(10, 2) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }

    // The `Option` type can be used to handle cases where a function may or
    // may not return a value. Here we call the `divide_option` function with
    // different values and pattern match on the result to handle the `Some`
    // and `None` cases.
    match divide_option(10, 0) {
        Some(result) => println!("Result: {}", result),
        None => println!("Error: Division by zero"),
    }

    // Using the `match` statement can become verbose. The `?` operator
    // offers us a shorthand way to propagate errors up the call stack,
    // returning them to the caller for handling. It can
    // only be used in functions returning a `Result`.
    let result = divide(10, 2)?;
    println!("Result: {}", result);

    // Typically you'll see multiple `?` operators chained together in a single
    // expression, or in a function that implements the `Result` trait.
    fn sum(a: &str, b: &str) -> Result<i32, ParseIntError> {
        let result = a.parse::<i32>()? + b.parse::<i32>()?;
        Ok(result)
    }

    match sum("13", "37") {
        Ok(result) => println!("The sum is: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // `Result<()>` is often used in functions that perform some operation but
    // do not return a meaningful value upon success. value but may fail. The
    // `()` type represents an empty tuple and is used to signify the absence
    // of a value. Return `Ok(())` to indicate success, again `()` is an empty
    // tuple and signifies the absence of a value.
    fn do_something() -> Result<(), io::Error> {
        Ok(())
    }
    do_something()?;

    // `unwrap()` is a method that will extract the contents of `Ok` variant and
    // assigns it to the variable. If the result is an `Err` variant, it will
    // panic. This is typically used in cases where the programmer is certain
    // that the result will be an `Ok` variant.
    let result = divide(10, 2).unwrap();
    println!("Result: {}", result);

    // `expect()` is similar to `unwrap()` but allows you to provide a custom
    // error message in case of an `Err` variant.
    let result = divide(10, 2).expect("Division by zero");
    println!("Result: {}", result);

    // `unwrap_or()` is a method that will extract the contents of the `Ok`
    // variant and assign it to the variable. If the result is an `Err` variant,
    // it will return the default value provided as an argument.
    let result = divide(10, 0).unwrap_or(
        "Error: Division by zero".to_string()
    );
    println!("{}", result);

    // We can declare a custom error types by defining a struct that implements
    // the `std::error::Error` trait. The `Error` trait requires the `Display`
    // trait to be implemented for the custom error type.
    #[derive(Debug)]
    struct CustomError {
        message: String,
    }

    impl fmt::Display for CustomError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    impl error::Error for CustomError {}

    #[derive(Debug)]
    struct AnotherCustomError;

    impl fmt::Display for AnotherCustomError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Another custom error")
        }
    }

    impl error::Error for AnotherCustomError {}

    // We can define functions that return custom error types by returning
    // `Result<T, E>` where `E` is the custom error type.
    fn f1(i: i32) -> Result<i32, CustomError> {
        if i < 0 {
            return Err(CustomError {
                message: "Custom error".to_string(),
            });
        }
        Ok(i)
    }

    // We can define multiple custom error types and return them from functions.
    // Here we define another custom error type `AnotherCustomError` and return
    // it from a function.
    fn f2(i: i32) -> Result<i32, AnotherCustomError> {
        if i > 0 {
            return Err(AnotherCustomError);
        }
        Ok(i)
    }

    // When a function encounters multiple error types, such as a combination
    // of `Option<T>` and `Result<T, E>` or different `Result<T, E>` types, we
    // can use `Box<dyn Error>` to handle them uniformly. `Box<dyn Error>` is a
    // trait object that can represent any type implementing the `Error` trait,
    // allowing us to return different error types from the same function.
    // Important to note is that we lose the specific error type information
    // when using `Box<dyn Error>`.
    fn f(i: i32) -> Result<i32, Box<dyn error::Error>> {
        let result = f1(i)?;
        let result = f2(result)?;
        Ok(result)
    }

    f(10).unwrap();

    // An alternative is to wrap the error type in a custom enum that implements
    // the `Error` trait. This allows us to define a single error type that
    // can represent multiple error types.
    #[derive(Debug)]
    enum CustomErrorEnum {
        Custom(CustomError),
        Another(AnotherCustomError),
    }

    impl fmt::Display for CustomErrorEnum {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                CustomErrorEnum::Custom(e) => write!(
                    f, "{}", e,
                ),
                CustomErrorEnum::Another(e) => write!(
                    f, "{}", e,
                ),
            }
        }
    }

    impl error::Error for CustomErrorEnum {}

    // We can define functions that return custom error types by returning
    // `Result<T, E>` where `E` is the custom error type.
    fn g(i: i32) -> Result<i32, CustomErrorEnum> {
        if i < 0 {
            return Err(CustomErrorEnum::Custom(
                CustomError{message: "Custom error".to_string()}
            ));
        } else if i > 100 {
            return Err(CustomErrorEnum::Another(
                AnotherCustomError
            ));
        }
        Ok(i)
    }

    g(10).unwrap();

    Ok(())
}

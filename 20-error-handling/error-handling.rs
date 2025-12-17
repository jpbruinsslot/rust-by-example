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

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // ### Returning Errors

    // To return errors in Rust, we use the `Result<T, E>` type. Here is a simple
    // function that performs division and returns a `Result<i32, String>`.
    fn divide(x: i32, y: i32) -> Result<i32, String> {
        if y == 0 {
            return Err("Division by zero".to_string());
        }
        Ok(x / y)
    }

    // Performs checked multiplication; returns an error when an integer
    // overflow occurs.
    fn multiply(a: i32, b: i32) -> Result<i32, String> {
        a.checked_mul(b)
            .ok_or("Integer overflow on multiplication".to_string())
    }

    // To signify the absence of a value upon success, we can use `Result<()>`.
    // `Ok(())` indicates success, while `Err(e)` indicates an error. Using
    // `()`, an empty tuple, signifies the absence of a value.
    fn do_something() -> Result<(), String> {
        Ok(())
    }

    // Uses `Box<dyn Error>` to return different error types uniformly.
    fn divide_and_multiply(
        i: i32,
        j: i32,
        k: i32,
    ) -> Result<i32, Box<dyn Error>> {
        let division = divide(i, j)?;
        let product = multiply(division, k)?;
        Ok(product)
    }

    // Important to note is that we lose the specific error type information
    // when using `Box<dyn Error>`. For real-world applications, creating custom
    // error types is often a better approach as it provides more specific
    // error information.

    // `Result` type alias is a kind of shorthand that can be used to
    // simplify function signatures. Instead of writing `Result<T, Box<dyn Error>>`
    // every time, we can define a type alias like this.
    type MyResult<T> = Result<T, Box<dyn Error>>;

    // Example function using the `MyResult` type alias.
    fn divide_and_multiply_alias(i: i32, j: i32, k: i32) -> MyResult<i32> {
        let calc_result = divide_and_multiply(i, j, k)?;
        Ok(calc_result)
    }

    // ### Handling Errors
    // #### Pattern Matching

    // One way to handle errors is to use a `match` expression. Here we call
    // the `divide` function and pattern match on the result to handle
    // the success and error cases.
    match divide(10, 2) {
        Ok(division_str) => println!("Success: {}", division_str),
        Err(error) => println!("Error: {}", error),
    }

    // Other pattern matching techniques for handling `Result` types are
    // `if let`, and `let else`. This can be useful when you want to handle
    // errors early and avoid nested code, and less verbose than `match`.

    // Use `if let` when you want to **handle** both success and error cases.
    if let Ok(division_str) = divide(10, 2) {
        println!("if let success: {}", division_str);
    } else {
        eprintln!("if let error occurred");
    }

    // Use `let else` when you want to **extract** a value and exit early on error.
    // Making the success value available for the rest of the function scope.
    let Ok(division_str) = divide(20, 4) else {
        return Err("let else encountered an error".into());
    };
    println!("let else success: {}", division_str);

    // #### Result Methods
    // Additionally the `Result<T, E>` type provides several useful methods for
    // working with results. Here are some commonly used methods:

    // `result.ok()` extracts the value from an `Ok` variant, discarding the
    // error and returning `Some(value)`.
    if let Some(value) = divide(15, 3).ok() {
        println!("Converted to Option: {}", value);
    }

    // `result.err()` returns the error value, if any as an `Option<E>`,
    // discarding the success value.
    if let Some(err) = divide(10, 0).err() {
        eprintln!("Handling error with Option: {}", err);
    }

    // `result.is_ok()` returns `true` if the result is an `Ok` variant and
    // `false` if it is an `Err` variant. We can use this method to check if
    // the result is successful before attempting to extract the value.
    if divide(10, 2).is_ok() {
        println!("Division was successful.");
    }

    // `result.is_err()` is a method that returns `true` if the result is an `Err`
    // variant and `false` if it is an `Ok` variant.
    if divide(10, 0).is_err() {
        println!("Division failed.");
    }

    // Another way to handle errors is to propagate them using the `?` operator.
    // The `?` operator can be used to return the error to the caller if the
    // result is an `Err` variant. We essentially say "if this is an error,
    // return it from the current function".
    let final_result = divide_and_multiply(20, 4, 2)?;
    println!("Final Result: {}", final_result);

    // #### Unwrapping Results

    // `unwrap()` extracts the contents of `Ok` variant and assigns it to the
    // variable. If the result is an `Err` variant, **it will panic**, so use
    // with care. This is typically used in cases where the programmer is
    // certain that the result will be an `Ok` variant.
    let unwrapped_division = divide(10, 2).unwrap();
    println!("Unwrapped Result: {}", unwrapped_division);

    // `unwrap_or(fallback)` will extract the contents of the `Ok`
    // variant and assign it to the variable. If the result is an `Err` variant,
    // it will return the default value provided as an argument.
    let value_or_default = divide(10, 2).unwrap_or(0);
    println!("Value with fallback: {}", value_or_default);

    // `unwrap_or_else(fallback_fn)` will extract the contents of the `Ok`
    // variant and assign it to the variable. If the result is an `Err`
    // variant, it will call the provided closure and return its result.
    // This is useful for providing a computed fallback value based on the error.
    let computed_fallback = divide(10, 0).unwrap_or_else(|err| {
        eprintln!("Error occurred: {}. Providing default value.", err);
        -1
    });
    println!("Computed Fallback: {}", computed_fallback);

    // `expect(message)` is similar to `unwrap()` but allows you to provide a custom
    // error message in case of an `Err` variant.
    let expected_value = divide(10, 2).expect("Division should succeed");
    println!("Expected Value: {}", expected_value);

    // #### Mapping Results

    // `map(convert_fn)` allows to change the value in the `Result` only if
    // it is an `Ok` variant. The function (usually a closure) is applied
    // to the contents of the `Ok` variant, and a new `Result` is returned
    // with the transformed value, if it is an `Err` variant, it is returned unchanged.
    let result_with_mapped_value = divide(10, 2).map(|val| val * 2);
    println!("Mapped Value Result: {:?}", result_with_mapped_value);

    // `map_err(convert_fn)` like `map`, but it applies the function to the contents
    // of the `Err` variant instead. This is useful for transforming error types
    // or adding additional context to errors.
    let result_with_mapped_err =
        divide(10, 0).map_err(|e| format!("Custom error: {}", e));
    println!("Mapped Error Result: {:?}", result_with_mapped_err);

    Ok(())
}

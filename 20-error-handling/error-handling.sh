# Language: shell
$ rustc error-handling.rs
$ ./error-handling
Success: 5
if let success: 5
let else success: 5
Converted to Option: 5
Handling error with Option: Division by zero
Division was successful.
Division failed.
Final Result: 10
Unwrapped Result: 5
Value with fallback: 5
Error occurred: Division by zero. Providing default value.
Computed Fallback: -1
Expected Value: 5
Mapped Value Result: Ok(10)
Mapped Error Result: Err("Custom error: Division by zero")

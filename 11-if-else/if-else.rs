// Title: If/Else
// Language: rust
//
// The `if` statement assesses a conditional expression, determining the control flow based on
// whether the expression evaluates to true or false.

fn main() {

    // If/else statements are similar to other languages.
    if true {
        println!("true");
    } else if false {
        println!("false");
    } else {
        println!("neither");
    }

    // The `if` statement can return a value and can be used in a `let`
    // statement.
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // Using `if let` expression, control is determined through pattern
    // matching instead of a conditional expression.
    let result: Option<i32> = Some(42);
    if let Some(value) = result {
        println!("The value is: {}", value);
    } else {
        println!("There is no value");
    }
}

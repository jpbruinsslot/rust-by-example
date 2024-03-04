// Title: Match
// Language: rust
//
// Rust has support for pattern matching through the `match` statement. It is
// somewhat similar to the `switch` statement in other languages.

fn main() {

    // Match statements are similar to switch statements in other languages.
    let value = 1;
    match value {
        // Match with a single pattern.
        1 => println!("one"),

        // Match with multiple patterns.
        2 | 3 => println!("two or three"),

        // Match with a range.
        2..=5 => println!("two through five"),

        // Match with a guard. A guard is an additional condition
        // that must be true for the pattern to match.
        x if x > 5 => println!("greater than five"),

        // Match with a binding. The `@` operator is used to create a binding
        // for the value that matched the pattern.
        x @ 12..=20 => println!("twelve through twenty: {}", x),

        // Match with a wildcard. The `_` pattern matches anything.
        _ => println!("something else"),
    }
}

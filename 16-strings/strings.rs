// Title: Strings
// Language: rust
//
// In Rust there are two main types of strings: string slices (`&str`) and the
// `String` type. String slices are references to some UTF-8 encoded string
// data stored elsewhere. The `String` type, on the other hand, is allocated
// on the heap and is therefore able to store an amount of text that is unknown
// to us at compile time.

fn main() {

    // String slice is an immutable reference to a UTF-8 encoded string in memory
    // that is not owned by the program. String slices are written as `&str`.
    let s = "hello, world!";
    let first_word = &s[0..5];
    let second_word = &s[7..12];
    println!("{} {}", first_word, second_word);

    // String literal, fixed size and immutable string slice, and are stored in
    // the program's binary as immutable data. Therefore, string literals are
    // considered string slices (`&str`).
    let literal: &str = "hello, world!";
    println!("{}", literal);

    // `String` type, a mutable UTF-8 encoded string allocated on the heap, and
    // can dynamically grow and shrink during runtime.
    let str_from: String = String::from("hello, world!");
    println!("{}", str_from);

    // Create `String` type with `new()`
    let mut str_new: String = String::new();
    str_new.push_str("hello, world!");
    println!("{}", str_new);

    // Create `String` type with `format!` ([docs](https://doc.rust-lang.org/std/fmt/))
    let world: &str = "world!";
    let str_format: String = format!("hello, {}", world);
    println!("{}", str_format);

    // Create `String` type with `to_string()`
    let str_to_str: String = "hello, world!".to_string();
    println!("{}", str_to_str);
}

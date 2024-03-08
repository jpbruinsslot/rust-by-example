// Title: Strings
// Language: rust
//
// In Rust there are two main types of strings: string slices (`&str`) and the
// `String` type. String slices are references to some UTF-8 encoded string
// data stored elsewhere. The `String` type, on the other hand, is allocated
// on the heap and is therefore able to store an amount of text that is unknown
// to us at compile time.

fn main() {
    // String slice, an immutable reference to a UTF-8 encoded string in memory
    // that is not owned by the program. String slices are written as `&str`.
    let s: &str = "hello, world!";
    let slice: &str = &s[0..5];
    println!("{}", slice);

    // String literal, fixed size and immutable string slice, and are stored in
    // the program's binary as immutable data. Therefore, string literals are
    // considered string slices (`&str`).
    let literal: &str = "hello, world!";
    println!("{}", literal);

    // String type, a mutable UTF-8 encoded string allocated on the heap, and
    // can dynamically grow and shrink during runtime.
    let mut str_new: String = String::new();
    str_new.push_str("hello, world!");
    println!("{}", str_new);

    // String type with `from()`
    let str_from: String = String::from("hello, world!");
    println!("{}", str_from);

    // String type with `format!`
    let str_format: String = format!("hello, world!");
    println!("{}", str_format);

    // String type with `to_string()`
    let str_to_str: String = "hello, world!".to_string();
    println!("{}", str_to_str);

    // String type with `push_str()`
    let mut str_push_str: String = String::from("hello, ");
    str_push_str.push_str("world!");
    println!("{}", str_push_str);

    // String type with `+` operator
    let str_op: String = String::from("hello, ") + "world!";
    println!("{}", string_op);

    // String type with `with_capacity`
    let cap = 13;
    let mut str_cap: String = String::with_capacity(cap);
    str_cap.push_str("hello, world!");
    println!("{}", str_cap);
}

// Title: Ownership
// Language: Rust
//
// Rust uses a system of ownership to ensure memory safety without garbage
// collection. Rust enforces this by enforcing these ownership rules:
// 1. Each value in Rust has a variable that is its **owner**.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value is dropped.

#![allow(unused)]
fn main() {

    // Simple scalar values are **copied** when assigned to another variable.
    let x = 5;
    let y = x;

    // s1 is **moved** to s2, and **ownership** of s1 is invalidated, The
    // destination now controls the value's **lifetime**.
    let s1 = String::from("hello");
    let s2 = s1;

    // Using a value without transferring ownership, is called **borrowing**.
    // Borrowing is done using references. References are immutable by default.
    let s3 = String::from("hello");
    let len = calculate_length(&s3);

    // Mutable references, denoted by `&mut`, allow modification of the
    // borrowed variable's value without transferring ownership.
    let mut s4 = String::from("hello");
    let s5 = &mut s4;
    *s5 = String::from("hello, world!");

    // Passing a value to a function moves or copies it, depending on the type.
    // Here, the value is **moved** into the function, invalidating the original
    // variable, which causes an error if used afterwards:
    // `error: value borrowed here after move`.
    let s6 = String::from("hello");
    takes_ownership(s6);

    // Return values of functions can also transfer ownership. Here the return
    // value is moved into `s7`.
    let s7 = gives_ownership();

    // Returning a value from a function moves or copies it, depending on the
    // type. Here, `s8` is moved into the function, which then moves its return
    // value into `s9`. Using `s8` afterward results in the error:
    // `error: value borrowed here after move`.
    let s8 = String::from("hello");
    let s9 = takes_and_gives_back(s8);

    // Allow a function to use a value without taking ownership by using
    // references. The function takes a reference, leaving the original
    // variable valid, a process called **borrowing**.
    let s10 = String::from("hello");
    let len = calculate_length(&s10);

    // References are immutable by default (`&`) and cannot be modified,
    // leading to the error: `error: cannot borrow as mutable`. Mutable
    // references (`&mut`) allow value modification without transferring
    // ownership.
    let mut s12 = String::from("hello");
    change(&mut s12);
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(i: i32) {
    println!("{}", i);
}

fn gives_ownership() -> String {
    let s = String::from("hello");
    return s;
}

fn takes_and_gives_back(s: String) -> String {
    return s;
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}

fn change(s: &mut String) {
    s.push_str(", world");
}

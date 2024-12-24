// Title: Ownership
// Language: Rust
//
// Rust uses a system of ownership to ensure memory safety without garbage
// collection. Rust enforces this by enforcing these ownership rules:
// 1. Each value in Rust has a variable that is its **owner**.
// 2. There can only be **one owner** at a time.
// 3. When the owner goes **out of scope**, the value is dropped.

#![allow(unused)]
fn main() {

    // Simple scalar values are **copied** when assigned to another variable.
    // And can use both variables without issue. Both `x` and `y` are valid and
    // accessible. If `x` were moved instead of copied, attempting to use `x`
    // after assigning it to `y` would result in a compile-time error.
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);

    // This is different in the following example, here `s1` is **moved** to
    // `s2`, and **ownership** of `s1` is invalidated, The destination now
    // controls the value's **lifetime**. When printing `s1` after moving
    // ownership to `s2` will results in the error:
    // `error: use of moved value: 's1'`.
    let s1 = String::from("hello");
    let s2 = s1;

    // Using a value without transferring ownership, is called **borrowing**.
    // Borrowing is done using **references**, which are denoted by `&` and are
    // immutable by default. You can use the reference to access the value,
    // and you can reference `z` directly because the ownership was not
    // transferred, however you cannot modify it.
    let z = 42;
    let z_ref = &z;
    println!("z: {}, z_ref: {}", z, z_ref);

    // When it is necessary to modify a borrowed value, a **mutable reference**
    // is used, denoted by `&mut`. Mutable references allow modification of these
    // borrowed variable's value without transferring ownership.
    let mut s4 = String::from("hello");
    let s5 = &mut s4;
    *s5 = String::from("hello, world!");
    println!("s5: {}", s5);

    // Passing a value to function *moves* or *copies* it, depending on the type.
    // In this example, the value is **moved** into the function and ownership
    // is transferred, invalidating the original variable, which causes an error
    // if used afterwards: `error: value borrowed here after move`.
    fn takes_ownership(s: String) {
        println!("took ownership of: {}", s);
    }
    let s6 = String::from("hello");
    takes_ownership(s6);

    // Also returning a value from a function moves or copies it, depending on
    // the type. Here the return value is moved into `s7`.
    fn gives_ownership() -> String {
        let s = String::from("hello");
        return s;
    }
    let s7 = gives_ownership();

    // A combination of both moving and returning a value is also possible.
    // Here, `s8` is moved into the function, which then moves its
    // return value into `s9`. Using `s8` afterward results in the error:
    // `error: value borrowed here after move`.
    fn takes_and_gives_back(s: String) -> String {
        return s;
    }
    let s8 = String::from("hello");
    let s9 = takes_and_gives_back(s8);

    // When we want to allow a function to use a value without taking ownership
    // we can achieve that by using references. The function takes a *reference*
    // `&String`, leaving the original variable valid, a process called
    // **borrowing**.
    fn calculate_length(s: &String) -> usize {
        return s.len();
    }
    let s10 = String::from("hello");
    let length = calculate_length(&s10);

    // References (`&)` are immutable by default and cannot be modified,
    // leading to the error: `error: cannot borrow as mutable`. Mutable
    // references (`&mut`) allow value modification without transferring
    // ownership.
    fn change(s: &mut String) {
        s.push_str(", world");
    }
    let mut s12 = String::from("hello");
    change(&mut s12);
}

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

    // Within a scope, a value is valid and accessible. When the scope ends,
    // the value is dropped and the memory is deallocated.
    {
        let msg = String::from("hello, world!");
        println!("'{}' is only valid within this scope", msg);
    }

    // Simple scalar values are *copied* when assigned, allowing both variables
    // to remain valid and accessible. Since scalars are stored on the stack,
    // the value is duplicated in the new variable. If `x` were moved instead
    // of copied, using `x` after assigning it to `y` would cause a compile-time
    // error.
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);

    // In this example, moving `s1` to `s2` invalidates `s1`'s ownership,
    // transferring control of the value's lifetime to `s2`. Since `String`
    // stores a pointer on the stack pointing to heap data, only the pointer
    // is copied during the move, not the heap value. This ensures a single
    // owner of the value at any time, preventing multiple variables from
    // pointing to the same content.
    let s1 = String::from("hello");
    let s2 = s1;

    // Using a value without transferring ownership is called **borrowing**, done
    // via references (`&`), which are immutable by default. A reference allows
    // access to the value without transferring ownership, but it cannot be
    // modified.
    let z = 42;
    let z_ref = &z;
    println!("z: {}, z_ref: {}", z, z_ref);

    // To modify a borrowed value, use a mutable reference (`&mut`), which
    // allows changes without transferring ownership.
    let mut s4 = String::from("hello");
    let s5 = &mut s4;
    *s5 = String::from("hello, world!");
    println!("s5: {}", s5);

    // Passing a value to a function also *moves* or *copies* it based on its
    // type. For the `String` type , the value is moved, transferring ownership
    // to the function and invalidating the original variable, as `String` data
    // is stored on the heap.
    fn takes_ownership(s: String) {
        println!("took ownership of: {}", s);
    }
    let s6 = String::from("hello");
    takes_ownership(s6);

    // Scalar values are copied when passed to a function, and the original
    // variable remains valid. This is because scalar values are stored on the
    // stack, and the value is copied into the function.
    fn copies(x: i32) {
        println!("copied: {}", x);
    }
    let x = 5;
    copies(x);

    // Returning a value from a function also *moves* or *copies* it, depending
    // on the type. Here the return value is moved into `s7`.
    fn gives_ownership() -> String {
        let s = String::from("hello");
        return s;
    }
    let s7 = gives_ownership();

    // A combination of both moving and returning a value is also possible.
    // Here, `s8` is moved into the function, which then moves its
    // return value into `s9`.
    fn takes_and_gives_back(s: String) -> String {
        return s;
    }
    let s8 = String::from("hello");
    let s9 = takes_and_gives_back(s8);

    // When we want to allow a function to use a value without taking ownership
    // we can achieve that by also using references. The function takes a
    // *reference* `&String`, leaving the original variable valid, a process
    // called **borrowing**.
    fn calculate_length(s: &String) -> usize {
        return s.len();
    }
    let s10 = String::from("hello");
    let length = calculate_length(&s10);

    // Mutable
    // references (`&mut`) allow value modification without transferring
    // ownership.
    fn change(s: &mut String) {
        s.push_str(", world");
    }
    let mut s12 = String::from("hello");
    change(&mut s12);
}

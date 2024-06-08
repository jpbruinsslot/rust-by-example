// Title: Ownership
// Language: Rust
//
// dangling pointers, double free, memory leak, null pointers, data races

fn main() {
    // Simple scalar values, and nothing that requires allocation or is some
    // form of resource are copied when assigned to another variable.
    // They implement the `Copy` trait.
    let x = 5;

    // x is **copied** to y because x is a primitive type (scalar value?)
    let y = x;

    // s1 is **moved** to s2, and **ownership** of s1 is invalidated, The
    // destination now controls the value's **lifetime**.
    let s1 = String::from("hello");
    let s2 = s1;

    // Using a value without transferring ownership, is called **borrowing**.
    // Borrowing is done using references. References are immutable by default.
    let s3 = String::from("hello");
    let len = calculate_length(&s3);
    println!("The length of '{}' is {}.", s3, len);

    // Mutable references are used to mutate the value they borrow. It allows
    // to change the value of the borrowed variable without transferring
    // ownership. The syntax for mutable references is `&mut`.
    let mut s4 = String::from("hello");
    let s5 = &mut s4;
    *s5 = String::from("hello, world!");

    // Passing a value to a function will move or copy, depending on the type.
    // Here the value is **moved** into the function, and the original variable
    // is invalidated. We can check that by trying to use the variable after
    // passing it to the function. This will result in the error: `error: value
    // borrowed here after move`
    let s6 = String::from("hello");
    takes_ownership(s6);
    println!("{}", s6);

    // Return values of functions can also transfer ownership. Here the return
    // value is moved into `s7`.
    let s7 = gives_ownership();
    println!("{}", s7);

    // Returning a value from a function will move or copy, depending on the
    // type. Here the value is moved to the calling function, and the original
    // variable is invalidated. `s8` is moved into the function, which also
    // moves its return value into `s9`. When we try to use `s8` after passing
    // it to the function, we will get the error: `error: value borrowed here
    // after move`
    let s8 = String::from("hello");
    let s9 = takes_and_gives_back(s8);
    println!("{}", s8);

    // Let a function use a value but not take ownership of it, by the use of
    // references. Here the function takes a reference to the value, and the
    // original variable is still valid. This is called **borrowing**.
    let s10 = String::from("hello");
    let len = calculate_length(&s10);
    println!("The length of '{}' is {}.", s10, len);

    // If we try to change the value of a borrowed variable, we will get the
    // following error: `error: cannot borrow s11 as mutable, as it is not
    // declared as mutable`
    let s11 = String::from("hello");
    cant_change(&s11);

    // Mutable references are used to mutate the value they borrow. It allows
    // to change the value of the borrowed variable without transferring
    // ownership. The syntax for mutable references is `&mut`.
    let mut s12 = String::from("hello");
    change(&mut s12);
    println!("{}", s12);
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

fn cant_change(s: &String) {
    s.push_str(", world");
}

fn change(s: &mut String) {
    s.push_str(", world");
}

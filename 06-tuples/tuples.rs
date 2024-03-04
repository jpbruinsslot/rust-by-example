// Title: Compound Types - Tuples
// Language: rust
//
// Tuples are a general way of grouping together a number of values with a
// variety of types into one compound type. Tuples have a fixed length: once
// declared, they cannot grow or shrink in size.

fn main() {

    // Here we create a tuple type with elements of different types
    // and then we destructure it into three variables.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);

    // We can also access a tuple element directly by using a period
    // followed by the index of the value we want to access.
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("five_hundred: {}", five_hundred);
    println!("six_point_four: {}", six_point_four);
    println!("one: {}", one);
}

// Title: Collections - Vectors
// Language: Rust
//
// The type `Vec<T>` represents a growable array type to is heap-allocated, so
// you can resize vectors, push new items to it, delete items, append other
// vectors to them, and more. You can only store values of the same type in a
// vector. Check [the documentation](https://doc.rust-lang.org/std/vec/struct.Vec.html)
// for the most common methods and operations that can be performed on vectors.

fn main() {

    // Initialize a new vector with the `vec!` macro
    let mut v = vec![1, 2, 3, 4, 5];

    // We can explicitly declare the type of the vector
    let mut vec_explicit = Vec::<i32>::new();
    vec_explicit.push(1);
    vec_explicit.push(2);
    vec_explicit.push(3);

    // And also implicitly declare the type of the vector
    let mut vec_implicit = Vec::new();
    vec_implicit.push("one");
    vec_implicit.push("two");
    vec_implicit.push("three");

    // When you know the number beforehand you can create a vector by
    // calling `with_capacity` method to create a vector with a certain
    // capacity. This way the vector can grow without needing to allocate
    // more memory each time it grows.
    let mut vec_with_capacity = Vec::with_capacity(10);
    vec_with_capacity.push(1);
    vec_with_capacity.push(2);
    vec_with_capacity.push(3);
    println!("Length is {}", vec_with_capacity.len());
    println!("Capacity is {}", vec_with_capacity.capacity());

    // Another possibility is to build a vector from the values produced by an
    // iterator by using the `collect()` method.
    let vec_from_iter: Vec<i32> = (0..5).collect();
    println!("Vector from iterator: {:?}", vec_from_iter);

    // To access elements of a vector we can use the `[]` operator with the
    // index of the element we want to access. If the index is out of bounds
    // and the program will crash.
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // Or we can use the `get()` method which returns an `Option<&T>`. If the
    // index is out of bounds it will return `None`.
    let does_not_exist = v.get(42);
    match does_not_exist {
        Some(value) => println!("42nd element is {}", value),
        None => println!("No 42nd element"),
    }

    // We can iterate over the elements of a vector using a `for` loop. We use
    // `iter()` to get an immutable reference to each element of the vector.
    for i in v.iter() {
        println!("{}", i);
    }

    // To store different types in a vector we can use an enum
    enum Variant {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let var_vec = vec![
        Variant::Int(3),
        Variant::Float(3.14),
        Variant::Text(String::from("hello, world!")),
    ];

    for i in var_vec {
        match i {
            Variant::Int(i) => println!("Integer: {}", i),
            Variant::Float(f) => println!("Float: {}", f),
            Variant::Text(s) => println!("Text: {}", s),
        }
    }

    // Slicing a vector
    let first_three = &v[0..3];
    for i in first_three {
        println!("{}", i);
    }

    // Insert an element at a specific index, shifting all elements after it to
    // the right
    v.insert(2, 10);
    println!("Vector after inserting 10 at index 2: {:?}", v);

    // Remove an element at a specific index
    let removed = v.remove(2);
    println!("Removed element is {}", removed);

    // Popping the last element of a vector
    let popped = v.pop();
    println!("Popped value is {:?}", popped);

    // Clear a vector
    v.clear();
    println!("Vector after clearing: {:?}", v);
}

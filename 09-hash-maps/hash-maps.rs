// Title: Collections - Hash Maps
// Language: Rust
//
// `Hashmap<K, V>` is a collection of key-value pairs, where `K` is the type of
// the key and `V` is the type of the value. The keys of the `HashMap` can be
// booleans, integers, string or any type that implements the `Eq` and `Hash`
// traits. Hash maps store their data on the heap, they are growable, but can
// shrink themselves when they have excess space. Check the
// [documentation](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
// for the most common methods and operations that can be performed on hash maps.

use std::collections::HashMap;

fn main() {

    // Create a new hashmap with `HashMap::new()`.
    let mut ages = HashMap::new();

    // Create a hashmap with initial key-value pairs using `HashMap::from()`.
    let _contacts = HashMap::from([
        ("Alice", "123-4567"),
        ("Bob", "234-5678"),
        ("Charlie", "345-6789"),
    ]);

    // Insert key-value pairs into the `HashMap`.
    ages.insert("Alice", 42);
    ages.insert("Bob", 24);
    ages.insert("Charlie", 36);

    // Insert a key-value pair only if the key is not present.
    ages.entry("Alice").or_insert(28);

    // Retrieve the value associated with a key, in combination with `if let`.
    if let Some(age) = ages.get("Alice") {
        println!("Alice's age is {}", age);
    } else {
        println!("Alice's age is not available");
    }

    // Retrieve the value associated with a key, in combination with `match`
    match ages.get("Alice") {
        Some(age) => println!("Alice's age is {}", age),
        None => println!("Alice's age is not available"),
    }

    // Updating a value in the `HashMap` by overwriting
    if let Some(age) = ages.insert("Alice", 43) {
        println!("Alice's previous age was {}", age);
    }

    // Updating a value based on the old value.
    let age = ages.entry("Alice").or_insert(0);
    *age += 1;
    println!("Alice's age is {}", ages.get("Alice").unwrap());

    // Removing a key-value pair
    ages.remove("Bob");

    // Removing a key-value pair and getting the value, in combination with
    // `if let`
    if let Some(age) = ages.remove("Bob") {
        println!("Bob's age was {}", age);
    }

    // Removing a key-value pair and getting the value, in combination with
    // `match`
    match ages.remove("Bob") {
        Some(age) => println!("Bob's age was {}", age),
        None => println!("Bob's age is not available"),
    }

    // Check if a key exist
    if ages.contains_key("Charlie") {
        println!(
            "Charlie's's age is {}",
            ages.get("Charlie").unwrap(),
        );
    }

    // Check if a key exists, in combination with `match`
    match ages.contains_key("Charlie") {
        true => println!(
            "Charlie's age is {}",
            ages.get("Charlie").unwrap(),
        ),
        false => println!(
            "Charlie's age is not available",
        ),
    }

    // Iterate over the key-value pairs
    for (name, age) in ages.iter() {
        println!("{}: {:?}", name, age);
    }

    // Iterate over the key-value pairs and modify the value
    for (_, age) in ages.iter_mut() {
        *age += 1;
    }

    // Iterate over just the keys
    for name in ages.keys() {
        println!("{}", name);
    }

    // Iterate over just the values
    for age in ages.values() {
        println!("{:?}", age);
    }

    // Check the number of elements in the `HashMap`
    println!("The number of elements are {}", ages.len());

    // Check if the `HashMap` is empty
    println!("Is empty? {}", ages.is_empty());

    // Clear all key-value pairs
    ages.clear();
}

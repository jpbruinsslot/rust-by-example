// Title: Collections - Sets
// Language: Rust
//
// Sets are a collection of values where each value must be unique, it allows
// for fast membership testing. Sets in Rust are implemented as a thin wrapper
// around `HashMap<T, ()>`. The `()` is a zero-sized type, which means that
// each value in the HashMap is just a key with no associated value.

use std::collections::HashSet;

fn main() {
    // Creating a new set
    let mut a: HashSet<i32> = HashSet::new();

    // Inserting values into the set
    a.insert(1);
    a.insert(2);
    a.insert(3);

    // Trying to insert a value that is already in the set, returns false if
    // this value is already in the set
    a.insert(2);

    // Removing a value from the set, returns true if the value was in the set
    a.remove(&2);

    // You can also create a set by using the `collect()` method with an
    // iterator. You can use any iterator that yields elements, such as a
    // range, a vector, or any other collection.
    let b: HashSet<i32> = [3, 4, 5].iter().cloned().collect();

    // Checking if a value is in the set by using `contains()`
    println!("Contains 1: {}", a.contains(&1));

    // `HashSet` supports whole-set operations like `union`,
    println!(
        "Union: {:?}", a.union(&b).collect::<HashSet<&i32>>()
    );

    // `intersection`,
    println!(
        "Intersection: {:?}",
        a.intersection(&b).collect::<HashSet<&i32>>()
    );

    // `difference`,
    println!(
        "Difference: {:?}",
        a.difference(&b).collect::<HashSet<&i32>>()
    );

    // and `symmetric difference`
    println!(
        "Symmetric Difference: {:?}",
        a.symmetric_difference(&b).collect::<HashSet<&i32>>()
    );

    // `HashSet` also supports three methods for testing relationships between
    // sets. `is_subset` is true if all elements in this set are in the other set.
    println!("Is a subset of: {}", a.is_subset(&b));

    // `is_superset` is true if all elements in the other set are in this set
    println!("Is a superset of: {}", a.is_superset(&b));

    // `is_disjoint` is true if the sets have no elements in common
    println!("Is disjoint: {}", a.is_disjoint(&b));

    // In order to iterate over a set we can use `.iter()`
    for x in a.iter() {
        println!("{}", x);
    }
}

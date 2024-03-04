// Title: Compound Types - Arrays
// Language: rust
//
// The array type is a compound type that allows you to store multiple values
// of the same type next to each other in memory. Arrays are useful when you
// want your data allocated on the stack rather than the heap or when you want
// to ensure you always have a fixed number of elements.

fn main() {

    // Initialize array with values of same type
    let arr_inferred = [1, 2, 3, 4, 5];
    let arr_explicit: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr_inferred: {:?}", arr_inferred);
    println!("arr_explicit: {:?}", arr_explicit);

    // Initialize array with same value
    let arr_same = [3; 5];
    println!("arr_same: {:?}", arr_same);

    // We can access an array element directly by using brackets and the index
    // of the value we want to access.
    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];
    let second = arr[1];
    println!("first: {}, second: {}", first, second);

    // Slicing arrays
    let slice = &arr[1..3];
    let rest = &arr[1..];
    let all = &arr[..];
    println!("slice: {:?}", slice);
    println!("rest: {:?}", rest);
    println!("all: {:?}", all);

    // Iterating over arrays
    for element in arr.iter() {
        println!("element: {}", element);
    }
}

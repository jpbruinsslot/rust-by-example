// Title: Generics
// Language: Rust
//
// Generics are a feature of Rust that allow you to define functions, structs,
// enums, and traits that can operate on any type. Generics are a powerful
// feature that allow you to write code that is more flexible and reusable.
// Generics differ from traits in that generics generalize types, while traits
// generalize code.
//
// In Rust generics are defined using angle brackets (`< >`) and by convention
// are typically named using a single upper cased letter (e.g. `T`). Generics
// can be used with functions, structs, enums, and traits.

use std::fmt::Display;

fn main() {

    // Here we define a **generic function** `first_element` over some type `T`
    // that takes a slice of any type `T` and returns an `Option` containing a
    // reference to the first element in the slice.
    fn first_element<T>(slice: &[T]) -> Option<&T> {
        if slice.is_empty() {
            None
        } else {
            Some(&slice[0])
        }
    }

    let numbers = vec![10, 20, 30];
    let floats = vec![1.1, 2.2, 3.3];
    let words = vec!["apple", "banana", "cherry"];

    // This allows us to call the `first_element` function with slices containing
    // elements of any type. In this case, we call the `first_element` function
    // with slices containing integers, floats, and strings.
    let first_float = first_element(&floats);
    let first_number = first_element(&numbers);
    let first_word = first_element(&words);

    // We can also define generic functions with multiple type parameters.
    // The `combine` function is a generic function that takes two arguments
    // of different types `T` and `U` and returns a tuple containing both values.
    fn combine<T, U>(x: T, y: U) -> (T, U) {
        (x, y)
    }

    let number_and_float = combine(10, 1.1);
    let word_and_number = combine("apple", 10);

    // We can also use **trait bounds** to restrict the types that can be used with
    // a generic function. In this case, the `largest` function can only be used
    // with types that implement the `PartialOrd` trait. This allows us to use
    // the `>` operator to compare values of type `T`.
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let largest_number = largest(&numbers);
    let largest_float = largest(&floats);
    let largest_word = largest(&words);

    // We can also use **multiple trait bounds** to restrict the types that can be used
    // with a generic function. In this case, the `max` function can only be
    // used with types that implement both the `Display` and `PartialOrd` traits.
    fn max<T: Display + PartialOrd>(x: T, y: T) -> T {
        if x > y {
            x
        } else {
            y
        }
    }

    let max_number = max(10, 20);
    let max_float = max(1.1, 2.2);
    let max_word = max("apple", "banana");

    // Structs can also be generic. In this case, `GenericStruct` is a
    // generic struct that takes a type `T` as a generic type parameter.
    // It also has a field `field` of the generic type `T`.
    struct GenericStruct<T> {
        field: T,
    }

    // You can define methods on instances of `GenericStruct<T>` using the `impl` block.
    // In this case, the `get_field` method returns a reference to the `field` of the struct.
    impl<T> GenericStruct<T> {
        fn get_field(&self) -> &T {
            &self.field
        }
    }

    let number_struct = GenericStruct { field: 10 };
    let float_struct = GenericStruct { field: 1.1 };
    let word_struct = GenericStruct { field: "apple" };

    // To specify a constraint when defining a method on a generic struct, you can
    // use the `impl` block with the specific type. In this case, the
    // `get_float_field` method is only available for instances of
    // `GenericStruct<f32>`.
    impl GenericStruct<f32> {
        fn get_float_field(&self) -> f32 {
            self.field
        }
    }

    // To specify constraints on the generic type `T`, you can use trait bounds.
    // In this case, the `DisplayStruct` can only be used with types that
    // implement the `Display` trait.
    struct DisplayStruct<T: Display> {
        field: T,
    }

    // We can also use generics with enums. In this case, the `HTTPResp` enum
    // is a generic enum that can be either a `Success` or `Error` response.
    enum HTTPResp<T> {
        Success(T),
        Error(T),
    }

    // Now we can create instances of the `HTTPResp` enum with different types.
    let resp1: HTTPResp<i32> = HTTPResp::Success(200);
    let resp2: HTTPResp<&str> = HTTPResp::Success("OK");
    let err1: HTTPResp<i32> = HTTPResp::Error(404);
    let err2: HTTPResp<&str> = HTTPResp::Error("Not Found");

    // In this case, the `handle_response` function takes an `HTTPResp`
    // enum as an argument and prints the value of the response.
    fn handle_response<T: Display>(response: HTTPResp<T>) {
        match response {
            HTTPResp::Success(value) => 
                println!("Success: {}", value),
            HTTPResp::Error(err) => 
                println!("Error: {}", err),
        }
    }

    handle_response(resp1);
    handle_response(resp2);
    handle_response(err1);
    handle_response(err2);

    // We can also define **generic traits**. In this case, the `Sorter` trait
    // is a generic trait that takes a type `T` as a generic type parameter.
    // This allows us to define methods that operate on slices of any type `T`.
    trait Sorter<T> {
        fn sort(&self, slice: &mut [T]);
    }

    struct SelectionSorter;

    // Implement the `Sorter` trait for the `SelectionSorter` struct for any
    // type `T` that implements the `PartialOrd` trait.
    impl<T: PartialOrd> Sorter<T> for SelectionSorter {
        fn sort(&self, slice: &mut [T]) {
            for i in 0..slice.len() {
                let mut min_index = i;
                for j in i + 1..slice.len() {
                    if slice[j] < slice[min_index] {
                        min_index = j;
                    }
                }
                slice.swap(i, min_index);
            }
        }
    }

    struct BubbleSorter;

    // Implement the `Sorter` trait for the `BubbleSorter` struct for any
    // type `T` that implements the `PartialOrd` trait.
    impl<T: PartialOrd> Sorter<T> for BubbleSorter {
        fn sort(&self, slice: &mut [T]) {
            for i in 0..slice.len() {
                for j in 0..slice.len() - 1 {
                    if slice[j] > slice[j + 1] {
                        slice.swap(j, j + 1);
                    }
                }
            }
        }
    }

    let mut numbers = vec![42, 1337, 28, 10];
    let sorter = SelectionSorter;
    sorter.sort(&mut numbers);

    let mut words = vec!["cherry", "apple", "banana"];
    let sorter = BubbleSorter;
    sorter.sort(&mut words);
}

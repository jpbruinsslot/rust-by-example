// Title: *Coming* Pointers
// Language: Rust

fn main() {
    // `&` is used to create a reference to a value, and it allows us to
    // *borrow* the value without taking ownership of it. As such, the lifetime
    // of the reference depends on its owner and is limited to the scope in
    // which it was created.

    // Therefore we would need to add lifetime annotations to the function
    // signature to ensure that the reference is valid for the duration of the
    // function call.

    // `Box` is what is known as a *smart pointer*. `Box` is used to allocate
    // memory on the heap. It is called *smart* because it is a pointer that
    // knows the size of the data it is pointing to, and it also knows how to
    // clean up the memory when it goes out of scope. When it goes out of scope
    // it will first drop the data it points to (`<T>`), and then drop itself.

    // `Rc` is a reference-counted smart pointer. It keeps track of the number of
    // references to a value and only cleans up the value when the last reference
    // is dropped. Choose `Rc` when you want to have multiple owners of the same
    // data in a single-threaded context.

    // `Arc` is an atomic reference-counted smart pointer. It is the atomic
    // version of `Rc`, which is safe to use in concurrent contexts. Choose `Arc`
    // when you want to have multiple owners of the same data in a multi-threaded
    // context.

    // TODO: Refcell

    // TODO: Mutex

    // TODO: Raw pointers
    println!("Pointers");
}

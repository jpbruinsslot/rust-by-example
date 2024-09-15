// Title: Pointers
// Language: Rust
//
// Rust has two types of pointers that are used to used to manage memory and
// ownership. These are: *non-owning pointers*
// (references: `&`, `&mut`; raw-pointers: `*const`, `*mut`). and
// *owning pointers* (`Box`, `Rc`, `Arc`). *Non-owning* pointers are used to 
// borrow values without taking ownership of them. They do not manage the memory
// they point to, they simply provide access to the data, without affecting its
// ownership. *Owning pointers* allocate memory on the heap and are responsible
// for cleaning up the memory when they go out of scope.

use std::cell::RefCell;
use std::mem::drop;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

#[allow(unused_variables, dead_code)]
fn main() {
    let mut x = 5;

    // `&` is used to create a *immutable* reference to the value of `x`. This
    // allows us to borrow `x` without taking ownership of it. We can use `r1`
    // to read the value of `x`, but we cannot modify it. You can have multiple
    // immutable references to a value at the same time.
    let immut_ref = &x;

    // `ref` is used to create a reference to a value. It is often used in
    // pattern matching to bind a reference to a value. It is similar to `&` but
    // is used in a different context.
    let ref immut_ref2 = x;

    // It's particularly useful when working with enums, like `Option` and
    // `Result`, or custom enums.
    let some_option = Some(5);
    match some_option {
        Some(ref x) => println!("Got a value: {}", x),
        None => (),
    }

    // `&mut` is used to create a *mutable* reference to value of `x`. This
    // allows us to borrow `x` mutably, and modify it. We can only have one
    // mutable reference to a value at a time.
    let mut_ref = &mut x;
    *mut_ref += 1;

    // `ref mut` is used to create a mutable reference to a value. Just like
    // with `ref`, it is similar to `&mut` but used in a different context it
    // is typically used in pattern matching to bind a mutable reference to a
    // value.
    let ref mut mut_ref2 = x;
    *mut_ref2 += 1;

    let mut some_other_option = Some(5);
    match some_other_option {
        Some(ref mut x) => *x += 1,
        None => (),
    }

    // The raw pointer `*const T` is a non-owning pointer that does not implement
    // any automatic cleanup. It is a simple C-style pointer that is used when
    // you need to interact with C code or need to work with unsafe Rust code.
    let raw1 = &x as *const i32;

    // The mutable raw pointer `*mut T`
    let raw2 = &mut x as *mut i32;

    // `Box` is what is known as a *smart pointer*. `Box` is used to allocate
    // memory on the heap. It is called *smart* because it is a pointer that
    // knows the size of the data it is pointing to, and it also knows how to
    // clean up the memory when it goes out of scope. When it goes out of scope
    // it will first drop the data it points to (`<T>`), and then drop itself.
    let b = Box::new(5);

    // `Rc` is a reference-counted smart pointer. It keeps track of the number of
    // references to a value and only cleans up the value when the last reference
    // is dropped. Choose `Rc` when you want to have multiple owners of the same
    // data in single-threaded context.
    let rc1 = Rc::new(5);
    println!(
        "reference count of rc1: {}",
        Rc::strong_count(&rc1),
    );

    {
        // `Rc::clone` is used to create a new reference to the same data. It does
        // not create a deep copy of the data, it simply increments the reference
        // count.
        let rc2 = Rc::clone(&rc1);
        println!(
            "reference count of rc1: {}",
            Rc::strong_count(&rc1),
        );

        // `Rc::clone` can also be called using the `clone` method.
        let rc3 = rc1.clone();
        println!(
            "reference count of rc1: {}",
            Rc::strong_count(&rc1),
        );
    }

    // The reference count of `rc1` is now 1, since `rc2` and `rc3` have gone out
    // of scope.
    println!(
        "reference count of rc1: {}",
        Rc::strong_count(&rc1),
    );

    // `Arc` is an atomic reference-counted smart pointer. It is the atomic
    // version of `Rc`, which is safe to use in concurrent contexts. Choose `Arc`
    // when you want to have multiple owners of the same data in a multi-threaded
    // context.
    let arc1 = Arc::new(5);
    let arc2 = Arc::clone(&arc1);
    let arc3 = arc1.clone();

    let thread1 = thread::spawn(move || {
        println!(
            "reference count of arc in thread1: {}",
            Arc::strong_count(&arc2),
        );
    });

    let thread2 = thread::spawn(move || {
        println!(
            "reference count of arc in thread2: {}",
            Arc::strong_count(&arc3),
        );
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    // At this point, arc2 and arc3 are out of scope, only arc1 remains
    println!(
        "reference count of arc1: {}",
        Arc::strong_count(&arc1),
    );

    // `RefCell` (Reference Cell) is a type that enforces the borrowing rules
    // at runtime instead of compile time. It allows you to mutate data even
    // when there are immutable references to that data. It circumvents
    // borrowing rules by mutating immutable references. This pattern is often
    // referred to as *interior mutability*, and is useful when you need mutable
    // data inside an otherwise immutable value.
    let refcell1: RefCell<i32> = RefCell::new(5);
    *refcell1.borrow_mut() += 1;
    println!("{:?}", *refcell1.borrow());

    // The following pattern is common when you want to manage shared state that
    // needs to be updated by multiple methods of a struct but still want to ensure
    // that the struct itself can be passed around without requiring mutability.
    // We define a struct that contains a `RefCell`, this allows us to modify
    // the `u32` value even if the `Counter` struct is immutable.
    struct Counter {
        count: RefCell<u32>,
    }

    impl Counter {
        fn new() -> Counter {
            Counter {
                count: RefCell::new(0),
            }
        }

        // The `increment` method borrows the `Counter` struct immutably, and then
        // borrows the `count` field mutably.
        fn increment(&self) {
            // You can think of `borrow_mut` as the `&mut` operator, for the `RefCell`.
            let mut count = self.count.borrow_mut();
            *count += 1;
        }

        // The `get` method borrows the `Counter` struct immutably, and then
        // borrows the `count` field immutably.
        fn get(&self) -> u32 {
            // You can think of `borrow` as the `&` operator, for the `RefCell`.
            *self.count.borrow()
        }
    }

    // The `Mutex` type is a mutual exclusion primitive useful for protecting
    // shared data. Mutexes are a way to ensure that only one thread can access
    // shared data at a time. Each thread can lock on a shared value while
    // mutating it until's out of scope, which prevents other threads from
    // mutating it.
    let mutex = Mutex::new(5);

    // Lock the mutex to access the data
    let mut mutex_guard = mutex.lock().unwrap();

    // Dereference the mutex guard to access the data, and then increment the
    // value.
    *mutex_guard += 1;
    println!("{:?}", &mutex_guard);

    // Simulate the variable going out of scope
    drop(mutex_guard);

    // Now we can access the mutex again
    println!("{:?}", mutex);
}

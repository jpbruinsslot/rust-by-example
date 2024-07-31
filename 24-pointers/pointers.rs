// Title: Pointers
// Language: Rust
//
// Rust has two types of pointers that are used to used to manage memory and
// ownership. These are: *owning pointers* (`Box`, `Rc`, `Arc`), and
// *non-owning pointers* (references: `&`, `&mut`; raw-pointers: `*const`, `*mut`).
// Owning pointers allocate memory on the heap and are responsible for cleaning
// up the memory when they go out of scope. Non-owning pointers are used to
// borrow values without taking ownership of them. They do not manage the memory
// they point to, they simply provide access to the data, without affecting its
// ownership.

use std::cell::RefCell;
use std::mem::drop;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    // `&` is used to create a reference to a value, and it allows us to
    // *borrow* the value without taking ownership of it. This is also known as
    // a *shared reference*. As such, the lifetime of the reference depends on
    // its owner and is limited to the scope in which it was created.
    let mut x = 5;

    // `r` is a immutable reference to `x`.
    let r1 = &x;

    // `r` is a mutable reference to `x`.
    let r2 = &mut x;

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
    // data in a single-threaded context.
    let rc1 = Rc::new(5);
    let rc2 = rc.clone();
    let rc3 = Rc::clone(&rc);

    // `Arc` is an atomic reference-counted smart pointer. It is the atomic
    // version of `Rc`, which is safe to use in concurrent contexts. Choose `Arc`
    // when you want to have multiple owners of the same data in a multi-threaded
    // context.
    let arc1 = Arc::new(5);
    let arc2 = arc.clone();
    let arc3 = Arc::clone(&arc);

    // `RefCell` (Reference Cell) is a type that enforces the borrowing rules
    // at runtime instead of compile time. It is useful when you need to mutate
    // data that is behind an immutable reference. It is also useful when you
    // need to have multiple mutable references to the same data. It circumvents
    // borrowing rules by mutating immutable references. This pattern is often
    // referred to as *interior mutability*.
    let refcell1 = RefCell::new(5);

    // `Ref` - immutable borrow
    let refcell2 = refcell1.borrow();

    // `RefMut` - mutable borrow
    let refcell3 = refcell1.borrow_mut();

    // The `Mutex` type is a mutual exclusion primitive useful for protecting
    // shared data. Mutexes are a way to ensure that only one thread can access
    // shared data at a time. Each thread can lock on a shared value while
    // mutating it until's out of scope, which prevents other threads from
    // mutating it.
    let mutex = Mutex::new(5);

    // Lock the mutex to access the data
    let mutex_guard = mutex.lock().unwrap();

    // Dereference the mutex guard to access the data, and then increment the
    // value.
    *mutex_guard += 1;
    println!("{:?}", &mutex_guard);

    // Simulate the variable going out of scope
    drop(mutex_guard);

    // Now we can access the mutex again
    println!("{:?}", mutex);
}

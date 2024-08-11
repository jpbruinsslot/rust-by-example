# Language: shell
$ rustc pointers.rs
$ ./pointers
Got a value: 5
reference count of rc1: 1
reference count of rc1: 2
reference count of rc1: 3
reference count of rc1: 1
reference count of arc in thread2: 3
reference count of arc in thread1: 2
reference count of arc1: 1
6
6
Mutex { data: 6, poisoned: false, .. }

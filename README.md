# Linked Lists in Rust 
Following the "Learn Rust With Entirely Too Many Linked Lists" book at [https://rust-unofficial.github.io/too-many-lists/](https://rust-unofficial.github.io/too-many-lists/)

1. [badstack.rs](src/badstack.rs), very basic and rudimentary implemenation of a stack using a single linked list
2. [betterstack.rs](src/betterstack.rs), modified version of list 1 with optimization using language features, iteration and generics
3. [persistentstack.rs](src/persistentstack.rs), a persistent immutable singly-linked list with atomic reference counting for memory management
4. [doublylinked.rs](src/doublylinked.rs), a doubly-linked list using interior mutability and refcells
5. [unsafequeue.rs](src/unsafequeue.rs), a queue via a singly linked list but programmed using unsafe rust and raw pointers. 
6. [doubledeque.rs](src/doubledeque.rs), a deque implemented using a doubly-linked list, with unsafe rust. Production-quality, equivalent to ```std::collections::LinkedList``` with methods. Includes cursor and concurrency-related traits. 
// a list is either empty or has an element followed by another list

use std::mem;

pub struct List { // only keep structure public / accessible
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List {
            head: Link::Empty
        }
    }

    pub fn push(&mut self, elem: i32) {
        let newnode = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty) // replace to avoid borrowing issues
        });

        self.head = Link::More(newnode);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}
enum Link {
    Empty,
    More(Box<Node>) // use box because recursive data structure and need heap allocation
}

struct Node {
    elem : i32,
    next: Link
}


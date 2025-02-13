// a list is either empty or has an element followed by another list

pub struct List { // only keep structure public / accessible
    head: Link,
}
enum Link {
    Empty,
    More(Box<Node>) // use box because recursive data structure and need heap allocation
}

struct Node {
    elem : i32,
    next: Link
}


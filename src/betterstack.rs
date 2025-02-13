pub struct List { // only keep structure public / accessible
    head: Link,
}
type Link = Option<Box<Node>>; // use option generics
struct Node {
    elem : i32,
    next: Link
}



impl List {
    pub fn new() -> Self {
        List { head : None }
    }

    pub fn push(&mut self, elem: i32) {
        let newnode = Box::new(Node {
            elem,
            next: self.head.take()
        });

        self.head = Some(newnode);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link { // until node is empty
            cur_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)] // indicate to compiler to only compile module when testing
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}


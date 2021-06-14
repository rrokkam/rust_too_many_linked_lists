pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for List {
    // Doing it this way (as opposed to while-let with pop()) avoids manipulating
    // the elem values, so they can be dropped in-place.
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take()
        }
    }
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_layout_optimization() {
        let ptr_size = std::mem::size_of::<Box<Node>>();
        let link_size = std::mem::size_of::<Link>();
        assert_eq!(link_size, ptr_size);
    }

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

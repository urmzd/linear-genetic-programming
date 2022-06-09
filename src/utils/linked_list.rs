use std::{mem, ptr::NonNull};

struct LinkedList<T> {
    head: Option<Pointer<T>>,
    tail: Option<Pointer<T>>,
    length: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct Node<T> {
    data: T,
    next: Option<Pointer<T>>,
    length: usize,
}

type Pointer<T> = NonNull<Node<T>>;

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(data: T) -> Self {
        Node {
            data,
            next: None,
            length: 1,
        }
    }

    fn new_dyn(data: T) -> Box<Node<T>> {
        Box::new(Self::new(data))
    }

    fn as_ptr(self: Box<Node<T>>) -> Option<Pointer<T>> {
        unsafe {
            let static_node = Box::leak(self).into();
            let some_static_node = Some(static_node);
            some_static_node
        }
    }

    // What should this return?
    // The unwrapped node
    fn point_next_to(&mut self, mut node: Option<Pointer<T>>) -> &Node<T> {
        unsafe {
            match node {
                None => panic!("Ensure you're using as_ptr to construct the raw pointer."),
                Some(ref mut inner_node) => {
                    self.next = Some(*inner_node);
                    self.length += mem::replace(&mut inner_node.as_mut().length, 1);
                    inner_node.as_ref()
                }
            }
        }
    }

    fn next(&self) -> Option<&Node<T>> {
        unsafe {
            if let Some(inside) = &self.next {
                return Some(inside.as_ref());
            }
        }

        return None;
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

// Why are we using this data structure?
//
// Chromosomes A: a b c d e f g h
// Chromosomes B: z y x w v u t s
//
// We want to swap <b,.., e> with <z,..x>
// make a -> z -> f
// make e -> w

impl<T> LinkedList<T>
where
    T: PartialEq,
{
    pub fn new() -> Self {
        LinkedList {
            length: 0,
            head: None,
            tail: None,
        }
    }

    /// Adds a new element to the end of the linked list.
    ///
    /// Cases:
    ///     Case 1: Empty LinkedList
    ///     Case 2: Head Only LinkedList
    ///     Case 3: Full LinkedList
    pub fn append(&mut self, data: T) {
        unsafe {
            let node = Node::new_dyn(data);
            let some_leaked_node = node.as_ptr();
            match self.head {
                // Case 1:
                None => {
                    self.head = some_leaked_node;
                }
                Some(head_ptr) => {
                    match self.tail {
                        // Case 2:
                        None => {
                            (*head_ptr.as_ptr()).point_next_to(some_leaked_node);
                        }

                        // Case 3:
                        Some(tail_ptr) => {
                            (*tail_ptr.as_ptr()).point_next_to(some_leaked_node);
                        }
                    }
                    self.tail = some_leaked_node;
                }
            }

            self.length += 1;
        }
    }

    pub fn pop_head(&mut self) -> Option<Box<Node<T>>> {
        self.head.map(|node| unsafe {
            let contained_node = Box::from_raw(node.as_ptr());

            // DEBUG: why is this none?
            self.head = contained_node.next;

            if self.head.is_none() {
                self.tail = None
            }

            contained_node
        })
    }

    pub fn head(&mut self) -> Option<&Node<T>> {
        unsafe { self.head.map(|node| node.as_ref()) }
    }

    pub fn tail(&mut self) -> Option<&Node<T>> {
        unsafe { self.tail.map(|node| node.as_ref()) }
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

#[cfg(test)]
mod test {
    use std::ptr;

    use super::{LinkedList, Node};

    #[test]
    fn given_a_list_of_elems_when_appended_to_linked_list_then_linked_list_contains_item() {
        let mut linked_list = LinkedList::new();
        linked_list.append(1);
        linked_list.append(2);
        linked_list.append(3);

        assert_eq!(linked_list.len(), 3);

        assert_eq!(linked_list.pop_head().map(|node| node.data), Some(1));
        assert_eq!(linked_list.pop_head().map(|node| node.data), Some(2));
        assert_eq!(linked_list.pop_head().map(|node| node.data), Some(3));

        /*
         *        let items = [1, 2, 3, 4, 5];
         *        let linked_list = LinkedList::new();
         *
         *        for item in items {
         *            linked_list.append(item)
         *        }
         *
         *        assert_eq!(linked_list.len(), items.len());
         *
         *        let head = linked_list.head();
         */
    }

    #[test]
    fn given_nodes_when_point_next_to_called_node_1_points_to_node_2() {
        let mut first_node = Node::new_dyn(1);
        let second_node = Node::new_dyn(2);

        let second_node_ptr = first_node.point_next_to(second_node.as_ptr());

        assert!(ptr::eq(second_node_ptr, first_node.next().unwrap()));
        assert_eq!(first_node.len(), 2);
    }
}

// Why are we using this data structure?
//
// Chromosomes A: a b c d e f g h
// Chromosomes B: z y x w v u t s
//
// We want to swap <b,.., e> with <z,..x>
// make a -> z -> f
// make e -> w
use std::{fmt, marker::PhantomData, ptr::NonNull};

struct LinkedList<T> {
    head: Option<Pointer<T>>,
    tail: Option<Pointer<T>>,
    length: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node<T> {
    data: T,
    next: Option<Pointer<T>>,
}

struct Iter<'a, T> {
    next: Option<Pointer<T>>,
    length: usize,
    _marker: PhantomData<&'a T>,
}

struct IterMut<'a, T> {
    next: Option<Pointer<T>>,
    length: usize,
    _marker: PhantomData<&'a mut T>,
}

pub struct IntoIter<T>(LinkedList<T>);

struct CursorMut<'a, T> {
    list: &'a mut LinkedList<T>,
    current: Option<Pointer<T>>,
    index: Option<usize>,
}

impl<'a, T> CursorMut<'a, T> {
    pub fn current(&mut self) -> Option<&mut T> {
        self.current.map(|node| unsafe {
            let element = &mut (*node.as_ptr());
            &mut element.data
        })
    }

    pub fn next(&mut self) {
        // We're somewhere in the "middle"
        if let Some(node) = self.current {
            self.current = unsafe { (*node.as_ptr()).next };
            self.index.map(|idx| idx + 1);
        } else {
            // We've reached the end, loop to ghost front
            if self.index > Some(self.list.length) {
                self.current = None;
                self.index = None;
            } else {
                // we're at the front, go to head
                self.current = self.list.head;
                // If head is empty, index is still 0
                match self.current {
                    Some(_) => self.index = Some(0),
                    None => return,
                }
            }
        }
    }

    fn split_before(&mut self) -> LinkedList<T> {
        // go from head -> self.element

        todo!("")
    }

    fn split_after(&mut self) -> LinkedList<T> {
        todo!("")
    }
}

type Pointer<T> = NonNull<Node<T>>;

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node { data, next: None }
    }

    fn new_dyn(data: T) -> Box<Node<T>> {
        Box::new(Self::new(data))
    }

    fn as_ptr(self: Box<Node<T>>) -> Pointer<T> {
        Box::leak(self).into()
    }

    fn point_to(&mut self, node: Pointer<T>) {
        self.next = Some(node);
    }

    fn next(&self) -> Option<&Node<T>> {
        unsafe { self.next.map(|node| node.as_ref()) }
    }

    fn next_mut(&mut self) -> Option<&mut Node<T>> {
        unsafe { self.next.map(|mut node| node.as_mut()) }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            length: 0,
            head: None,
            tail: None,
        }
    }

    pub fn clear(&mut self) {
        while self.head.is_some() {
            self.dequeue();
        }
    }

    pub fn append(&mut self, data: T) {
        unsafe {
            let node = Node::new_dyn(data);
            let some_leaked_node = node.as_ptr();
            match self.head {
                None => {
                    self.head = Some(some_leaked_node);
                }
                Some(head_ptr) => {
                    match self.tail {
                        None => {
                            (*head_ptr.as_ptr()).point_to(some_leaked_node);
                        }
                        Some(tail_ptr) => {
                            (*tail_ptr.as_ptr()).point_to(some_leaked_node);
                        }
                    }
                    self.tail = Some(some_leaked_node);
                }
            }

            self.length += 1;
        }
    }

    pub fn dequeue(&mut self) -> Option<Box<Node<T>>> {
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

    pub fn head(&self) -> Option<&Node<T>> {
        unsafe { self.head.map(|node| node.as_ref()) }
    }

    pub fn tail(&self) -> Option<&Node<T>> {
        unsafe { self.tail.map(|node| node.as_ref()) }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head,
            length: self.length,
            _marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head,
            length: self.length,
            _marker: PhantomData,
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn cursor_mut(&mut self) -> CursorMut<T> {
        CursorMut {
            list: self,
            current: None,
            index: None,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

// Reference Iterator
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| unsafe {
            self.next = (*node.as_ptr()).next;
            &(*node.as_ptr()).data
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.length, Some(self.length))
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.length
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// Mutable Iterator
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| unsafe {
            self.next = (*node.as_ptr()).next;
            &mut (*node.as_ptr()).data
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.length, Some(self.length))
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        self.length
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;

    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

// Owned
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T> Default for LinkedList<T>
where
    T: PartialEq,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for LinkedList<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        let mut cloned_list = Self::new();
        cloned_list.extend(self.iter().cloned());
        cloned_list
    }
}

impl<E> Extend<E> for LinkedList<E> {
    fn extend<T: IntoIterator<Item = E>>(&mut self, iter: T) {
        for element in iter {
            self.append(element)
        }
    }
}

impl<'a, E> FromIterator<E> for LinkedList<E> {
    fn from_iter<T: IntoIterator<Item = E>>(iter: T) -> Self {
        let mut list = Self::new();
        list.extend(iter);
        list
    }
}

impl<'a, E> IntoIterator for LinkedList<E> {
    type Item = E;

    type IntoIter = IntoIter<E>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<E> Iterator for IntoIter<E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.dequeue().map(|node| node.data)
    }
}

impl<E> ExactSizeIterator for IntoIter<E> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<E> fmt::Debug for LinkedList<E>
where
    E: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LinkedList")
            .field("head", &self.head)
            .field("tail", &self.tail)
            .field("length", &self.length)
            .finish()
    }
}

impl<E> PartialEq for LinkedList<E>
where
    E: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other)
    }

    fn ne(&self, other: &Self) -> bool {
        self.len() != other.len() && self.iter().ne(other)
    }
}

impl<E> Eq for LinkedList<E> where E: PartialEq {}

impl<E> PartialOrd for LinkedList<E>
where
    E: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<E> Ord for LinkedList<E>
where
    E: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.iter().cmp(other)
    }
}

#[cfg(test)]
mod tests {
    use super::{LinkedList, Node};

    #[test]
    fn given_a_list_of_elems_when_extended_then_linked_list_is_fill_with_elements() {
        let elements = [1, 2, 3, 4, 5];
        let mut linked_list = LinkedList::new();

        assert_eq!(linked_list.len(), 0);
        linked_list.extend(elements);
        assert_eq!(linked_list.len(), elements.len());

        // test iter

        let mut index = 0;
        for element in linked_list {
            assert_eq!(element, elements[index]);
            index += 1
        }
    }

    #[test]
    fn given_a_list_of_elems_when_appended_to_linked_list_then_linked_list_contains_item() {
        let mut linked_list = LinkedList::new();
        linked_list.append(1);
        linked_list.append(2);
        linked_list.append(3);

        assert_eq!(linked_list.len(), 3);

        assert_eq!(linked_list.dequeue().map(|node| node.data), Some(1));
        assert_eq!(linked_list.dequeue().map(|node| node.data), Some(2));
        assert_eq!(linked_list.dequeue().map(|node| node.data), Some(3));
    }

    #[test]
    fn given_two_nodes_when_point_to_is_called_then_node_one_points_to_node_two() {
        let mut first_node = Node::new_dyn(1);
        let second_node = Node::new_dyn(2);

        first_node.point_to(second_node.as_ptr());

        assert_eq!(first_node.next().map(|node| node.data), Some(2));
        assert_eq!(first_node.next().and_then(|node| node.next()), None)
    }

    #[test]
    fn given_linked_list_when_accessors_called_then_nodes_are_returned() {
        let elems = [1, 2, 3, 4];
        let mut linked_list = LinkedList::new();
        linked_list.extend(elems);

        assert_eq!(linked_list.head().map(|node| node.data), Some(1));
        assert_eq!(linked_list.tail().map(|node| node.data), Some(4));
    }

    #[test]
    fn given_linked_list_cursor_when_next_is_called_then_nodes_are_cycled() {
        let elems = [1, 2, 3, 4];
        let mut linked_list_one = LinkedList::new();
        linked_list_one.extend(elems);
        let mut cursor_one = linked_list_one.cursor_mut();
        assert_eq!(cursor_one.current(), None);
        cursor_one.next();
        assert_eq!(cursor_one.current(), Some(&mut 1));
        cursor_one.next();
        assert_eq!(cursor_one.current(), Some(&mut 2));
        cursor_one.next();
        assert_eq!(cursor_one.current(), Some(&mut 3));
        cursor_one.next();
        assert_eq!(cursor_one.current(), Some(&mut 4));
        cursor_one.next();
        assert_eq!(cursor_one.current(), None);
        cursor_one.next();
        assert_eq!(cursor_one.current(), Some(&mut 1));
    }
}

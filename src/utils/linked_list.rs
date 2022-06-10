// Why are we using this data structure?
//
// Chromosomes A: a b c d e f g h
// Chromosomes B: z y x w v u t s
//
// We want to swap <b,.., e> with <z,..x>
// make a -> z -> f
// make e -> w
use std::{fmt, marker::PhantomData, mem, ptr::NonNull};

use log::debug;
use more_asserts::assert_lt;

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
            self.index = self.index.map(|idx| idx + 1);
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

    // TODO: Determine how to handle looping when seeking?
    // If we go all the way around multiple times, performance would suffer.
    // We might want to reset and use modulo?
    pub fn seek(&mut self, idx: usize) -> bool {
        if idx >= self.list.len() {
            debug!(
                "Index out of range. Expected index provided to be between 0 and {}",
                self.list.len()
            );
            false
        } else {
            while self.index < Some(idx) {
                self.next();
            }
            true
        }
    }

    pub fn seek_before(&mut self, idx: usize) -> bool {
        // We need to reset
        if idx == 0 {
            self.reset();
            true
        } else {
            self.seek(idx)
        }
    }

    pub fn seek_after(&mut self, idx: usize) -> bool {
        self.seek(idx + 1)
    }

    fn split_after(&mut self) -> LinkedList<T> {
        // We're somewhere between the head and the tail
        if let Some(current) = self.current {
            let n_nodes_used = self.index.unwrap() + 1;
            let new_linked_list = LinkedList {
                head: unsafe { (*current.as_ptr()).next },
                tail: self.list.tail,
                length: self.list.length - n_nodes_used,
            };

            unsafe {
                // Break the list
                // Before: a -> b -> c -> d -> e (c -> current)
                // After: a -> b > c && d -> e
                (*current.as_ptr()).remove_next();
                assert_eq!((*current.as_ptr()).next, None);
                self.list.length = n_nodes_used;
                // We become the new tail.
                self.list.tail = self.current;
            }

            new_linked_list
        } else {
            // We're at the spot before the the head
            mem::replace(self.list, LinkedList::new())
        }
    }

    fn reset(&mut self) {
        self.current = None;
        self.index = None
    }

    fn swap(
        &mut self,
        other: &mut CursorMut<'a, T>,
        start_idx: usize,
        other_start_idx: usize,
        end_idx: Option<usize>,
        other_end_idx: Option<usize>,
    ) {
        // We don't want to deal with edge-cases at the moment.
        // TODO: deal with edge cases.
        // Let X => a -> b -> c -> d -> e
        // Let Y => 1 -> 2 - > 3 -> 4 -> 5
        // We want to point 2 -> c, d -> 4, b -> 3, 4 -> e

        assert_ne!(self.list.len(), 0);
        assert_ne!(other.list.len(), 0);
        assert_lt!(start_idx, self.list.len());
        assert_lt!(other_start_idx, other.list.len());
        assert_lt!(end_idx, Some(self.list.len()));
        assert_lt!(other_end_idx, Some(self.list.len()));
        assert_eq!(end_idx.is_none(), other_end_idx.is_none());

        let no_end = end_idx.is_none();

        // Start at the beginning;
        // TODO: optimize by finding quickest path to start_idx, and if end_idx is used, grab a reference to the pointer.
        self.reset();
        other.reset();

        self.seek_before(start_idx);
        other.seek_before(other_start_idx);

        let self_before = self.current.unwrap();
        let other_before = other.current.unwrap();

        if no_end {
            unsafe {
                let self_before_next = (*self_before.as_ptr()).next();
                let other_before_next = (*other_before.as_ptr()).next();
            }
        }

        // self.seek_before(end_idx);
        // other.seek_before(other_end_idx);

        // let self_after = self.current;
        // let other_after = self.
    }
}

type Pointer<T> = NonNull<Node<T>>;
type BoundedPointer<T> = Box<Node<T>>;

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

    // Ensure you clear the allocated space once done.
    fn point_to(&mut self, node: Option<Pointer<T>>) -> Option<Pointer<T>> {
        let current_next = self.next;
        self.next = node;
        current_next
    }

    fn remove_next(&mut self) -> Option<Pointer<T>> {
        self.point_to(None)
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

    pub fn is_empty(&self) -> bool {
        self.len() == 0
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
                            (*head_ptr.as_ptr()).point_to(Some(some_leaked_node));
                        }
                        Some(tail_ptr) => {
                            // Debug: Double free -- be careful
                            (*tail_ptr.as_ptr()).point_to(Some(some_leaked_node));
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

        let mut current = linked_list.head();
        assert_eq!(current.map(|node| node.data), Some(elements[0]));
        current = current.and_then(|node| node.next());
        assert_eq!(current.map(|node| node.data), Some(elements[1]));
        current = current.and_then(|node| node.next());
        assert_eq!(current.map(|node| node.data), Some(elements[2]));
        current = current.and_then(|node| node.next());
        assert_eq!(current.map(|node| node.data), Some(elements[3]));
        current = current.and_then(|node| node.next());
        assert_eq!(current.map(|node| node.data), Some(elements[4]));
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

        let previous_next = first_node.point_to(Some(second_node.as_ptr()));
        assert_eq!(previous_next, None);

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

        let mut list = LinkedList::new();
        list.extend(elems);

        let mut cursor = list.cursor_mut();

        assert_eq!(cursor.current(), None);
        cursor.next();
        assert_eq!(cursor.current(), Some(&mut 1));
        cursor.next();
        assert_eq!(cursor.current(), Some(&mut 2));
        cursor.next();
        assert_eq!(cursor.current(), Some(&mut 3));
        cursor.next();
        assert_eq!(cursor.current(), Some(&mut 4));
        cursor.next();
        assert_eq!(cursor.current(), None);
        cursor.next();
        assert_eq!(cursor.current(), Some(&mut 1));

        let mut null_list = LinkedList::<i32>::new();
        let mut cursor_null = null_list.cursor_mut();

        assert_eq!(cursor_null.current(), None);
        cursor_null.next();
        assert_eq!(cursor_null.current(), None);
    }

    #[test]
    fn given_linked_lists_when_split_after_is_called_then_a_new_list_is_returned() {
        let elems = [1, 2, 3, 4, 5];
        let mut list = LinkedList::new();
        list.extend(elems);

        let mut cursor = list.cursor_mut();

        cursor.next();

        assert_eq!(cursor.current(), Some(&mut 1));

        let mut split_list = cursor.split_after();

        cursor.next();

        assert_eq!(cursor.current(), None);

        cursor.next();

        assert_eq!(cursor.current(), Some(&mut 1));

        let mut split_cursor = split_list.cursor_mut();

        split_cursor.next();

        assert_eq!(split_cursor.current(), Some(&mut 2));
    }

    #[test]
    fn given_linked_list_cursor_when_seek_then_element_at_index_is_reached() {
        let elems = [1, 2, 3, 4, 5];
        let mut list = LinkedList::new();
        list.extend(elems);

        let mut cursor = list.cursor_mut();
        assert_eq!(cursor.seek(3), true);
        assert_eq!(cursor.current(), Some(&mut 4));
        // We stay in the same place;
        assert_eq!(cursor.seek(55), false);
        assert_eq!(cursor.current(), Some(&mut 4));
    }
}

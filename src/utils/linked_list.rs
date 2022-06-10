use std::{marker::PhantomData, ptr::NonNull};

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

type Pointer<T> = NonNull<Node<T>>;

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node { data, next: None }
    }

    fn new_dyn(data: T) -> Box<Node<T>> {
        Box::new(Self::new(data))
    }

    fn as_ptr(self: Box<Node<T>>) -> Pointer<T> {
        unsafe { Box::leak(self).into() }
    }

    fn point_to(&mut self, mut node: Pointer<T>) {
        self.next = Some(node);
    }

    fn next(&self) -> Option<&Node<T>> {
        unsafe { self.next.map(|node| node.as_ref()) }
    }

    fn next_mut(&mut self) -> Option<&mut Node<T>> {
        unsafe { self.next.map(|mut node| node.as_mut()) }
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

// Reference Iterator
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.and_then(|node| unsafe {
            self.next = (*node.as_ptr()).next;
            Some(node.as_ref())
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
    type Item = &'a Node<T>;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// Mutable Iterator
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.and_then(|mut node| unsafe {
            self.next = (*node.as_ptr()).next;
            Some(node.as_mut())
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
    type Item = &'a mut Node<T>;

    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

// Base

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
        cloned_list.extend(self.iter().map(|node| node.data.clone()));
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

    pub fn head(&mut self) -> Option<&Node<T>> {
        unsafe { self.head.map(|node| node.as_ref()) }
    }

    pub fn tail(&mut self) -> Option<&Node<T>> {
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

    pub fn len(&self) -> usize {
        self.length
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
}

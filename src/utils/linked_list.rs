// Single-linked list.
struct LinkedList<'a, T> {
    length: usize,
    head: Node<'a, T>,
    tail: Node<'a, T>,
}

#[derive(Debug, PartialEq, Eq)]
enum Node<'a, T> {
    Cons(T, &'a Node<'a, T>),
    Nil,
}

impl<'a, T> Node<'a, T> {
    pub fn new() -> Self {
        Node::Nil
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

impl<'a, T> LinkedList<'a, T>
where
    T: PartialEq,
{
    pub fn new() -> Self {
        LinkedList {
            length: 0,
            head: Node::Nil,
            tail: Node::Nil,
        }
    }

    /// Adds a new element to the end of the linked list.
    pub fn append(&'a mut self, data: T) {
        // Current List: (Nil, Nil)
        if self.head == Node::Nil {
            let new_head = Node::Cons(data, &mut self.tail);
            self.head = new_head
            // After: (Cons T &Nil)
        } else {
            // Current List: (... Cons(... Cons(T, Nil)))
        }
        /*
         *if self.head == List::Nil {
         *    let new_head = List::Cons(data, &List::Nil);
         *    self.head = new_head
         *} else {
         *    self.tail.append(data)
         *}
         */
        todo!("fix")
    }

    pub fn extract_between(&mut self, start_index: usize, end_index: usize) -> Self {
        todo!()
    }

    pub fn head(&mut self) -> &mut Self {
        todo!()
    }

    pub fn tail(&mut self) -> &mut Self {
        todo!("")
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

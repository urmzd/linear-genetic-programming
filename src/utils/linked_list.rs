// Double LinkedList
enum LinkedList<'a, T> {
    Node {
        data: T,
        next: &'a LinkedList<'a, T>,
    },
    Tail,
}

// Why are we using this data structure?
//
// Chromosomes A: a b c d e f g h
// Chromosomes B: z y x w v u t s
//
// We want to swap <b,.., e> with <z,..x>
// make a -> z -> f
// make e -> w

impl<'a, T> LinkedList<'a, T> {
    pub fn new() -> Self {}

    pub fn append(&mut self, data: Self) {}

    pub fn extract_between(&mut self, start_index: usize, end_index: usize) -> Self {
        todo!()
    }

    pub fn head(&mut self) -> &mut Self {
        todo!()
    }

    pub fn tail(&mut self) -> &mut Self {
        todo!("")
    }
}

use rand::Rng;

use crate::utils::{linked_list::LinkedList, random::generator};

use super::{characteristics::Breed, instruction::Instruction};

impl<'a> Breed for Instructions<'a> {
    fn two_point_crossover(&self, mate: &Self) -> [Self; 2] {
        let mut instructions_a = self.clone();
        let mut instructions_b = mate.clone();

        let a_start = generator().gen_range(0..instructions_a.len() - 1);
        let a_end = if a_start == instructions_a.len() {
            None
        } else {
            Some(generator().gen_range(a_start..=instructions_a.len())).and_then(|index| {
                if index == instructions_a.len() || a_start == index {
                    None
                } else {
                    Some(index)
                }
            })
        };

        let b_start = generator().gen_range(0..instructions_b.len() - 1);
        let b_end = if b_start == instructions_b.len() {
            None
        } else {
            Some(generator().gen_range(b_start..=instructions_b.len())).and_then(|index| {
                if index == instructions_b.len() || b_start == index {
                    None
                } else {
                    Some(index)
                }
            })
        };

        let mut cursor_a = instructions_a.cursor_mut();
        let mut cursor_b = instructions_b.cursor_mut();

        cursor_a.swap(&mut cursor_b, a_start, b_start, a_end, b_end);

        [instructions_a, instructions_b]
    }
}

pub type Instructions<'a> = LinkedList<Instruction<'a>>;

use rand::{distributions::Uniform, prelude::Distribution};

use crate::utils::{linked_list::LinkedList, random::generator};

use super::{characteristics::Breed, inputs::ValidInput, instruction::Instruction};

impl<'a, T> Breed for Instructions<'a, T>
where
    T: ValidInput,
{
    fn two_point_crossover(&self, mate: &Self) -> [Self; 2] {
        let mut instructions_a = self.clone();
        let mut instructions_b = mate.clone();
        let current_generator = &mut generator();

        let a_start = Uniform::new(0, instructions_a.len()).sample(current_generator);
        let a_end = if a_start == instructions_a.len() - 1 {
            None
        } else {
            let tmp_end = Uniform::new(a_start + 1, instructions_a.len()).sample(current_generator);

            Some(tmp_end)
        };

        let b_start = Uniform::new(0, instructions_b.len()).sample(current_generator);
        let b_end = if b_start == instructions_b.len() - 1 {
            None
        } else {
            let tmp_end = Uniform::new(b_start + 1, instructions_b.len()).sample(current_generator);
            Some(tmp_end)
        };

        let mut cursor_a = instructions_a.cursor_mut();
        let mut cursor_b = instructions_b.cursor_mut();

        cursor_a.swap(&mut cursor_b, a_start, b_start, a_end, b_end);

        [instructions_a, instructions_b]
    }
}

pub type Instructions<'a, T> = LinkedList<Instruction<'a, T>>;

use derive_new::new;
use serde::Serialize;

use crate::{
    core::{
        characteristics::{Fitness, FitnessScore, Organism},
        program::{ExtensionParameters, Program},
    },
    measure::{accuracy::Accuracy, definitions::Metric},
    utils::common_traits::{Compare, Inputs, Show, ValidInput},
};

#[derive(Clone, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord, new)]
pub struct ClassificationParameters<'a, InputType>
where
    InputType: ClassificationInput,
{
    inputs: &'a Inputs<InputType>,
}

impl<'a, T> ExtensionParameters for ClassificationParameters<'a, T>
where
    T: ClassificationInput,
{
    type InputType = T;
}

pub trait ClassificationInput: ValidInput {
    const N_INPUTS: usize;
    fn get_class(&self) -> Self::Actions;
}

impl<'a, T> Fitness for Program<'a, ClassificationParameters<'a, T>>
where
    T: ClassificationInput,
{
    fn eval_fitness(&self) -> FitnessScore {
        let inputs = self.other.inputs;

        let mut fitness: Accuracy<Option<T::Actions>> = Accuracy::new();

        for input in inputs {
            let mut registers = self.registers.clone();

            for instruction in &self.instructions {
                instruction.apply(&mut registers, input);
            }

            let ties = registers.argmax();
            let predicted_class = T::argmax(ties);
            let correct_class = input.get_class();

            fitness.observe([predicted_class, Some(correct_class)]);

            registers.reset();
        }

        fitness.calculate()
    }

    fn eval_set_fitness(&mut self) -> FitnessScore {
        *self.fitness.get_or_insert(self.eval_fitness())
    }

    fn get_fitness(&self) -> Option<FitnessScore> {
        self.fitness
    }
}

impl<'a, T> Organism<'a> for Program<'a, ClassificationParameters<'a, T>> where
    T: ClassificationInput
{
}
impl<'a, T> Show for ClassificationParameters<'a, T> where T: ClassificationInput {}
impl<'a, T> Compare for ClassificationParameters<'a, T> where T: ClassificationInput {}
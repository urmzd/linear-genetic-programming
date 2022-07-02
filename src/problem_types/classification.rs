use derive_new::new;
use num::FromPrimitive;
use serde::Serialize;

use crate::{
    genes::{
        characteristics::{Fitness, FitnessScore, Organism},
        program::Program,
    },
    metrics::{accuracy::Accuracy, definitions::Metric},
    utils::common_traits::{Compare, Inputs, Show, ValidInput},
};

#[derive(Clone, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord, new)]
pub struct Classification<'a, InputType>
where
    InputType: ClassificationInput,
{
    inputs: &'a Inputs<InputType>,
}

pub trait ClassificationInput: ValidInput {
    fn get_class(&self) -> Self::Represent;
}

impl<'a, InputType> Fitness for Program<'a, Classification<'a, InputType>>
where
    InputType: ClassificationInput,
{
    fn eval_fitness(&self) -> FitnessScore {
        let inputs = self.other.inputs;

        let mut fitness: Accuracy<Option<InputType::Represent>> = Accuracy::new();

        for input in inputs {
            let mut registers = self.registers.clone();

            for instruction in &self.instructions {
                instruction.apply(&mut registers, input);
            }

            let argmax = registers.argmax::<InputType, _>(|mut ties| {
                if ties.len() > 1 {
                    None
                } else {
                    let classification: Option<InputType::Represent> =
                        FromPrimitive::from_usize(ties.pop().unwrap());
                    classification
                }
            });
            let correct_class = input.get_class();

            fitness.observe([argmax, Some(correct_class)]);

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

impl<'a, T> Organism<'a> for Program<'a, Classification<'a, T>> where T: ClassificationInput {}
impl<'a, T> Show for Classification<'a, T> where T: ClassificationInput {}
impl<'a, T> Compare for Classification<'a, T> where T: ClassificationInput {}

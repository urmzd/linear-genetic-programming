use derive_new::new;
use num::ToPrimitive;
use ordered_float::OrderedFloat;
use serde::Serialize;
use smartcore::metrics::accuracy::Accuracy;

use crate::{
    core::{
        characteristics::{Fitness, FitnessScore, Organism},
        program::{ExtensionParameters, Program},
        registers::Registers,
    },
    utils::common_traits::{Compare, Inputs, Show, ValidInput},
};

use std::hash::Hash;

#[derive(Clone, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord, new)]
pub struct ClassificationParameters<'a, InputType>
where
    InputType: ClassificationInput,
    <InputType as ValidInput>::Actions: Hash + Compare,
{
    inputs: &'a Inputs<InputType>,
}

impl<'a, T> ExtensionParameters for ClassificationParameters<'a, T>
where
    T: ClassificationInput,
    <T as ValidInput>::Actions: Hash + Compare,
{
    type InputType = T;
}

pub trait ClassificationInput: ValidInput + Into<Registers> + Compare
where
    Self::Actions: Compare + Hash,
{
    fn get_class(&self) -> Self::Actions;
}

impl<'a, T> Fitness for Program<'a, ClassificationParameters<'a, T>>
where
    T: ClassificationInput,
    <T as ValidInput>::Actions: Hash + Compare,
{
    fn eval_fitness(&self) -> FitnessScore {
        let inputs = self.other.inputs;

        let mut scores = vec![];

        for input in inputs {
            let mut registers = self.registers.clone();

            for instruction in &self.instructions {
                instruction.apply(&mut registers, input);
            }

            let ties = registers.argmax();
            let predicted_class = T::argmax(ties)
                .map(|action| action.to_i32())
                .unwrap()
                .unwrap_or(-1);
            let correct_class = input.get_class().to_i32().unwrap();

            scores.push((predicted_class, correct_class));

            registers.reset();
        }

        let predicted: Vec<f32> = scores.iter().map(|score| score.0 as f32).collect();
        let correct: Vec<f32> = scores.iter().map(|score| score.1 as f32).collect();

        let fitness = (Accuracy {}).get_score(&predicted, &correct);

        OrderedFloat(fitness)
    }

    fn eval_set_fitness(&mut self) -> FitnessScore {
        *self.fitness.get_or_insert(self.eval_fitness())
    }

    fn get_fitness(&self) -> Option<FitnessScore> {
        self.fitness
    }
}

impl<'a, T> Organism<'a> for Program<'a, ClassificationParameters<'a, T>>
where
    T: ClassificationInput,
    <T as ValidInput>::Actions: Hash + Compare,
{
}
impl<'a, T> Show for ClassificationParameters<'a, T>
where
    T: ClassificationInput,
    <T as ValidInput>::Actions: Hash + Compare,
{
}
impl<'a, T> Compare for ClassificationParameters<'a, T>
where
    T: ClassificationInput,
    <T as ValidInput>::Actions: Hash + Compare,
{
}

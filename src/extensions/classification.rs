use derive_new::new;
use num::ToPrimitive;
use ordered_float::OrderedFloat;
use serde::Serialize;

use crate::core::{
    characteristics::{Fitness, FitnessScore, Organism, Show},
    inputs::{Inputs, ValidInput},
    program::{ExtensionParameters, Program},
};

#[derive(Clone, Debug, Serialize, new)]
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
    fn get_class(&self) -> Self::Actions;
}

impl<'a, T> Fitness for Program<'a, ClassificationParameters<'a, T>>
where
    T: ClassificationInput,
{
    fn eval_fitness(&mut self) -> FitnessScore {
        let inputs = self.problem_parameters.inputs;

        let mut pred_truth_array = vec![];

        let mut n_correct = 0;

        for input in inputs {
            self.exec(input);

            let predicted_class = T::argmax(&self.registers)
                .and_then(|action| action.to_i32())
                .unwrap_or(-1);
            let correct_class = input.get_class().to_i32().unwrap();

            if predicted_class == correct_class {
                n_correct += 1;
            }

            pred_truth_array.push((predicted_class, correct_class));

            self.registers.reset();
        }

        let fitness = OrderedFloat(n_correct as f32 / inputs.len() as f32);

        self.fitness = Some(fitness);

        fitness
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

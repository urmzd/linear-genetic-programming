use derive_new::new;
use num::ToPrimitive;
use ordered_float::OrderedFloat;
use serde::Serialize;
use smartcore::metrics::accuracy::Accuracy;

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
    fn eval_fitness(&self) -> FitnessScore {
        let inputs = self.problem_parameters.inputs;

        let mut pred_truth_array = vec![];

        let mut registers = self.registers.clone();
        for input in inputs {
            for instruction in &self.instructions {
                instruction.apply(&mut registers, input);
            }

            let ties = registers.argmax();
            let predicted_class = T::argmax(ties)
                .and_then(|action| action.to_i32())
                .unwrap_or(-1);
            let correct_class = input.get_class().to_i32().unwrap();

            pred_truth_array.push((predicted_class, correct_class));

            registers.reset();
        }

        let predicted: Vec<f32> = pred_truth_array
            .iter()
            .map(|score| score.0 as f32)
            .collect();
        let correct: Vec<f32> = pred_truth_array
            .iter()
            .map(|score| score.1 as f32)
            .collect();

        let fitness = (Accuracy {}).get_score(&predicted, &correct);

        OrderedFloat(fitness)
    }

    fn get_or_eval_fitness(&mut self) -> FitnessScore {
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

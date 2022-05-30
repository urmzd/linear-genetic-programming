use crate::utils::common_traits::{Compare, Show};

use super::{chromosomes::Instruction, registers::RegisterValue};

pub type FitnessScore = RegisterValue;

pub trait Fitness: Show {
    fn eval_fitness(&self) -> FitnessScore;
    fn eval_set_fitness(&mut self) -> FitnessScore;
    fn get_fitness(&self) -> Option<FitnessScore>;
}

pub trait Breed: Show {
    fn crossover(&self, other: &Self) -> Self;
}

pub trait Mutate: Show {
    fn mutate(&mut self) -> () {}
}

pub trait Generate {
    type GenerateParamsType;

    fn generate<'a>(parameters: &'a Self::GenerateParamsType) -> Self;
}

pub trait Organism: Fitness + Generate + Compare {
    fn get_instructions(&self) -> &[Instruction];
}
use crate::utils::common_traits::{Compare, Show};

use super::{program::Instructions, registers::RegisterValue};

pub type FitnessScore = RegisterValue;

pub trait Fitness: Show {
    fn eval_fitness(&self) -> FitnessScore;
    fn eval_set_fitness(&mut self) -> FitnessScore;
    fn get_fitness(&self) -> Option<FitnessScore>;
}

pub trait Breed: Clone {
    fn two_point_crossover(&self, mate: &Self) -> [Self; 2];
}

pub trait Mutate: Show {
    fn mutate(&mut self) -> () {}
}

pub trait Generate
where
    Self::GenerateParamsType: Show,
{
    type GenerateParamsType;

    fn generate(parameters: &Self::GenerateParamsType) -> Self;
}

pub trait Organism: Fitness + Generate + Compare + Show + Sized + Clone {
    fn get_instructions(&self) -> &Instructions;
}

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

pub trait Mutate: Show
where
    Self: Sized,
{
    fn mutate(&self) -> Self;
}

pub trait Generate<'a>
where
    Self::GenerateParamsType: Show,
{
    type GenerateParamsType;

    fn generate(parameters: &'a Self::GenerateParamsType) -> Self;
}

pub trait Organism<'a>: Fitness + Generate<'a> + Compare + Show + Sized + Clone + Mutate {
    fn get_instructions(&'a self) -> &'a Instructions<'a>;
}

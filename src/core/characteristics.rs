use crate::utils::common_traits::{Compare, Show};

use super::registers::RegisterValue;

pub type FitnessScore = RegisterValue;

pub trait Fitness {
    fn eval_fitness(&self) -> FitnessScore;
    fn eval_set_fitness(&mut self) -> FitnessScore;
    fn get_fitness(&self) -> Option<FitnessScore>;
}

pub trait Breed: Clone {
    // TODO: Add parameter to select number of "genes".
    fn two_point_crossover(&self, mate: &Self) -> [Self; 2];
}

pub trait Mutate: Clone {
    fn mutate(&self) -> Self;
}

pub trait Generate<'a, T = Self> {
    type GeneratorParameters;

    fn generate(parameters: &'a Self::GeneratorParameters) -> Self;
}

pub trait Organism<'a>:
    Fitness + Generate<'a> + Compare + Show + Sized + Clone + Mutate + Breed
{
}

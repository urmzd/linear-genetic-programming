use std::fmt;

use serde::Serialize;

use super::registers::RegisterValue;

pub type FitnessScore = RegisterValue;

pub trait Fitness {
    type FitnessParameters;
    fn eval_fitness(&mut self) -> FitnessScore;
    fn get_fitness(&self) -> Option<FitnessScore>;
}

pub trait Breed: Clone {
    // TODO: Add parameter to select number of "genes".
    fn two_point_crossover(&self, mate: &Self) -> [Self; 2];
}

pub trait Mutate: Clone {
    type MutateParameters;

    fn mutate(&self, parameters: Self::MutateParameters) -> Self;
}

pub trait Generate {
    type GeneratorParameters;

    fn generate<'a>(parameters: &'a Self::GeneratorParameters) -> Self;
}

pub trait Organism: Fitness + Generate + Compare + Show + Sized + Clone + Mutate + Breed {}

pub trait Compare<V = Self>: PartialEq<V> + Eq + PartialOrd<V> {}
pub trait Show: fmt::Debug + Serialize {}

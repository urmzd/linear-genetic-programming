use super::registers::R32;

pub type FitnessScore = R32;

pub trait Fitness {
    type FitnessParameters;

    fn eval_fitness(&mut self, parameters: &mut Self::FitnessParameters) -> FitnessScore;
    fn get_fitness(&self) -> Option<FitnessScore>;
}

pub trait Breed: Clone {
    fn two_point_crossover(&self, mate: &Self) -> [Self; 2];
}

pub trait Mutate: Generate + Clone {
    fn mutate<'a>(&self, parameters: &'a Self::GeneratorParameters) -> Self;
}

pub trait Generate {
    type GeneratorParameters;

    fn generate<'a>(parameters: &'a Self::GeneratorParameters) -> Self;
}

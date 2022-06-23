use std::path::PathBuf;

use csv::ReaderBuilder;
use more_asserts::assert_le;
use rand::prelude::{IteratorRandom, SliceRandom};

use crate::{
    genes::characteristics::{Breed, Fitness, Generate},
    utils::{
        common_traits::{Inputs, ValidInput},
        random::generator,
    },
};

use log::trace;

use super::{
    characteristics::{Mutate, Organism},
    population::Population,
};

#[derive(Clone, Debug)]
pub struct HyperParameters<'a, OrganismType>
where
    OrganismType: Organism<'a>,
{
    pub population_size: usize,
    pub gap: f32,
    pub max_generations: usize,
    pub program_params: OrganismType::GenerateParamsType,
}

pub trait Loader
where
    Self::InputType: ValidInput,
{
    type InputType;

    fn load_inputs(file_path: impl Into<PathBuf>) -> Inputs<Self::InputType> {
        let mut csv_reader = ReaderBuilder::new()
            .has_headers(false)
            .from_path(file_path.into())
            .unwrap();

        let mut inputs: Vec<Self::InputType> = vec![];

        for input in csv_reader.deserialize() {
            let record: Self::InputType = input.unwrap();
            inputs.push(record);
        }

        inputs
    }
}

pub trait GeneticAlgorithm<'a>
where
    Self::O: Organism<'a>,
{
    type O;

    fn init_env() -> () {
        // Prevent errors from being thrown when "multple" initializations occur.
        pretty_env_logger::try_init().unwrap_or(());
    }

    fn init_population(hyper_params: &'a HyperParameters<'a, Self::O>) -> Population<Self::O> {
        let mut population = Population::new_with_capacity(hyper_params.population_size);

        for _ in 0..hyper_params.population_size {
            let program = Self::O::generate(&hyper_params.program_params);
            population.push_back(program)
        }

        population
    }

    fn evaluate(population: &mut Population<Self::O>) -> () {
        for individual in population.iter_mut() {
            individual.eval_set_fitness();
        }
    }

    fn rank(population: &mut Population<Self::O>) -> () {
        population.sort();
    }

    fn apply_selection(population: &mut Population<Self::O>, gap: f32) -> () {
        assert!(gap >= 0f32 && gap <= 1f32);

        assert_le!(
            population.first().unwrap().eval_fitness(),
            population.last().unwrap().eval_fitness()
        );

        let pop_len = population.len();

        let lowest_index = ((1f32 - gap) * (pop_len as f32)).floor() as i32 as usize;

        for _ in 0..lowest_index {
            population.pop_front();
        }
    }

    fn breed(
        population: &mut Population<Self::O>,
        n_mutations: Option<usize>,
        n_crossovers: Option<usize>,
    ) -> () {
        let pop_cap = population.capacity();
        let pop_len = population.len();
        let mut remaining_size: usize = pop_cap - pop_len;

        let mut n_mutations_todo = n_mutations.unwrap_or(0);
        let mut n_crossovers_todo = n_crossovers.unwrap_or(0);

        assert_le!(n_mutations_todo + n_crossovers_todo, remaining_size);

        // Mutate
        while n_mutations_todo > 0 {
            let selected_individual = population.iter().choose(&mut generator());
            let mutated_child = selected_individual
                .map(|mut parent| parent.mutate())
                .unwrap();
            population.push_back(mutated_child);
            remaining_size -= 1;
            n_mutations_todo -= 1;
        }

        // Crossover
        while n_crossovers_todo > 0 {
            if let &mut [parent_a, parent_b] = population
                .iter()
                .choose_multiple(&mut generator(), 2)
                .as_mut_slice()
            {
                let child = parent_a
                    .two_point_crossover(parent_b)
                    .choose(&mut generator())
                    .unwrap()
                    .to_owned();

                population.push_back(child);

                remaining_size -= 1;
                n_crossovers_todo -= 1;
            };
        }

        // Fill reset with clones
        for individual in population
            .iter()
            .cloned()
            .choose_multiple(&mut generator(), remaining_size)
        {
            population.push_back(individual)
        }
    }

    // TODO: Add hooks?
    fn execute(hyper_params: &'a HyperParameters<'a, Self::O>) -> Population<Self::O> {
        Self::init_env();

        trace!("{:#?}", hyper_params);

        let mut population = Self::init_population(hyper_params);

        for _ in 0..hyper_params.max_generations {
            Self::apply_selection(&mut population, hyper_params.gap);
            Self::evaluate(&mut population);
            Self::rank(&mut population);
            Self::breed(&mut population, None, None);
        }

        population
    }
}

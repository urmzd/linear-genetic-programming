use std::path::PathBuf;

use csv::ReaderBuilder;
use more_asserts::{assert_ge, assert_le};
use ordered_float::OrderedFloat;
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
    pub n_mutations: f32,
    pub n_crossovers: f32,
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

    fn breed(population: &mut Population<Self::O>, n_mutations: f32, n_crossovers: f32) -> () {
        let pop_cap = population.capacity();
        let pop_len = population.len();
        let mut remaining_size: usize = pop_cap - pop_len;

        assert_ge!(OrderedFloat(n_mutations), OrderedFloat(0f32));
        assert_ge!(OrderedFloat(n_crossovers), OrderedFloat(0f32));
        assert_le!(OrderedFloat(n_crossovers + n_mutations), OrderedFloat(1f32));
        assert_le!(OrderedFloat(n_mutations), OrderedFloat(1f32));
        assert_le!(OrderedFloat(n_crossovers), OrderedFloat(1f32));

        let mut n_mutations_todo =
            math::round::floor((n_mutations * remaining_size as f32) as f64, 0) as usize;
        let mut n_crossovers_todo =
            math::round::floor((n_crossovers * remaining_size as f32) as f64, 0) as usize;

        assert_le!(n_mutations_todo + n_crossovers_todo, remaining_size);

        // Mutate
        while n_mutations_todo > 0 {
            let selected_individual = population.iter().choose(&mut generator());
            let mutated_child = selected_individual.map(|parent| parent.mutate()).unwrap();
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

    fn execute<A, B, C, D, E>(
        hyper_params: &'a HyperParameters<'a, Self::O>,
        mut after_init: A,
        mut after_evaluate: B,
        mut after_selection: C,
        mut after_rank: D,
        mut after_breed: E,
    ) -> Result<Population<Self::O>, Box<dyn std::error::Error>>
    where
        A: FnMut(&mut Population<Self::O>) -> Result<(), Box<dyn std::error::Error>>,
        B: FnMut(&mut Population<Self::O>) -> Result<(), Box<dyn std::error::Error>>,
        C: FnMut(&mut Population<Self::O>) -> Result<(), Box<dyn std::error::Error>>,
        D: FnMut(&mut Population<Self::O>) -> Result<(), Box<dyn std::error::Error>>,
        E: FnMut(&mut Population<Self::O>) -> Result<(), Box<dyn std::error::Error>>,
    {
        Self::init_env();

        trace!("{:#?}", hyper_params);

        let mut population = Self::init_population(hyper_params);

        after_init(&mut population)?;

        for _ in 0..hyper_params.max_generations {
            // Step 1: Evaluate Fitness
            Self::evaluate(&mut population);
            after_evaluate(&mut population)?;

            // Step 2: Sort
            Self::rank(&mut population);
            after_rank(&mut population)?;

            // Step 3: Drop by Gap
            Self::apply_selection(&mut population, hyper_params.gap);
            after_selection(&mut population)?;

            // Step 4: Crossover + Mutation
            Self::breed(
                &mut population,
                hyper_params.n_mutations,
                hyper_params.n_crossovers,
            );

            after_breed(&mut population)?;
        }

        Ok(population)
    }
}

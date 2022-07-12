use core::fmt;
use std::path::PathBuf;

use csv::ReaderBuilder;
use more_asserts::{assert_ge, assert_le};
use ordered_float::OrderedFloat;
use rand::prelude::{IteratorRandom, SliceRandom};
use serde::de::DeserializeOwned;

use crate::{
    core::characteristics::{Breed, Fitness, Generate},
    utils::{
        common_traits::{Inputs, ValidInput},
        random::generator,
    },
};

use super::{
    characteristics::{Mutate, Organism},
    population::Population,
};

#[derive(Debug)]
pub struct HyperParameters<'a, OrganismType>
where
    OrganismType: Organism<'a>,
{
    pub population_size: usize,
    pub gap: f32,
    pub n_mutations: f32,
    pub n_crossovers: f32,
    pub max_generations: usize,
    pub program_params: OrganismType::GeneratorParameters,
}

pub trait Loader
where
    Self::InputType: ValidInput + DeserializeOwned,
{
    type InputType;

    fn load_inputs(file_path: impl Into<PathBuf>) -> Inputs<Self::InputType> {
        let mut csv_reader = ReaderBuilder::new()
            .has_headers(false)
            .from_path(file_path.into())
            .unwrap();

        let inputs: Result<Inputs<Self::InputType>, _> = csv_reader
            .deserialize()
            .into_iter()
            .map(|input| input)
            .collect();

        inputs.unwrap()
    }
}

pub trait GeneticAlgorithm<'a>
where
    Self::O: Organism<'a>,
{
    type O;

    /// Prevent errors from being thrown when "multple" initializations occur.
    fn init_env() -> () {
        pretty_env_logger::try_init().unwrap_or(());
    }

    fn init_population(hyper_params: &'a HyperParameters<'a, Self::O>) -> Population<Self::O> {
        let mut population = Population::with_capacity(hyper_params.population_size);

        for _ in 0..hyper_params.population_size {
            let program = Self::O::generate(&hyper_params.program_params);
            population.push(program)
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

        assert_le!(population.first(), population.last());

        let pop_len = population.len();

        let cutoff_index = ((1f32 - gap) * (pop_len as f32)).floor() as i32 as usize;

        for _ in 0..cutoff_index {
            population.pop();
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

        let mut n_mutations_todo = ((n_mutations * remaining_size as f32) as f64).floor() as usize;
        let mut n_crossovers_todo =
            ((n_crossovers * remaining_size as f32) as f64).floor() as usize;

        assert_le!(n_mutations_todo + n_crossovers_todo, remaining_size);

        // Crossover + Mutation
        while (n_crossovers_todo + n_mutations_todo) > 0 {
            if let [parent_a, parent_b] = population
                .iter()
                .choose_multiple(&mut generator(), 2)
                .as_slice()
            {
                let mut child_a = None;
                let mut child_b = None;
                if n_crossovers_todo > 0 {
                    let crossover_child = parent_a
                        .two_point_crossover(parent_b)
                        .choose(&mut generator())
                        .unwrap()
                        .to_owned();

                    child_a = Some(crossover_child);

                    remaining_size -= 1;
                    n_crossovers_todo -= 1;
                }

                if n_mutations_todo > 0 {
                    let parents = [parent_a, parent_b];
                    let selected_parent = parents.choose(&mut generator());

                    let mutation_child = selected_parent.map(|parent| parent.mutate()).unwrap();

                    child_b = Some(mutation_child);

                    remaining_size -= 1;
                    n_mutations_todo -= 1;
                }

                if child_a.is_some() {
                    population.push(child_a.unwrap())
                }

                if child_b.is_some() {
                    population.push(child_b.unwrap())
                }
            };
        }

        // Fill reset with clones
        for individual in population
            .iter()
            .cloned()
            .choose_multiple(&mut generator(), remaining_size)
        {
            population.push(individual)
        }
    }

    fn execute<'b>(
        hyper_params: &'a HyperParameters<'a, Self::O>,
        mut hooks: EventHooks<'a, Self::O>,
    ) -> Result<Population<Self::O>, Box<dyn std::error::Error>> {
        Self::init_env();

        let EventHooks {
            after_init,
            after_evaluate,
            after_rank,
            after_selection,
            after_breed,
            ..
        } = &mut hooks;

        let mut population = Self::init_population(hyper_params);

        if let Some(hook) = after_init {
            (hook)(&mut population)?;
        }

        for _ in 0..hyper_params.max_generations {
            // Step 1: Evaluate Fitness
            Self::evaluate(&mut population);
            if let Some(hook) = after_evaluate {
                (hook)(&mut population)?;
            }

            // Step 2: Sort
            Self::rank(&mut population);
            if let Some(hook) = after_rank {
                (hook)(&mut population)?;
            }

            // Step 3: Drop by Gap
            Self::apply_selection(&mut population, hyper_params.gap);
            if let Some(hook) = after_selection {
                (hook)(&mut population)?;
            }

            // Step 4: Crossover + Mutation
            Self::breed(
                &mut population,
                hyper_params.n_mutations,
                hyper_params.n_crossovers,
            );

            if let Some(hook) = after_breed {
                (hook)(&mut population)?;
            }
        }

        Ok(population)
    }
}

pub type GpHook<'a, O> =
    &'a mut dyn FnMut(&mut Population<O>) -> Result<(), Box<dyn std::error::Error>>;
pub struct EventHooks<'a, O>
where
    O: Organism<'a>,
{
    pub after_init: Option<GpHook<'a, O>>,
    pub after_evaluate: Option<GpHook<'a, O>>,
    pub after_rank: Option<GpHook<'a, O>>,
    pub after_selection: Option<GpHook<'a, O>>,
    pub after_breed: Option<GpHook<'a, O>>,
}

impl<'a, O> EventHooks<'a, O>
where
    O: Organism<'a>,
{
    pub fn with_after_init(self, f: GpHook<'a, O>) -> Self {
        Self {
            after_init: Some(f),
            ..self
        }
    }

    pub fn with_after_evaluate(self, f: GpHook<'a, O>) -> Self {
        Self {
            after_evaluate: Some(f),
            ..self
        }
    }

    pub fn with_after_selection(self, f: GpHook<'a, O>) -> Self {
        Self {
            after_selection: Some(f),
            ..self
        }
    }

    pub fn with_after_rank(self, f: GpHook<'a, O>) -> Self {
        Self {
            after_rank: Some(f),
            ..self
        }
    }

    pub fn with_after_breed<F>(self, f: GpHook<'a, O>) -> Self {
        Self {
            after_breed: Some(f),
            ..self
        }
    }
}

impl<'a, O> fmt::Debug for EventHooks<'a, O>
where
    O: Organism<'a>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EventHooks")
            .field("after_init", &"after_init")
            .field("after_evaluate", &"after_evaluate")
            .field("after_selection", &"after_selection")
            .field("after_rank", &"after_rank")
            .field("after_breed", &"after_breed")
            .finish()
    }
}

impl<'a, O> Default for EventHooks<'a, O>
where
    O: Organism<'a>,
{
    fn default() -> Self {
        Self {
            after_init: None,
            after_evaluate: None,
            after_rank: None,
            after_selection: None,
            after_breed: None,
        }
    }
}

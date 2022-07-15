mod set_up;

use std::error;

use lgp::{
    core::{
        algorithm::{EventHooks, GeneticAlgorithm, HyperParameters, Loader},
        instruction::InstructionGeneratorParameters,
        program::ProgramGeneratorParameters,
        registers::RegisterGeneratorParameters,
    },
    extensions::classification::ClassificationParameters,
};
use set_up::{get_iris_content, ContentFilePair, IrisLgp};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let ContentFilePair(_, file) = get_iris_content().await?;
    let inputs = IrisLgp::load_inputs(file.path());

    let hyper_params = HyperParameters {
        population_size: 100,
        max_generations: 100,
        gap: 0.5,
        n_mutations: 0.5,
        n_crossovers: 0.5,
        program_params: ProgramGeneratorParameters::new(
            100,
            InstructionGeneratorParameters::from(1),
            RegisterGeneratorParameters::new(1),
            ClassificationParameters::new(&inputs),
        ),
    };

    IrisLgp::execute(&hyper_params, EventHooks::default())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use lgp::{
        core::{
            algorithm::{EventHooks, GeneticAlgorithm, HyperParameters, Loader},
            instruction::InstructionGeneratorParameters,
            program::{Program, ProgramGeneratorParameters},
            registers::RegisterGeneratorParameters,
        },
        extensions::classification::ClassificationParameters,
        utils::plots::plot_population_benchmarks,
    };
    use more_asserts::{assert_le, assert_lt};
    use ndarray::{aview1, s, Array, Array2, Axis, Dim};
    use ndarray_stats::{interpolate::Higher, QuantileExt};
    use noisy_float::prelude::n64;
    use pretty_assertions::{assert_eq, assert_ne};
    use std::{error, mem::MaybeUninit};

    use crate::set_up::{get_iris_content, ContentFilePair, IrisInput, IrisLgp};

    // TODO: Update tests to include assertions about benchmark trends.
    #[tokio::test]
    async fn given_lgp_instance_with_mutation_and_crossover_operations_when_sufficient_iterations_have_been_met_then_population_shows_increase_in_median_and_best_fitness(
    ) -> Result<(), Box<dyn error::Error>> {
        IrisLgp::init_env();

        let ContentFilePair(_, tmp_file) = get_iris_content().await?;
        let inputs = IrisLgp::load_inputs(tmp_file.path());

        let hyper_params: HyperParameters<Program<ClassificationParameters<IrisInput>>> =
            HyperParameters {
                population_size: 5,
                max_generations: 100,
                gap: 0.5,
                n_mutations: 0.5,
                n_crossovers: 0.5,
                program_params: ProgramGeneratorParameters::new(
                    100,
                    InstructionGeneratorParameters::<IrisInput>::from(1),
                    RegisterGeneratorParameters::new(1),
                    ClassificationParameters::new(&inputs),
                ),
            };

        assert_eq!(hyper_params.n_crossovers, 0.5);
        assert_eq!(hyper_params.n_mutations, 0.5);

        let mut uninit_populations: Array<
            MaybeUninit<Program<ClassificationParameters<IrisInput>>>,
            Dim<[usize; 2]>,
        > = Array2::uninit((hyper_params.max_generations, hyper_params.population_size));
        let mut generations: usize = 0;

        IrisLgp::execute(
            &hyper_params,
            EventHooks::default().with_after_rank(&mut |population| {
                population
                    .clone()
                    .into_ndarray()
                    .assign_to(uninit_populations.slice_mut(s![generations, ..]));
                generations += 1;
                Ok(())
            }),
        )?;

        let init_populations = unsafe { uninit_populations.assume_init() };

        const PLOT_FILE_NAME: &'static str =
            "./assets/tests/plots/lgp_with_mutate_crossover_test.png";
        plot_population_benchmarks(init_populations, PLOT_FILE_NAME, 0f32..1f32)?;
        Ok(())
    }
    #[tokio::test]
    async fn given_lgp_instance_with_mutation_operations_when_sufficient_iterations_have_been_met_then_population_shows_increase_in_median_and_best_fitness(
    ) -> Result<(), Box<dyn error::Error>> {
        IrisLgp::init_env();

        let ContentFilePair(_, tmp_file) = get_iris_content().await?;
        let inputs = IrisLgp::load_inputs(tmp_file.path());

        let hyper_params: HyperParameters<Program<ClassificationParameters<IrisInput>>> =
            HyperParameters {
                population_size: 100,
                max_generations: 100,
                gap: 0.5,
                n_mutations: 0.5,
                n_crossovers: 0.,
                program_params: ProgramGeneratorParameters::new(
                    100,
                    InstructionGeneratorParameters::<IrisInput>::from(1),
                    RegisterGeneratorParameters::new(1),
                    ClassificationParameters::new(&inputs),
                ),
            };

        assert_eq!(hyper_params.n_crossovers, 0.);
        assert_eq!(hyper_params.n_mutations, 0.5);

        let mut uninit_populations =
            Array2::uninit((hyper_params.max_generations, hyper_params.population_size));
        let mut generations: usize = 0;

        IrisLgp::execute(
            &hyper_params,
            EventHooks::default().with_after_rank(&mut |population| {
                population
                    .clone()
                    .into_ndarray()
                    .assign_to(uninit_populations.slice_mut(s![generations, ..]));
                generations += 1;
                Ok(())
            }),
        )?;

        let init_populations = unsafe { uninit_populations.assume_init() };

        const PLOT_FILE_NAME: &'static str = "./assets/tests/plots/lgp_with_mutate_test.png";
        plot_population_benchmarks(init_populations, PLOT_FILE_NAME, 0f32..1f32)?;
        Ok(())
    }

    #[tokio::test]
    async fn given_lgp_instance_with_crossover_operations_when_sufficient_iterations_have_been_met_then_population_shows_increase_in_worst_and_median_fitness(
    ) -> Result<(), Box<dyn error::Error>> {
        IrisLgp::init_env();

        let ContentFilePair(_, tmp_file) = get_iris_content().await?;
        let inputs = IrisLgp::load_inputs(tmp_file.path());

        let hyper_params: HyperParameters<Program<ClassificationParameters<IrisInput>>> =
            HyperParameters {
                population_size: 100,
                max_generations: 100,
                gap: 0.5,
                n_mutations: 0.,
                n_crossovers: 0.5,
                program_params: ProgramGeneratorParameters::new(
                    100,
                    InstructionGeneratorParameters::<IrisInput>::from(1),
                    RegisterGeneratorParameters::new(1),
                    ClassificationParameters::new(&inputs),
                ),
            };

        assert_eq!(hyper_params.n_crossovers, 0.5);
        assert_eq!(hyper_params.n_mutations, 0.);

        let mut uninit_populations =
            Array2::uninit((hyper_params.max_generations, hyper_params.population_size));
        let mut generations: usize = 0;

        IrisLgp::execute(
            &hyper_params,
            EventHooks::default().with_after_rank(&mut |population| {
                population
                    .clone()
                    .into_ndarray()
                    .assign_to(uninit_populations.slice_mut(s![generations, ..]));
                generations += 1;
                Ok(())
            }),
        )?;

        let init_populations = unsafe { uninit_populations.assume_init() };

        const PLOT_FILE_NAME: &'static str = "./assets/tests/plots/lgp_with_crossover_test.png";
        plot_population_benchmarks(init_populations, PLOT_FILE_NAME, 0f32..1f32)?;

        Ok(())
    }

    #[tokio::test]
    async fn given_lgp_instance_when_sufficient_iterations_have_been_used_then_population_contains_the_same_benchmark_fitness(
    ) -> Result<(), Box<dyn error::Error>> {
        IrisLgp::init_env();

        let ContentFilePair(_, tmp_file) = get_iris_content().await?;
        let inputs = IrisLgp::load_inputs(tmp_file.path());

        let hyper_params: HyperParameters<Program<ClassificationParameters<IrisInput>>> =
            HyperParameters {
                population_size: 100,
                max_generations: 100,
                gap: 0.5,
                n_mutations: 0.,
                n_crossovers: 0.,
                program_params: ProgramGeneratorParameters::new(
                    100,
                    InstructionGeneratorParameters::<IrisInput>::from(1),
                    RegisterGeneratorParameters::new(1),
                    ClassificationParameters::new(&inputs),
                ),
            };

        assert_eq!(hyper_params.n_crossovers, 0.);
        assert_eq!(hyper_params.n_mutations, 0.);

        let mut generations: usize = 0;

        let mut vec_pops = vec![];

        let mut best = None;
        let mut median = None;
        let mut worst = None;

        IrisLgp::execute(
            &hyper_params,
            EventHooks::default().with_after_rank(&mut |population| {
                let array = population.clone().into_ndarray();

                vec_pops.push(array);

                let current_array = vec_pops.get_mut(generations).unwrap();

                let qs = &[n64(0.), n64(0.5), n64(1.)];
                let benchmark = current_array
                    .quantiles_axis_mut(Axis(0), &aview1(qs), &Higher)
                    .unwrap();

                worst = benchmark.get([0]).map(|v| v.clone());
                median = benchmark.get([1]).map(|v| v.clone());
                best = benchmark.get([2]).map(|v| v.clone());

                generations += 1;
                Ok(())
            }),
        )?;

        let mut uninit_populations = Array2::uninit((generations, hyper_params.population_size));

        (0..generations).for_each(|g_i| {
            let array = vec_pops.get(g_i).unwrap();
            array.assign_to(uninit_populations.slice_mut(s![g_i, ..]))
        });

        let init_populations = unsafe { uninit_populations.assume_init() };

        // TODO: Pull the graph section out into a seperate function.
        const PLOT_FILE_NAME: &'static str = "./assets/tests/plots/lgp_smoke_test.png";
        plot_population_benchmarks(init_populations, PLOT_FILE_NAME, 0f32..1f32)?;

        if worst != median || median != best {
            // TODO: Create concrete error type; SNAFU or Failure?
            panic!("Generations exceeded expect convergence time.")
        }

        Ok(())
    }

    #[tokio::test]
    async fn given_population_when_breeding_occurs_then_population_capacity_is_met(
    ) -> Result<(), Box<dyn error::Error>> {
        let ContentFilePair(_, tmp_file) = get_iris_content().await?;

        let inputs = IrisLgp::load_inputs(tmp_file.path());
        let hyper_params: HyperParameters<Program<ClassificationParameters<IrisInput>>> =
            HyperParameters {
                population_size: 100,
                max_generations: 100,
                gap: 0.5,
                n_mutations: 0.,
                n_crossovers: 0.5,
                program_params: ProgramGeneratorParameters::new(
                    100,
                    InstructionGeneratorParameters::<IrisInput>::from(1),
                    RegisterGeneratorParameters::new(1),
                    ClassificationParameters::new(&inputs),
                ),
            };

        let mut population = IrisLgp::init_population(&hyper_params);

        IrisLgp::rank(&mut population);
        IrisLgp::rank(&mut population);
        IrisLgp::apply_selection(&mut population, hyper_params.gap);

        let dropped_pop_len = population.len();

        assert_lt!(dropped_pop_len, hyper_params.population_size);

        IrisLgp::breed(&mut population, 0f32, 0f32);

        assert_eq!(population.len(), hyper_params.population_size);

        Ok(())
    }

    #[tokio::test]
    async fn given_population_and_retention_rate_when_selection_occurs_then_population_is_cut_by_dropout(
    ) -> Result<(), Box<dyn error::Error>> {
        let ContentFilePair(_, tmp_file) = get_iris_content().await?;

        let inputs = IrisLgp::load_inputs(tmp_file.path());

        let hyper_params: HyperParameters<Program<ClassificationParameters<IrisInput>>> =
            HyperParameters {
                population_size: 100,
                max_generations: 100,
                gap: 0.5,
                n_mutations: 0.,
                n_crossovers: 0.5,
                program_params: ProgramGeneratorParameters::new(
                    100,
                    InstructionGeneratorParameters::<IrisInput>::from(1),
                    RegisterGeneratorParameters::new(1),
                    ClassificationParameters::new(&inputs),
                ),
            };

        let mut population = IrisLgp::init_population(&hyper_params);

        IrisLgp::rank(&mut population);
        IrisLgp::rank(&mut population);
        IrisLgp::apply_selection(&mut population, hyper_params.gap);

        self::assert_eq!(
            population.len(),
            ((hyper_params.population_size as f32 * (1f32 - hyper_params.gap)).floor() as i32
                as usize)
        );

        Ok(())
    }

    #[tokio::test]
    async fn given_inputs_and_hyperparams_when_population_is_initialized_then_population_generated_with_hyperparams_and_inputs(
    ) -> Result<(), Box<dyn error::Error>> {
        let ContentFilePair(_, tmp_file) = get_iris_content().await?;

        let inputs = IrisLgp::load_inputs(tmp_file.path());

        let hyper_params: HyperParameters<Program<ClassificationParameters<IrisInput>>> =
            HyperParameters {
                population_size: 100,
                max_generations: 100,
                gap: 0.5,
                n_mutations: 0.,
                n_crossovers: 0.5,
                program_params: ProgramGeneratorParameters::new(
                    100,
                    InstructionGeneratorParameters::<IrisInput>::from(1),
                    RegisterGeneratorParameters::new(1),
                    ClassificationParameters::new(&inputs),
                ),
            };

        let population = IrisLgp::init_population(&hyper_params);

        self::assert_eq!(population.len(), hyper_params.population_size);

        for individual in population {
            assert_le!(individual.instructions.len(), 100)
        }

        Ok(())
    }

    #[tokio::test]
    async fn given_iris_dataset_when_csv_is_read_then_rows_are_deserialized_as_structs(
    ) -> Result<(), Box<dyn error::Error>> {
        let ContentFilePair(content, _) = get_iris_content().await?;
        assert_ne!(content.len(), 0);

        let content_bytes = content.as_bytes();

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(content_bytes);

        let data = reader.deserialize();
        let mut count = 0;

        for result in data {
            let _record: IrisInput = result?;
            count += 1;
        }

        assert_ne!(count, 0);

        Ok(())
    }

    #[tokio::test]
    async fn given_iris_dataset_when_csv_path_is_provided_then_collection_of_iris_structs_are_returned(
    ) -> Result<(), Box<dyn error::Error>> {
        let ContentFilePair(_, tmpfile) = get_iris_content().await?;
        let inputs = IrisLgp::load_inputs(tmpfile.path());
        assert_ne!(inputs.len(), 0);
        Ok(())
    }
}

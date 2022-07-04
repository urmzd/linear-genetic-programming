use std::error;

use lgp::{
    core::{
        algorithm::{EventHooks, GeneticAlgorithm, HyperParameters, Loader},
        characteristics::FitnessScore,
        instruction::{InstructionGeneratorParameters, Mode},
        program::{Program, ProgramGeneratorParameters},
    },
    examples::iris::{data::IrisInput, ops::IRIS_EXECUTABLES},
    extensions::classification::ClassificationParameters,
    measure::benchmarks::{Benchmark, ComplexityBenchmark},
    utils::common_traits::ValidInput,
};

use lgp::examples::iris::{
    data::IrisLgp,
    utils::{get_iris_content, ContentFilePair},
};
use more_asserts::{assert_le, assert_lt};
use plotters::{
    prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea, LineSeries, PathElement},
    style::{Color, IntoFont, BLACK, BLUE, GREEN, RED, WHITE},
};
use pretty_assertions::{assert_eq, assert_ne};

fn plot_from_benchmarks(
    benchmarks: Vec<ComplexityBenchmark<Option<FitnessScore>>>,
    plot_path: &str,
) -> Result<(), Box<dyn error::Error>> {
    let fitness_benchmarks: Vec<ComplexityBenchmark<f32>> = benchmarks
        .into_iter()
        .map(|benchmark| ComplexityBenchmark {
            best: benchmark.best.unwrap().into_inner(),
            worst: benchmark.worst.unwrap().into_inner(),
            median: benchmark.median.unwrap().into_inner(),
        })
        .collect();
    let root = BitMapBackend::new(plot_path, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Fitness Over Generations", ("sans-serif", 50).into_font())
        .margin(5u32)
        .x_label_area_size(30u32)
        .y_label_area_size(30u32)
        .build_cartesian_2d(0..fitness_benchmarks.len(), 0f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (0..fitness_benchmarks.len())
                .map(|x_i| (x_i, fitness_benchmarks.get(x_i).unwrap().best)),
            &RED,
        ))?
        .label("Best")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .draw_series(LineSeries::new(
            (0..fitness_benchmarks.len())
                .map(|x_i| (x_i, fitness_benchmarks.get(x_i).unwrap().median)),
            &GREEN,
        ))?
        .label("Median")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    chart
        .draw_series(LineSeries::new(
            (0..fitness_benchmarks.len())
                .map(|x_i| (x_i, fitness_benchmarks.get(x_i).unwrap().worst)),
            &BLUE,
        ))?
        .label("Worst")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}

// TODO: Update tests to include assertions about benchmark trends.
#[tokio::test]
async fn given_lgp_instance_with_mutation_and_crossover_operations_when_sufficient_iterations_have_been_met_then_population_shows_increase_in_median_and_best_fitness(
) -> Result<(), Box<dyn error::Error>> {
    IrisLgp::init_env();

    let ContentFilePair(_, tmp_file) = get_iris_content().await?;
    let inputs = IrisLgp::load_inputs(tmp_file.path());

    let hyper_params: HyperParameters<Program<ClassificationParameters<IrisInput>>> =
        HyperParameters {
            population_size: 100,
            gap: 0.5,
            max_generations: 100,
            program_params: ProgramGeneratorParameters::new(
                100,
                InstructionGeneratorParameters::new(
                    IrisInput + 1,
                    Some(IrisInput),
                    Mode::all(),
                    IRIS_EXECUTABLES,
                ),
                ClassificationParameters::new(&inputs),
            ),
            n_mutations: 0.5,
            n_crossovers: 0.5,
        };

    let mut benchmarks = vec![];
    IrisLgp::execute(
        &hyper_params,
        EventHooks::default().with_after_rank(&mut |population| {
            let benchmark = population.get_benchmark_individuals();
            benchmarks.push(benchmark);
            Ok(())
        }),
    )?;

    const PLOT_FILE_NAME: &'static str = "./assets/tests/plots/lgp_with_mutate_crossover_test.png";
    plot_from_benchmarks(benchmarks, PLOT_FILE_NAME)?;
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
            gap: 0.5,
            max_generations: 100,
            program_params: ProgramGeneratorParameters::new(
                100,
                InstructionGeneratorParameters::new(
                    IrisInput::N_OUTPUTS + 1,
                    Some(IrisInput::N_INPUTS),
                    Mode::all(),
                    IRIS_EXECUTABLES,
                ),
                ClassificationParameters::new(&inputs),
            ),
            n_mutations: 0.5,
            n_crossovers: 0.,
        };

    let mut benchmarks = vec![];
    IrisLgp::execute(
        &hyper_params,
        EventHooks::default().with_after_rank(&mut |population| {
            let benchmark = population.get_benchmark_individuals();
            benchmarks.push(benchmark);
            Ok(())
        }),
    )?;

    const PLOT_FILE_NAME: &'static str = "./assets/tests/plots/lgp_with_mutate_test.png";
    plot_from_benchmarks(benchmarks, PLOT_FILE_NAME)?;
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
            gap: 0.5,
            max_generations: 100,
            program_params: ProgramGeneratorParameters::new(
                100,
                InstructionGeneratorParameters::new(
                    IrisInput::N_OUTPUTS + 1,
                    Some(IrisInput::N_INPUTS),
                    Mode::all(),
                    IRIS_EXECUTABLES,
                ),
                ClassificationParameters::new(&inputs),
            ),
            n_mutations: 0.,
            n_crossovers: 0.5,
        };

    let mut benchmarks = vec![];
    IrisLgp::execute(
        &hyper_params,
        EventHooks::default().with_after_rank(&mut |population| {
            let benchmark = population.get_benchmark_individuals();
            benchmarks.push(benchmark);
            Ok(())
        }),
    )?;

    const PLOT_FILE_NAME: &'static str = "./assets/tests/plots/lgp_with_crossover_test.png";
    plot_from_benchmarks(benchmarks, PLOT_FILE_NAME)?;
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
            gap: 0.5,
            max_generations: 100,
            program_params: ProgramGeneratorParameters::new(
                100,
                InstructionGeneratorParameters::new(
                    IrisInput::N_OUTPUTS + 1,
                    Some(IrisInput::N_INPUTS),
                    Mode::all(),
                    IRIS_EXECUTABLES,
                ),
                ClassificationParameters::new(&inputs),
            ),
            n_mutations: 0.5,
            n_crossovers: 0.5,
        };

    let mut generations = 0;

    let mut benchmarks = vec![];
    IrisLgp::execute(
        &hyper_params,
        EventHooks::default().with_after_rank(&mut |population| {
            let benchmark = population.get_benchmark_individuals();

            if benchmark.worst == benchmark.median && benchmark.median == benchmark.best {
            } else {
                generations += 1;

                if generations > hyper_params.max_generations {
                    // TODO: Create concrete error type; SNAFU or Failure?
                    return Err("Generations exceeded expect convergence time.")?;
                }
            }

            benchmarks.push(benchmark);
            Ok(())
        }),
    )?;

    // TODO: Pull the graph section out into a seperate function.
    const PLOT_FILE_NAME: &'static str = "./assets/tests/plots/lgp_smoke_test.png";
    plot_from_benchmarks(benchmarks, PLOT_FILE_NAME)?;

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
            gap: 0.5,
            max_generations: 100,
            program_params: ProgramGeneratorParameters::new(
                100,
                InstructionGeneratorParameters::new(
                    IrisInput::N_OUTPUTS + 1,
                    Some(IrisInput::N_INPUTS),
                    Mode::all(),
                    IRIS_EXECUTABLES,
                ),
                ClassificationParameters::new(&inputs),
            ),
            n_mutations: 0.5,
            n_crossovers: 0.5,
        };

    let mut population = IrisLgp::init_population(&hyper_params);

    IrisLgp::evaluate(&mut population);
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
            gap: 0.5,
            max_generations: 100,
            program_params: ProgramGeneratorParameters::new(
                100,
                InstructionGeneratorParameters::new(
                    IrisInput::N_OUTPUTS + 1,
                    Some(IrisInput::N_INPUTS),
                    Mode::all(),
                    IRIS_EXECUTABLES,
                ),
                ClassificationParameters::new(&inputs),
            ),
            n_mutations: 0.5,
            n_crossovers: 0.5,
        };

    let mut population = IrisLgp::init_population(&hyper_params);

    IrisLgp::evaluate(&mut population);
    IrisLgp::rank(&mut population);
    IrisLgp::apply_selection(&mut population, hyper_params.gap);

    self::assert_eq!(
        population.len(),
        ((hyper_params.population_size as f32 * (1f32 - hyper_params.gap)).floor() as i32 as usize)
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
            gap: 0.5,
            max_generations: 100,
            program_params: ProgramGeneratorParameters::new(
                100,
                InstructionGeneratorParameters::new(
                    IrisInput::N_OUTPUTS + 1,
                    Some(IrisInput::N_INPUTS),
                    Mode::all(),
                    IRIS_EXECUTABLES,
                ),
                ClassificationParameters::new(&inputs),
            ),
            n_mutations: 0.5,
            n_crossovers: 0.5,
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

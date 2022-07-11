use gym_rs::{envs::classical_control::mountain_car::MountainCarEnv, utils::renderer::RenderMode};
use lgp::{
    core::{
        algorithm::{EventHooks, GeneticAlgorithm, HyperParameters},
        instruction::InstructionGeneratorParameters,
        program::ProgramGeneratorParameters,
        registers::RegisterGeneratorParameters,
    },
    extensions::reinforcement_learning::ReinforcementLearningParameters,
};
use set_up::{MountainCarInput, MountainCarLgp};

mod set_up;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let game = MountainCarEnv::new(RenderMode::None, None);
    let input = MountainCarInput::new(game);

    let hyper_params = HyperParameters {
        population_size: 10,
        gap: 0.5,
        n_mutations: 0.5,
        n_crossovers: 0.5,
        max_generations: 5,
        program_params: ProgramGeneratorParameters {
            max_instructions: 200,
            instruction_generator_parameters: InstructionGeneratorParameters::new(6, None),
            register_generator_parameters: RegisterGeneratorParameters::new(3),
            other: ReinforcementLearningParameters::new(5, input),
        },
    };

    MountainCarLgp::execute(&hyper_params, EventHooks::default())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::set_up;

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

    #[tokio::test]
    async fn run_test() {
        MountainCarEnv::init_env();
        let game = MountainCarEnv::new(RenderMode::None, None);
        let input = MountainCarInput::new(game);

        let hyper_params: HyperParameters<Program<ClassificationParameters<IrisInput>>> =
            HyperParameters {
                population_size: 100,
                max_generations: 100,
                program_params: ProgramGeneratorParameters {
                    max_instructions: 100,
                    register_generator_parameters: RegisterGeneratorParameters::new(1),
                    other: ClassificationParameters::new(&inputs),
                    instruction_generator_parameters: InstructionGeneratorParameters::new(
                        <IrisInput as ValidInput>::Actions::COUNT,
                        Some(<IrisInput as ClassificationInput>::N_INPUTS),
                    ),
                },
                gap: 0.5,
                n_mutations: 0.5,
                n_crossovers: 0.5,
            };

        let mut benchmarks = vec![];
        MountainCarEnv::execute(
            &hyper_params,
            EventHooks::default().with_after_rank(&mut |population| {
                let benchmark = population.get_benchmark_individuals();
                benchmarks.push(benchmark);
                Ok(())
            }),
        )?;

        const PLOT_FILE_NAME: &'static str =
            "./assets/tests/plots/lgp_with_mutate_crossover_test.png";
        plot_from_benchmarks(benchmarks, PLOT_FILE_NAME)?;
        Ok(())
    }
}

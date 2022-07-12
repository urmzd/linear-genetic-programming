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
    use gym_rs::{
        envs::classical_control::mountain_car::MountainCarEnv, utils::renderer::RenderMode,
    };
    use lgp::{
        core::{
            algorithm::{EventHooks, GeneticAlgorithm, HyperParameters},
            instruction::InstructionGeneratorParameters,
            program::ProgramGeneratorParameters,
            registers::RegisterGeneratorParameters,
        },
        extensions::reinforcement_learning::ReinforcementLearningParameters,
        measure::benchmarks::Benchmark,
        utils::{common_traits::ValidInput, plots::plot_from_benchmarks},
    };

    use crate::set_up::{MountainCarInput, MountainCarLgp};
    use strum::EnumCount;

    #[tokio::test]
    async fn run_test() -> Result<(), Box<dyn std::error::Error>> {
        MountainCarLgp::init_env();

        let game = MountainCarEnv::new(RenderMode::None, None);
        let input = MountainCarInput::new(game);

        let hyper_params = HyperParameters {
            population_size: 100,
            max_generations: 100,
            program_params: ProgramGeneratorParameters {
                max_instructions: 100,
                register_generator_parameters: RegisterGeneratorParameters::new(1),
                other: ReinforcementLearningParameters::new(5, input),
                instruction_generator_parameters: InstructionGeneratorParameters::new(
                    <MountainCarInput as ValidInput>::Actions::COUNT,
                    Some(<MountainCarInput as ValidInput>::N_INPUTS),
                ),
            },
            gap: 0.5,
            n_mutations: 0.5,
            n_crossovers: 0.5,
        };

        let mut benchmarks = vec![];
        MountainCarLgp::execute(
            &hyper_params,
            EventHooks::default().with_after_rank(&mut |population| {
                let benchmark = population.get_benchmark_individuals();
                benchmarks.push(benchmark);
                Ok(())
            }),
        )?;

        const PLOT_FILE_NAME: &'static str = "./assets/tests/plots/mountain_car.png";
        plot_from_benchmarks(benchmarks, PLOT_FILE_NAME)?;
        Ok(())
    }
}

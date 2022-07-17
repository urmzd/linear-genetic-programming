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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let game = MountainCarEnv::new(RenderMode::Human, None);
    let input = MountainCarInput::new(game);

    let hyper_params = HyperParameters {
        population_size: 1,
        gap: 0.5,
        n_crossovers: 0.5,
        n_mutations: 0.5,
        max_generations: 1,
        program_params: ProgramGeneratorParameters::new(
            100,
            InstructionGeneratorParameters::from(1),
            RegisterGeneratorParameters::new(1),
            ReinforcementLearningParameters::new(5, 200, input),
        ),
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
        utils::plots::plot_population_benchmarks,
    };

    use crate::set_up::{MountainCarInput, MountainCarLgp};

    #[tokio::test]
    async fn run_test() -> Result<(), Box<dyn std::error::Error>> {
        MountainCarLgp::init_env();

        let game = MountainCarEnv::new(RenderMode::None, None);
        let input = MountainCarInput::new(game);

        let hyper_params = HyperParameters {
            population_size: 100,
            gap: 0.5,
            n_crossovers: 0.5,
            n_mutations: 0.5,
            max_generations: 100,
            program_params: ProgramGeneratorParameters::new(
                100,
                InstructionGeneratorParameters::from(1),
                RegisterGeneratorParameters::new(1),
                ReinforcementLearningParameters::new(5, 200, input),
            ),
        };

        let mut populations = vec![];

        MountainCarLgp::execute(
            &hyper_params,
            EventHooks::default().with_after_rank(&mut |population| {
                populations.push(population.clone());

                Ok(())
            }),
        )?;

        const PLOT_FILE_NAME: &'static str = "./assets/tests/plots/mountain_car.png";
        plot_population_benchmarks(populations, PLOT_FILE_NAME, -200f32..0f32)?;
        Ok(())
    }
}

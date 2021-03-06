use gym_rs::{envs::classical_control::cartpole::CartPoleEnv, utils::renderer::RenderMode};
use lgp::{
    core::{
        algorithm::{EventHooks, GeneticAlgorithm, HyperParameters},
        instruction::InstructionGeneratorParameters,
        program::ProgramGeneratorParameters,
    },
    extensions::reinforcement_learning::ReinforcementLearningParameters,
};
use set_up::{CartPoleInput, CartPoleLgp};

mod set_up;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let environment = CartPoleEnv::new(RenderMode::Human);
    let input = CartPoleInput::new(environment);

    let mut hyper_params = HyperParameters {
        population_size: 1,
        gap: 0.5,
        n_crossovers: 0.5,
        n_mutations: 0.5,
        max_generations: 1,
        fitness_parameters: ReinforcementLearningParameters::new(5, 500, input),
        program_parameters: ProgramGeneratorParameters::new(
            100,
            InstructionGeneratorParameters::from::<CartPoleInput>(1),
        ),
    };

    CartPoleLgp::execute(&mut hyper_params, EventHooks::default())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error;

    use gym_rs::{envs::classical_control::cartpole::CartPoleEnv, utils::renderer::RenderMode};
    use lgp::{
        core::{
            algorithm::{EventHooks, GeneticAlgorithm, HyperParameters},
            instruction::InstructionGeneratorParameters,
            program::ProgramGeneratorParameters,
        },
        extensions::reinforcement_learning::ReinforcementLearningParameters,
        utils::plots::plot_population_benchmarks,
    };

    use crate::set_up::{CartPoleInput, CartPoleLgp};

    #[test]
    fn given_cart_pole_when_lgp_executed_then_task_is_solved() -> Result<(), Box<dyn error::Error>>
    {
        let environment = CartPoleEnv::new(RenderMode::None);
        let input = CartPoleInput::new(environment);

        let mut hyper_params = HyperParameters {
            population_size: 10,
            gap: 0.5,
            n_crossovers: 0.5,
            n_mutations: 0.5,
            max_generations: 100,
            fitness_parameters: ReinforcementLearningParameters::new(5, 500, input),
            program_parameters: ProgramGeneratorParameters::new(
                100,
                InstructionGeneratorParameters::from::<CartPoleInput>(1),
            ),
        };

        let mut populations = vec![];

        CartPoleLgp::execute(
            &mut hyper_params,
            EventHooks::default()
                .with_after_rank(&mut |population| Ok(populations.push(population.clone()))),
        )?;

        const PLOT_FILE_NAME: &'static str = "assets/tests/plots/cart_pole.png";
        let range = (0.)..(hyper_params.fitness_parameters.max_episode_length as f32);
        plot_population_benchmarks(populations, PLOT_FILE_NAME, range)?;
        Ok(())
    }
}

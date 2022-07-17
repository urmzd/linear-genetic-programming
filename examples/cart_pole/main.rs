use gym_rs::{envs::classical_control::cartpole::CartPoleEnv, utils::renderer::RenderMode};
use lgp::{
    core::{
        algorithm::{EventHooks, GeneticAlgorithm, HyperParameters},
        instruction::InstructionGeneratorParameters,
        program::ProgramGeneratorParameters,
        registers::RegisterGeneratorParameters,
    },
    extensions::reinforcement_learning::ReinforcementLearningParameters,
};
use set_up::{CartPoleInput, CartPoleLgp};

mod set_up;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let game = CartPoleEnv::new(RenderMode::Human);
    let input = CartPoleInput::new(game);

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

    CartPoleLgp::execute(&hyper_params, EventHooks::default())?;

    Ok(())
}

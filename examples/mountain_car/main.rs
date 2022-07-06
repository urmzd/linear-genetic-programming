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

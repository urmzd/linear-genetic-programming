use gym_rs::{envs::classical_control::mountain_car::MountainCarEnv, utils::renderer::RenderMode};
use lgp::{
    core::{
        algorithm::{EventHooks, GeneticAlgorithm, HyperParameters},
        instruction::InstructionGeneratorParameters,
        program::ProgramGeneratorParameters,
        registers::RegisterGeneratorParameters,
    },
    extensions::reinforcement_learning::ReinforcementLearningParameters,
    utils::common_traits::ValidInput,
};
use set_up::{MountainCarInput, MountainCarLgp};
use strum::EnumCount;

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
            instruction_generator_parameters: InstructionGeneratorParameters::new(
                <MountainCarInput as ValidInput>::Actions::COUNT,
                <MountainCarInput as ValidInput>::N_INPUTS,
            ),
            register_generator_parameters: RegisterGeneratorParameters::new(3),
            other: ReinforcementLearningParameters::new(5, 200, input),
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
            program::{Program, ProgramGeneratorParameters},
            registers::RegisterGeneratorParameters,
        },
        extensions::reinforcement_learning::ReinforcementLearningParameters,
        utils::{common_traits::ValidInput, plots::plot_population_benchmarks},
    };
    use ndarray::{s, Array2};

    use crate::set_up::{MountainCarInput, MountainCarLgp};
    use strum::EnumCount;

    #[tokio::test]
    async fn run_test() -> Result<(), Box<dyn std::error::Error>> {
        MountainCarLgp::init_env();

        let game = MountainCarEnv::new(RenderMode::None, None);
        let input = MountainCarInput::new(game);

        let hyper_params: HyperParameters<
            Program<ReinforcementLearningParameters<MountainCarInput>>,
        > = HyperParameters {
            population_size: 10,
            max_generations: 10,
            program_params: ProgramGeneratorParameters {
                max_instructions: 100,
                register_generator_parameters: RegisterGeneratorParameters::new(1),
                other: ReinforcementLearningParameters::new(5, 200, input),
                instruction_generator_parameters: InstructionGeneratorParameters::new(
                    <MountainCarInput as ValidInput>::Actions::COUNT,
                    <MountainCarInput as ValidInput>::N_INPUTS,
                ),
            },
            gap: 0.5,
            n_mutations: 0.5,
            n_crossovers: 0.5,
        };

        let mut uninit_populations =
            Array2::uninit((hyper_params.max_generations, hyper_params.population_size));
        let mut generations: usize = 0;

        MountainCarLgp::execute(
            &hyper_params,
            EventHooks::default().with_after_rank(&mut |population| {
                let array = population.clone().into_ndarray();
                array.assign_to(uninit_populations.slice_mut(s![generations, ..]));
                generations += 1;

                Ok(())
            }),
        )?;

        let init_populations = unsafe { uninit_populations.assume_init() };

        const PLOT_FILE_NAME: &'static str = "./assets/tests/plots/mountain_car.png";
        plot_population_benchmarks(init_populations, PLOT_FILE_NAME, -200f32..0f32)?;
        Ok(())
    }
}

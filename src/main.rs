use std::error;

use lgp::{
    data::iris::{
        data::{IrisInput, IrisLgp},
        ops::IRIS_EXECUTABLES,
        utils::{get_iris_content, ContentFilePair},
    },
    genes::{
        algorithm::{EventHooks, GeneticAlgorithm, HyperParameters, Loader},
        program::{Program, ProgramGenerateParams},
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let ContentFilePair(_, file) = get_iris_content().await?;
    let inputs = IrisLgp::load_inputs(file.path());

    let hyper_params: HyperParameters<Program<IrisInput>> = HyperParameters {
        population_size: 1000,
        gap: 0.5,
        n_crossovers: 0.5,
        n_mutations: 0.5,
        max_generations: 5,
        program_params: ProgramGenerateParams::new(&inputs, 100, IRIS_EXECUTABLES, None),
    };

    IrisLgp::execute(
        &hyper_params,
        EventHooks {
            after_init: &mut |_| Ok(()),
            after_evaluate: &mut |_| Ok(()),
            after_selection: &mut |_| Ok(()),
            after_rank: &mut |_| Ok(()),
            after_breed: &mut |_| Ok(()),
        },
    )?;
    Ok(())
}

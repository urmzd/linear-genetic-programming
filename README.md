# Linear Genetic Programming

A framework for implementing algorithms involving Linear Genetic Programming.

![build passing](https://github.com/urmzd/linear-genetic-programming/actions/workflows/develop.yml/badge.svg)

## Modules

-   [Core](src/core/)
-   [Measurement Tools](src/measure/)
-   [Examples](src/examples/)
-   [Extension](src/extensions/)
-   [Utilities](src/utils/)

## Examples

### Classification (Iris)

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let ContentFilePair(_, file) = get_iris_content().await?;
    let inputs = IrisLgp::load_inputs(file.path());


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
        
    IrisLgp::execute(&hyper_params, EventHooks::default())?;
    Ok(())
}
```

### Reinforcement Learning (Mountain Car)

```rust
fn main() {
    todo!()
}
```

## Building

Requirements:

-   Cargo
-   Stable Rust

## Contributions

Contributions are welcomed. The guidelines can be found in [CONTRIBUTING.md](./CONTRIBUTING.md).

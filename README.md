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

    let hyper_params: HyperParameters<Program<Classification<IrisInput>>> = HyperParameters {
        population_size: 100,
        gap: 0.5,
        max_generations: 100,
        program_params: ProgramGeneratorParameters::new(
            100,
            InstructionGeneratorParameters::new(
                IrisInput::N_OUTPUTS + 1,
                Some(IrisInput::N_INPUTS),
                Modes::all(),
                IRIS_EXECUTABLES,
            ),
            Classification::new(&inputs),
        ),
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

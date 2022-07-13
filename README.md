# Linear Genetic Programming

A framework for implementing algorithms involving Linear Genetic Programming.

![build passing](https://github.com/urmzd/linear-genetic-programming/actions/workflows/develop.yml/badge.svg)

## Modules

-   [Core](src/core/)
-   [Measurement Tools](src/measure/)
-   [Extension](src/extensions/)
-   [Utilities](src/utils/)

## Examples

All examples can be built and ran through Cargo:

```bash
cargo build --example <example_name>
cargo run --example <example_name>
```

### Classification (iris)

```rust
//examples/iris/main.rs#L19-L44

async fn main() -> Result<(), Box<dyn error::Error>> {
    let ContentFilePair(_, file) = get_iris_content().await?;
    let inputs = IrisLgp::load_inputs(file.path());

    let hyper_params = HyperParameters {
        population_size: 100,
        max_generations: 100,
        program_params: ProgramGeneratorParameters {
            max_instructions: 100,
            register_generator_parameters: RegisterGeneratorParameters::new(1),
            other: ClassificationParameters::new(&inputs),
            instruction_generator_parameters: InstructionGeneratorParameters::new(
                <IrisInput as ValidInput>::Actions::COUNT,
                <IrisInput as ValidInput>::N_INPUTS,
            ),
        },
        gap: 0.5,
        n_mutations: 0.5,
        n_crossovers: 0.5,
    };

    let mut x = vec![];
    let hooks = EventHooks::default().with_after_rank(&mut |mut p| {
        x.push(p.into_ndarray());
        Ok(())
    });
```

### Reinforcement Learning (mountain_car)

```rust
//examples/mountain_car/main.rs#L15-L36

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
            other: ReinforcementLearningParameters::new(5, input),
        },
```

## Building

Requirements:

-   Cargo
-   Stable Rust

## Contributions

Contributions are welcomed. The guidelines can be found in [CONTRIBUTING.md](./CONTRIBUTING.md).

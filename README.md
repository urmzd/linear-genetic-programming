# Linear Genetic Programming

A framework for implementing algorithms involving Linear Genetic Programming.

![build passing](https://github.com/urmzd/linear-genetic-programming/actions/workflows/develop.yml/badge.svg)

## Modules

- [Core](src/core/)
- [Extension](src/extensions/)
- [Utilities](src/utils/)

## Examples

All examples can be built and ran through Cargo:

```bash
cargo build --example <example_name>
cargo run --example <example_name>
```

### Classification

#### iris

```rust
//examples/iris/main.rs#L16-L37

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let ContentFilePair(_, file) = get_iris_content().await?;
    let inputs = IrisLgp::load_inputs(file.path());

    let hyper_params = HyperParameters {
        population_size: 100,
        max_generations: 100,
        gap: 0.5,
        n_mutations: 0.5,
        n_crossovers: 0.5,
        program_params: ProgramGeneratorParameters::new(
            100,
            InstructionGeneratorParameters::from(1),
            RegisterGeneratorParameters::new(1),
            ClassificationParameters::new(&inputs),
        ),
    };

    IrisLgp::execute(&hyper_params, EventHooks::default())?;
    Ok(())
}
```

### Reinforcement Learning

#### mountain_car

```rust
//examples/mountain_car/main.rs#L15-L36

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
```

#### cart_pole

```rust
//examples/cart_pole/main.rs#L15-L36

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
```

## Building

Requirements:

- Cargo
- Stable Rust

## Contributions

Contributions are welcomed. The guidelines can be found in [CONTRIBUTING.md](./CONTRIBUTING.md).

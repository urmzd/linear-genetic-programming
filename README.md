# Linear Genetic Programming

A framework for solving tasks using linear genetic programming.

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

async fn main() -> Result<(), Box<dyn error::Error>> {
    let ContentFilePair(_, file) = get_iris_content().await?;
    let inputs = IrisLgp::load_inputs(file.path());

    let hyper_params = HyperParameters {
        population_size: 100,
        max_generations: 100,
        gap: 0.5,
        n_mutations: 0.5,
        n_crossovers: 0.5,
        mutate_parameters: (),
        fitness_parameters: ClassificationParameters::new(&inputs),
        program_parameters: ProgramGeneratorParameters::new(
            100,
            InstructionGeneratorParameters::from(1),
        ),
    };

    IrisLgp::execute(&hyper_params, EventHooks::default())?;
    Ok(())
}

```

### Reinforcement Learning

#### mountain_car

```rust
//examples/mountain_car/main.rs#L14-L34

    let environment = MountainCarEnv::new(RenderMode::Human, None);
    let input = MountainCarInput::new(environment);

    let hyper_params = HyperParameters {
        population_size: 1,
        gap: 0.5,
        n_crossovers: 0.5,
        n_mutations: 0.5,
        max_generations: 1,
        mutate_parameters: (),
        fitness_parameters: ReinforcementLearningParameters::new(5, 200, input),
        program_parameters: ProgramGeneratorParameters::new(
            100,
            InstructionGeneratorParameters::from(1),
        ),
    };

    MountainCarLgp::execute(&hyper_params, EventHooks::default())?;

    Ok(())
}

```

#### cart_pole

```rust
//examples/cart_pole/main.rs#L14-L34

    let environment = CartPoleEnv::new(RenderMode::Human);
    let input = CartPoleInput::new(environment);

    let hyper_params = HyperParameters {
        population_size: 1,
        gap: 0.5,
        n_crossovers: 0.5,
        n_mutations: 0.5,
        mutate_parameters: (),
        max_generations: 1,
        fitness_parameters: ReinforcementLearningParameters::new(5, 500, input),
        program_parameters: ProgramGeneratorParameters::new(
            100,
            InstructionGeneratorParameters::from(1),
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

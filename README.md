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

    let mut hyper_params = HyperParameters {
        population_size: 100,
        max_generations: 100,
        gap: 0.5,
        n_mutations: 0.5,
        n_crossovers: 0.5,
        fitness_parameters: ClassificationParameters::new(inputs),
        program_parameters: ProgramGeneratorParameters::new(
            100,
            InstructionGeneratorParameters::from::<IrisInput>(1),
        ),
    };

    IrisLgp::execute(&mut hyper_params, EventHooks::default())?;
    Ok(())
}

#[cfg(test)]
```

### Reinforcement Learning

#### mountain_car

```rust
//examples/mountain_car/main.rs#L15-L36

    let environment = MountainCarEnv::new(RenderMode::Human, None);
    let input = MountainCarInput::new(environment);

    let mut hyper_params = HyperParameters {
        population_size: 1,
        gap: 0.5,
        n_crossovers: 0.5,
        n_mutations: 0.5,
        max_generations: 1,
        fitness_parameters: ReinforcementLearningParameters::new(5, 200, input),
        program_parameters: ProgramGeneratorParameters::new(
            100,
            InstructionGeneratorParameters::from::<MountainCarInput>(1),
        ),
    };

    MountainCarLgp::execute(&mut hyper_params, EventHooks::default())?;

    Ok(())
}

#[cfg(test)]
```

#### cart_pole

```rust
//examples/cart_pole/main.rs#L15-L36

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
```

## Building

Requirements:

- Cargo
- Stable Rust

## Contributions

Contributions are welcomed. The guidelines can be found in [CONTRIBUTING.md](./CONTRIBUTING.md).

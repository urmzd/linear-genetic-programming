# Linear Genetic Programming

A framework for implementing algorithms involving Linear Genetic Programming.

![build passing](https://github.com/urmzd/linear-genetic-programming/actions/workflows/develop.yml/badge.svg)

## Modules

-   [Metrics](src/metrics)
-   [Examples](src/examples)
-   [Genes](src/genes)
-   [Utils](src/utils)

## Examples

### Classification (Iris)

```rust
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

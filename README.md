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
cargo build --bin <example_name>
cargo run --bin <example_name>
```

### Classification (iris)

```rust
//examples/iris/main.rs#L19-L44
```

### Reinforcement Learning (mountain_car)

```rust
//examples/mountain_car/main.rs#L15-L36
```

## Building

Requirements:

-   Cargo
-   Stable Rust

## Contributions

Contributions are welcomed. The guidelines can be found in [CONTRIBUTING.md](./CONTRIBUTING.md).

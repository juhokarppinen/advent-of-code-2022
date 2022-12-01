# Advent of Code 2022 - Rust edition

## What is this all about?

See https://adventofcode.com/2022 for more information.

## Getting started

### Clone this repository

```shell
git clone git@github.com:juhokarppinen/advent-of-code-2022.git
```

### Install the Rust toolchain

See https://www.rust-lang.org/tools/install installation instructions for your platform.

### Install dependencies, build and run in development mode

```shell
cargo run
```

## Development

Each day's solution resides within a separate module. The file `./src/day_00.rs` can be used as a template for new solutions. The modules need to be declared in the `main` module and included within the `solvers` array inside the `main` function.

Input data is unique for each user and this excluded from the repository. The input files are expected to be located within `./input/nn.txt`, where `nn` corresponds to a zero padded day number, e.g. `./input/07.txt`.

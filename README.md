# advent-of-code-rust-starter-pack
Basic setup for solving Advent of Code puzzles using Rust

## Requirements
- Rust 1.75
```shell
$ rustc --version
rustc 1.75.0 (82e1608df 2023-12-21)
```
- `rustfmt` and `clippy` for code quality

## Quickstart

### Input
Place your own input files in the input folder as follows:
- input/2015/1.in
- input/2023/25.in

### Build
```shell
make build
```

### Run
```shell
make run year=2015 day=1
```

### Run all tests
```shell
make test
```

### Makefile
```shell
$ make     

==============
Advent of Code
==============
Usage: 

  help          Prints this help message
  build-debug   Build the local package and all of its dependencies
  run-debug     Build and run the current package
  build         Build the local package and all of its dependencies with optimizations (release mode)
  run           Build and run the current optimized package
  update        Update dependencies listed in Cargo.lock
  check         Analyze the current package and report errors, but don't build object files
  clean         Clean the current package and all build artifacts
  fmt           Format all Rust files of the current crate
  test          Run the tests
  clippy        Run cargo clippy for static ckecks
```

## TODOs:
[ ] Input downloading
[ ] Benchmarking

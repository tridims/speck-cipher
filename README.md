# Implementation of SPECK cipher

based on the paper: https://eprint.iacr.org/2013/404.pdf

# Usage

To run the program, you need rust and cargo installed. Then you can run the following command:

```bash
cargo run
```

## To install rust and cargo

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## To run the tests

```bash
cargo test
```

## To run the benchmarks

```bash
cargo bench
```

## To run the statistical tests, you need to run the example program.

```bash
cargo run --example <example_name>
```

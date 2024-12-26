# Hash Finder

Rust Developer Intern / Task 3 / Asqar Arslanov

## About

This CLI program outputs SHA-256 hashes of positive integer numbers if their
hash ends with `N` zeros. It utilizes multithreading to improve performance.

## How to run it?

Build the program in release mode.

```shell
cargo build --release
```

Run the compiled binary with the following options:

- `-N`: number of zeros hashes should end with;
- `-F`: number of results to output.

```shell
target/release/hash_finder -N 3 -F 6
```

Or simply launch the program with Cargo.

```shell
cargo run --release -- -N 3 -F 6
```

By default, these commands will launch the version that doesn&CloseCurlyQuote;t
rely on external crates.

To run the other version, enable the `ecosystem` feature of the crate by
modifying [`Cargo.toml`](/Cargo.toml) or through CLI.

```shell
cargo run --release --features ecosystem -- -N 3 -F 6
```

This will make use of the [`rayon`](https://crates.io/crates/rayon) and
[`sha256`](https://crates.io/crates/sha256) crates.

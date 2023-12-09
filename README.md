# Advent of Code project template

A Rust template made to easily run any day or combination of days and measure the execution time. Credit goes to [agubelu](https://github.com/agubelu/AoC-rust-template), although I have modified it heavily.

Each day has a `solve()` function that returns a pair of `Solution`. The type `Solution` is an enum that can contain any integer or a string.

To run: `cargo run --release [days...]`

## Downloading Inputs

Get a session cookie from your browser and set it as the `COOKIE` environment variable.

```sh
export COOKIE=your_key
```

Then run `./get-input.sh 1`.

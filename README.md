# Advent of Code project template

A Rust template for Advent of Code that *someone on Reddit*(sorry, I forgot who it was) made to easily run any day or combination of days and measure the execution time.

Each day has a `solve()` function that returns a pair of `Solution`. The type `Solution` is an enum that can contain any integer or a string.

To run: `cargo run --release [days...]`

## Downloading Inputs

Get a session cookie from your browser and copy it into `cookie.key`. Then run `./get-input.sh day_you_want_to_download`.

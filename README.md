# CC2 random calc

This is a simple Rust CLI which runs the Chip's Challenge 2 PRNG generator to find certain number patterns.

## Usage example

```sh
cargo run -- -n -p 2,1
```

The above command runs the `n`atural search looking for the pattern of `[2, 1]`.

## Search Types

A `n`atural search goes from the default state of `0`/`0` and calls it until it hits the maximum theoretical repeat period of 65536.

An `a`ll search goes through `0`/`0` all the way to `255`/`255`, checking all possible seeds, even the ones which can't be gotten in practice.

## Details

All randomness results are modulo'd by 4, to help with the most common case of calculating direction offsets for Walker and Blobs.

This currently only emulates the basic PRNG, so blob PRNG is not yet emulated.

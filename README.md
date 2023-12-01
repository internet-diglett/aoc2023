# aoc2023

A cli puzzle solver for Advent of Code 2023

## How to run

To just run the thing:

```sh
$ cargo run --package=bin -- -d 1 -i day1.txt
```

To build a release version for *blazingly fast puzzle solving*:

``` sh
cargo build --release
./target/release/bin -d 1 -i day1.txt
```

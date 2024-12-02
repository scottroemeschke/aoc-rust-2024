<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code {year}

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

<!--- advent_readme_stars table --->

<!--- benchmarking table --->

## Benchmarks

|           Day            |  Part 1   |  Part 2   |
|:------------------------:|:---------:|:---------:|
| [Day 1](./src/bin/01.rs) | `53.5µs`  | `79.7µs`  |
| [Day 2](./src/bin/02.rs) | `185.1µs` | `349.2µs` |

**Total: 0.67ms**
<!--- benchmarking table --->

---

> This is forked via the excellent [fspoettel/advent-of-code-rust](https://github.com/fspoettel/advent-of-code-rust?tab=readme-ov-file).

## Usage

### ➡️ Run solutions for a day

```sh
# example: `cargo solve 01`
cargo solve <day>

# output:
#     Finished dev [unoptimized + debuginfo] target(s) in 0.13s
#     Running `target/debug/01`
# Part 1: 42 (166.0ns)
# Part 2: 42 (41.0ns)
```

The `solve` command runs your solution against real puzzle inputs. To run an optimized build of your code, append the
`--release` flag as with any other rust program.

### ➡️ Benchmark your solutions

```sh
# example: `cargo time 8 --store`
cargo time <day> [--all] [--store]

# output:
# Day 08
# ------
# Part 1: 1 (39.0ns @ 10000 samples)
# Part 2: 2 (39.0ns @ 10000 samples)
#
# Total (Run): 0.00ms
#
# Stored updated benchmarks.
```

The `cargo time` command allows you to benchmark your code and store timings in the readme. When benching, the runner
will run your code between `10` and `10.000` times, depending on execution time of first execution, and print the
average execution time.

`cargo time` has three modes of execution:

1. `cargo time` without arguments incrementally benches solutions that do not have been stored in the readme yet and
   skips the rest.
2. `cargo time <day>` benches a single solution.
3. `cargo time --all` benches all solutions.

By default, `cargo time` does not write to the readme. In order to do so, append the `--store` flag:
`cargo time --store`.

> Please note that these are not _scientific_ benchmarks, understand them as a fun approximation. 😉 Timings, especially
> in the microseconds range, might change a bit between invocations.

### ➡️ Run all tests

```sh
cargo test
```

To run tests for a specific day, append `--bin <day>`, e.g. `cargo test --bin 01`. You can further scope it down to a
specific part, e.g. `cargo test --bin 01 part_one`.

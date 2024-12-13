<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code 2024

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/)
> This is forked via the excellent [fspoettel/advent-of-code-rust](https://github.com/fspoettel/advent-of-code-rust?tab=readme-ov-file).

<!--- advent_readme_stars table --->

<!--- benchmarking table --->
## Benchmarks

| Day | Part 1 | Part 2 |
| :---: | :---: | :---:  |
| [Day 1](./src/bin/01.rs) | `103.1µs` | `184.0µs` |
| [Day 2](./src/bin/02.rs) | `183.7µs` | `247.5µs` |
| [Day 3](./src/bin/03.rs) | `30.9µs` | `31.1µs` |
| [Day 4](./src/bin/04.rs) | `396.9µs` | `386.9µs` |
| [Day 5](./src/bin/05.rs) | `323.7µs` | `825.0µs` |
| [Day 6](./src/bin/06.rs) | `745.3µs` | `928.5ms` |
| [Day 7](./src/bin/07.rs) | `3.3ms` | `1.3s` |
| [Day 8](./src/bin/08.rs) | `120.4µs` | `249.6µs` |
| [Day 9](./src/bin/09.rs) | `1.1ms` | `165.2ms` |

**Total: 2401.93ms**
<!--- benchmarking table --->

---

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

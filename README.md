# Yandex Practicum Optimization

A Rust learning project for the Yandex Practicum course focused on performance profiling, optimization, and memory safety analysis.

## Overview

The project (`broken-app`) provides a set of functions used as optimization exercises. Each function is analyzed with profiling tools, benchmarked before and after optimization, and verified with memory safety checkers.

## Project Structure

```
src/
  lib.rs            # sum_even, leak_buffer, normalize, average_positive, use_after_free
  algo.rs           # slow_dedup, slow_fib
  concurrency.rs    # race_increment, read_after_sleep, reset_counter
  bin/demo.rs       # Demo binary
benches/
  criterion.rs      # Criterion benchmark suite
  baseline.rs       # Manual timing harness
tests/
  integration.rs    # Integration tests
scripts/
  compare.sh        # Run before/after benchmark comparison
  profile.sh        # Profile with Linux perf
artifacts/
  flamegraphs/      # Flamegraph SVGs (baseline, after each optimization)
  benchmarks/       # Criterion HTML reports (baseline, after each optimization)
  test_result.txt   # cargo test output
  miri_result.txt   # Miri UB detection output
  sanitizer_result.txt  # AddressSanitizer / ThreadSanitizer output
  valgrind_result.txt   # Valgrind memory analysis output
```

## Functions

| Function | Module | Description |
|---|---|---|
| `sum_even` | `lib` | Sum of even elements with overflow protection |
| `leak_buffer` | `lib` | Count of non-zero bytes in a buffer |
| `normalize` | `lib` | Remove spaces and lowercase a string |
| `average_positive` | `lib` | Mean of positive elements |
| `use_after_free` | `lib` | Returns `42 * 2 = 84` |
| `slow_dedup` | `algo` | Deduplicate and sort values via `BTreeSet` |
| `slow_fib` | `algo` | Iterative Fibonacci in O(n) |
| `race_increment` | `concurrency` | Atomic multi-threaded counter increment |

## Running

```bash
# Tests
cargo test

# Benchmarks
cargo bench --bench criterion
```

## Memory Safety Checks

```bash
# Miri (undefined behavior detection)
cargo +nightly miri test

# AddressSanitizer
RUSTFLAGS="-Zsanitizer=address" cargo +nightly test --target x86_64-unknown-linux-gnu

# ThreadSanitizer
RUSTFLAGS="-Z sanitizer=thread"  RUSTDOCFLAGS="-Zsanitizer=thread" cargo +nightly -Z build-std test --target x86_64-unknown-linux-gnu

# Valgrind
valgrind --leak-check=full --show-leak-kinds=all ./target/release/demo
```

All checks pass: no undefined behavior, no memory leaks, no data races.

## Optimization Results

Three optimization iterations are documented in `artifacts/`, each with a flamegraph and Criterion report:

1. **Baseline** — initial measurements
2. **After `slow_fib` optimization** — iterative implementation replaces naive recursion
3. **After `slow_dedup` optimization** — `BTreeSet`-based deduplication replaces the previous approach

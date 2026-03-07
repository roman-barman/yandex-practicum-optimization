# Baseline Flamegraph Analysis

**Binary:** `demo`
**Total samples:** 1,615,116 (100%)
**Tool:** `perf` + `flamegraph` (flamegraph.svg)

## Application Functions

The `demo` binary calls the following application functions from `broken_app`:

| Function | Location | Samples | % |
|---|---|---|---|
| `algo::slow_fib` | `src/algo.rs:22` | 65,920 | 4.08% |
| `algo::slow_dedup` | `src/algo.rs:2` | — | not sampled |
| `sum_even` | `src/lib.rs:7` | — | not sampled |
| `leak_buffer` | `src/lib.rs:28` | — | not sampled |
| `normalize` | `src/lib.rs:34` | — | not sampled |

`slow_dedup`, `sum_even`, `leak_buffer`, and `normalize` complete too quickly on the given inputs for `perf` to capture samples — they do not appear in the flamegraph.

## Hotspot: `algo::slow_fib` — 4.08% (65,920 samples)

**Location:** `src/algo.rs:22`

The only application function visible in the profile. Implements Fibonacci using naive double recursion without memoization:

```rust
pub fn slow_fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => slow_fib(n - 1) + slow_fib(n - 2),
    }
}
```

This has **O(2^n)** time complexity. Called with `n=20`, it expands to ~21,891 recursive calls.

**Fix direction:** Replace with iterative computation or memoization to reduce complexity to O(n).

## Note on Other Application Functions

The remaining functions called in `demo::main` run in negligible time for their inputs:

- **`algo::slow_dedup`** — O(n²) algorithm with a sort on each insert, but input is only 7 elements so it completes instantly.
- **`sum_even`** — simple linear scan over 4 elements.
- **`leak_buffer`** — single iterator pass over 4 bytes.
- **`normalize`** — string replace + lowercase on a short string.

To expose these functions in a future flamegraph, run them on larger inputs or in a loop.

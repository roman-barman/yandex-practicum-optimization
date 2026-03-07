# After `slow_fib` Optimization — Flamegraph Analysis

**Binary:** `demo`
**Total samples:** 1,695,184 (100%)
**Tool:** `perf` + `flamegraph` (flamegraph.svg)

## Application Functions

| Function | Location | Samples | % |
|---|---|---|---|
| `algo::slow_fib` | `src/algo.rs:22` | — | not sampled |
| `algo::slow_dedup` | `src/algo.rs:2` | — | not sampled |
| `sum_even` | `src/lib.rs:7` | — | not sampled |
| `leak_buffer` | `src/lib.rs:28` | — | not sampled |
| `normalize` | `src/lib.rs:34` | — | not sampled |

No application functions appear in the flamegraph — all complete too quickly to be sampled.

## Performance Changes vs Baseline

| Metric | Baseline | After optimization | Change |
|---|---|---|---|
| `algo::slow_fib` samples | 65,920 (4.08%) | 0 (0%) | **-100%** |
| `demo::main` subtree samples | 131,498 (8.14%) | 63,225 (3.73%) | **-52%** |

### `algo::slow_fib` eliminated

`slow_fib` no longer appears in the profile at all. The previous O(2^n) recursive implementation made ~21,891 calls for `n=20`. After optimization the function completes in negligible time, producing zero samples.

### `demo::main` subtree halved

In the baseline, `demo::main`'s subtree split between `slow_fib` (4.08%) and stdout I/O (4.06%). After removing `slow_fib`, the subtree shrinks by ~52% and is now entirely composed of `write_fmt` — stdout output is the only remaining work done in `main`.

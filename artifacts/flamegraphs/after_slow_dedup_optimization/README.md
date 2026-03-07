# After `slow_dedup` Optimization — Flamegraph Analysis

**Binary:** `demo`
**Total samples:** 2,115,718 (100%)
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

## Performance Changes vs Previous Step

| Metric | After `slow_fib` opt | After `slow_dedup` opt | Change |
|---|---|---|---|
| `algo::slow_dedup` samples | 0 (0%) | 0 (0%) | no change |
| `demo::main` subtree samples | 63,225 (3.73%) | 65,641 (3.10%) | within noise |

### `slow_dedup` not visible in the flamegraph

`slow_dedup` was not sampled in any of the profiles — neither in the baseline nor after the `slow_fib` optimization — because `demo::main` calls it with only 7 elements (`[1, 2, 2, 3, 1, 4, 4]`). Even the O(n²) implementation with a sort on every insert completes in nanoseconds on such a small input, making it impossible for `perf` to capture samples.

The optimization is correct and improves algorithmic complexity, but its impact cannot be observed in this flamegraph. To make it visible, `slow_dedup` would need to be called with a significantly larger input (thousands of elements or more) in a loop.

### `demo::main` subtree unchanged

The `demo::main` subtree holds steady at ~65k samples (~3.1–3.7% across runs). The small variation between profiles is measurement noise from `perf` sampling, not a real performance difference. The subtree is entirely composed of stdout I/O (`__libc_write`) — the only remaining work in `main` after both algorithmic optimizations.

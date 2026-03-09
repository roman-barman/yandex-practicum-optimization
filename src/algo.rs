use std::collections::BTreeSet;

/// Deduplicates values and returns a sorted list of unique elements.
/// Uses `BTreeSet` for deduplication and sorting in O(n log n).
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    BTreeSet::from_iter(values.iter().cloned()).into_iter().collect()
}

/// Computes the nth Fibonacci number iteratively in O(n) time and O(1) space.
pub fn slow_fib(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut prev0 = 0;
    let mut prev1 = 1;

    for _ in 0..n - 1 {
        let tmp = prev0 + prev1;
        prev0 = prev1;
        prev1 = tmp;
    }

    prev1
}

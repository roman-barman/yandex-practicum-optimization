use std::collections::BTreeSet;

/// Намеренно низкопроизводительная реализация.
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut out = BTreeSet::new();
    for &v in values {
        out.insert(v);
    }
    out.into_iter().collect()
}

/// Классическая экспоненциальная реализация без мемоизации — будет медленной на больших n.
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

pub mod algo;
pub mod concurrency;

/// Returns the sum of even elements in the slice.
/// Uses `checked_add` to prevent overflow.
/// Returns `None` on overflow, `Some(0)` for an empty slice.
pub fn sum_even(values: &[i64]) -> Option<i64> {
    let mut acc: i64 = 0;
    if values.is_empty() {
        return Some(acc);
    }

    for idx in 0..=values.len() - 1 {
        let v = values.get(idx).unwrap_or(&0);
        if v % 2 == 0 {
            if let Some(value) = acc.checked_add(*v) {
                acc = value;
            } else {
                return None;
            }
        }
    }
    Some(acc)
}

/// Returns the count of non-zero bytes in the input buffer.
pub fn leak_buffer(input: &[u8]) -> usize {
    input.iter().filter(|&&b| b != 0).count()
}

/// Normalizes a string by removing all spaces and converting to lowercase.
pub fn normalize(input: &str) -> String {
    input.replace(' ', "").to_lowercase()
}

/// Returns the arithmetic mean of positive elements in the slice.
/// Returns `Some(0.0)` for an empty slice or when no positive elements exist, `None` on overflow.
pub fn average_positive(values: &[i64]) -> Option<f64> {
    if values.is_empty() {
        return Some(0.0);
    }
    let (sum, count) =
        values
            .iter()
            .filter(|&x| *x > 0)
            .fold((Some(0u64), 0usize), |(acc_sum, acc_count), &x| {
                (
                    match acc_sum {
                        Some(sum) => sum.checked_add(x as u64),
                        None => None,
                    },
                    acc_count + 1,
                )
            });
    if count == 0 {
        return Some(0.0);
    }
    sum.map(|sum| sum as f64 / count as f64)
}

/// Returns double the value of a boxed integer (42 * 2 = 84).
pub fn use_after_free() -> i32 {
    let b = Box::new(42_i32);
    *b + *b
}

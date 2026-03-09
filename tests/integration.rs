use broken_app::{algo, leak_buffer, normalize, sum_even};

#[test]
fn sums_even_numbers() {
    let nums: [i64; 0] = [];
    // Expected sum: 0.
    assert_eq!(sum_even(&nums), Some(0));

    let nums = [1, 2, 3, 4];
    // Expected sum: 2 + 4 = 6.
    assert_eq!(sum_even(&nums), Some(6));

    let nums = [1, 3];
    // Expected sum: 0 (no even numbers).
    assert_eq!(sum_even(&nums), Some(0));

    let nums = [2];
    // Expected sum: 2.
    assert_eq!(sum_even(&nums), Some(2));

    let nums = [2, 4];
    // Expected sum: 2 + 4 = 6.
    assert_eq!(sum_even(&nums), Some(6));

    let nums = [i64::MAX - 1, 2];
    // Expected overflow: None.
    assert_eq!(sum_even(&nums), None);
}

#[test]
fn counts_non_zero_bytes() {
    let data = [0_u8, 1, 0, 2, 3];
    assert_eq!(leak_buffer(&data), 3);
}

#[test]
fn dedup_preserves_uniques() {
    let uniq = algo::slow_dedup(&[5, 5, 1, 2, 2, 3]);
    assert_eq!(uniq, vec![1, 2, 3, 5]); // order and contents matter
}

#[test]
fn fib_small_numbers() {
    assert_eq!(algo::slow_fib(10), 55);
}

#[test]
fn normalize_simple() {
    assert_eq!(normalize(" Hello World "), "helloworld");
}

#[test]
fn averages_only_positive() {
    let nums = [-5, 5, 15];
    // Expected: (5 + 15) / 2 = 10
    assert_eq!(broken_app::average_positive(&nums), Some(10.0));

    let nums = [-5];
    // Expected: 0 (no positive elements)
    assert_eq!(broken_app::average_positive(&nums), Some(0.0));

    let nums = [
        i64::MAX,
        i64::MAX,
        i64::MAX,
        i64::MAX,
        i64::MAX,
        i64::MAX,
        i64::MAX,
        i64::MAX,
    ];
    // Expected: None (overflow)
    assert_eq!(broken_app::average_positive(&nums), None);
}

#[test]
fn use_after_free() {
    assert_eq!(broken_app::use_after_free(), 84);
}

#[test]
fn race_increment() {
    assert_eq!(broken_app::concurrency::race_increment(10, 10), 100);
}

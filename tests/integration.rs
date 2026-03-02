use broken_app::{algo, leak_buffer, normalize, sum_even};

#[test]
fn sums_even_numbers() {
    let nums: [i64; 0] = [];
    // Ожидаем корректное суммирование: 0.
    assert_eq!(sum_even(&nums), Some(0));

    let nums = [1, 2, 3, 4];
    // Ожидаем корректное суммирование: 2 + 4 = 6.
    assert_eq!(sum_even(&nums), Some(6));

    let nums = [1, 3];
    // Ожидаем корректное суммирование: 0.
    assert_eq!(sum_even(&nums), Some(0));

    let nums = [2];
    // Ожидаем корректное суммирование: 2 = 2.
    assert_eq!(sum_even(&nums), Some(2));

    let nums = [2, 4];
    // Ожидаем корректное суммирование: 2 + 4 = 6.
    assert_eq!(sum_even(&nums), Some(6));

    let nums = [i64::MAX - 1, 2];
    // Ожидаем переполнение: None.
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
    assert_eq!(uniq, vec![1, 2, 3, 5]); // порядок и состав важны
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
    // Ожидается (5 + 15) / 2 = 10, но текущая реализация делит на все элементы.
    assert!((broken_app::average_positive(&nums) - 10.0).abs() < f64::EPSILON);
}

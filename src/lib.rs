pub mod algo;
pub mod concurrency;

/// Сумма чётных значений.
/// Здесь намеренно используется `get_unchecked` с off-by-one,
/// из-за чего возникает UB при доступе за пределы среза.
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

/// Подсчёт ненулевых байтов. Буфер намеренно не освобождается,
/// что приведёт к утечке памяти (Valgrind это покажет).
pub fn leak_buffer(input: &[u8]) -> usize {
    input.iter().filter(|&&b| b != 0).count()
}

/// Небрежная нормализация строки: удаляем пробелы и приводим к нижнему регистру,
/// но игнорируем повторяющиеся пробелы/табуляции внутри текста.
pub fn normalize(input: &str) -> String {
    input.replace(' ', "").to_lowercase()
}

/// Логическая ошибка: усредняет по всем элементам, хотя требуется учитывать
/// только положительные. Деление на длину среза даёт неверный результат.
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
    sum.map(|sum| sum as f64 / count as f64)
}

/// Use-after-free: возвращает значение после освобождения бокса.
/// UB, проявится под ASan/Miri.
pub fn use_after_free() -> i32 {
    let b = Box::new(42_i32);
    *b + *b
}

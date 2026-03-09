use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

static COUNTER: AtomicU64 = AtomicU64::new(0);

/// Atomically increments a global counter from multiple threads.
/// Each of the `threads` threads performs `iterations` increments using `SeqCst` ordering.
/// Returns the final counter value.
pub fn race_increment(iterations: usize, threads: usize) -> u64 {
    COUNTER.store(0, Ordering::SeqCst);
    let mut handles = Vec::with_capacity(threads);
    for _ in 0..threads {
        handles.push(thread::spawn(move || {
            for _ in 0..iterations {
                COUNTER.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }
    for h in handles {
        let _ = h.join();
    }
    COUNTER.load(Ordering::SeqCst)
}

/// Returns the current value of the global counter with `SeqCst` ordering.
pub fn read_after_sleep() -> u64 {
    COUNTER.load(Ordering::SeqCst)
}

/// Resets the global counter to zero with `SeqCst` ordering.
pub fn reset_counter() {
    COUNTER.store(0, Ordering::SeqCst);
}

use std::sync::atomic::{AtomicU64, Ordering};

// AtomicU64 is an atomic 64-bit unsigned integer type in Rust that provides thread-safe operations without requiring explicit locking mechanisms like mutexes.
// It's part of Rust's atomic types family in std::sync::atomic.
pub fn atomic_main() {
    // Create an atomic counter
    let counter = AtomicU64::new(0);

    // Load the current value
    let value = counter.load(Ordering::SeqCst);
    println!("Current value: {}", value);

    // Store a new value
    counter.store(42, Ordering::SeqCst);

    // Atomic increment
    let old_value = counter.fetch_add(1, Ordering::SeqCst);
    println!("Old value: {}, New value: {}", old_value, old_value + 1);
}
use std::sync::atomic::{AtomicU64, Ordering};

// AtomicU64 is an atomic 64-bit unsigned integer type in Rust that provides thread-safe operations without requiring explicit locking mechanisms like mutexes.
// It's part of Rust's atomic types family in std::sync::atomic.
pub fn atomic_main() {
    basic_example();
    atomic_timestamp_example();
}

pub fn basic_example() {
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

pub fn atomic_timestamp_example() {
    use time_library::Timestamp;

    // let ts = time_library::Timestamp::now()?;
    // let json = serde_json::to_string(&ts)?;
    // println!("Current timestamp: {}", json);

    // for i in 1..=6 {
    //     match Timestamp::now() {
    //         Ok(timestamp) => println!("Timestamp {}: {:?}", i, timestamp),
    //         Err(e) => eprintln!("Error getting timestamp {}: {}", i, e),
    //     }
    // }

    // Attempt to get the current timestamp
    match Timestamp::now() {
        Ok(timestamp) => println!("Current timestamp: {:?}", timestamp),
        Err(e) => eprintln!("Error getting timestamp: {}", e),
    }
}

mod time_library {
    use std::sync::atomic::{AtomicU64, Ordering};

    use serde::Serialize;

    // #[derive(Serialize, Clone)]
    #[derive(Serialize, Clone, Debug)] 
    pub struct Timestamp(u64);

    impl Timestamp {
        pub fn now() -> Result<Self, Error> {
            static COUNTER: AtomicU64 = AtomicU64::new(0);

            if COUNTER.fetch_add(1, Ordering::SeqCst) % 3 == 0 {
                Err(Error::FailedToGetTime)
            } else {
                Ok(Self(1337))
            }
        }
    }

    #[derive(Debug)]
    pub enum Error {
        FailedToGetTime,
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "failed to get time")
        }
    }
}
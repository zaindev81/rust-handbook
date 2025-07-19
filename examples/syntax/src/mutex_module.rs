use std::sync::{Arc, Mutex};
use std::thread;

// Mutex<T> (Mutual Exclusion) is Rust's primary synchronization primitive
// that provides thread-safe access to shared data. It ensures that only one thread can access the protected data at a time, preventing data races.

pub fn mutex_main() {
    mutex_basic();
    mutex_sharing();
}

pub fn mutex_basic() {
    // Create a mutex to protect shared data
    let mutex = Mutex::new(0);

    // Lock the mutex to access the data
    {
        let mut data = mutex.lock().unwrap();
        *data += 1; // Increment the value
        println!("Incremented value: {}", *data);
    } // Mutex is automatically unlocked here

    // Lock again to read the value
    {
        let data = mutex.lock().unwrap();
        println!("Current value: {}", *data);
    }
}

pub fn mutex_sharing() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

   println!("Result: {}", *counter.lock().unwrap()); // Result: 10
}
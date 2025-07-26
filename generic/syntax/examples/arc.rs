use std::sync::Arc;
use std::thread;

fn main() {
    basic_operations();
    sharing_data_operations();
    arc_with_mutex_for_mutable_shared_state_operations();
    arc_with_rwlock_for_read_heavy_operations();
}

fn basic_operations() {
    println!("Basic Arc operations==================");
    // Create an Arc (Atomic Reference Counted) pointer
    let arc_value = Arc::new(42);

    // Clone the Arc to create a new reference
    let arc_clone = Arc::clone(&arc_value);

    // Use the value inside the Arc
    println!("Original value: {}", arc_value);
    println!("Cloned value: {}", arc_clone);
}

fn sharing_data_operations() {
    println!("\n");
    println!("Sharing data across threads using Arc==================");
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];

    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("Thread {}: {:?}", i, data_clone);
            // Do some work with the shared data
            let sum: i32 = data_clone.iter().sum();
            println!("Thread {}: Sum is {}", i, sum);
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Data after threads: {:?}", data);
}

fn arc_with_mutex_for_mutable_shared_state_operations() {
    println!("\n");
    println!("Using Arc with Mutex for mutable shared state==================");
}

fn arc_with_rwlock_for_read_heavy_operations() {
    println!("\n");
    println!("Using Arc with RwLock for read-heavy operations==================");
}
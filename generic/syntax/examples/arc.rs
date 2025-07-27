use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

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

// Since Arc only provides shared immutable access, combine it with Mutex for mutable shared state:
fn arc_with_mutex_for_mutable_shared_state_operations() {
    println!("\n");
    println!("Using Arc with Mutex for mutable shared state==================");

    // shared mutable state using Arc and Mutex
    // Mutex (Mutual Exclusion): An exclusive control mechanism that ensures only one thread can access the data at a time.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // By using Arc::clone(), the reference counter is incremented, allowing each thread to hold a reference to the same data.
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap(); // lock and get a mutable reference
            *num += 1; // increment the counter
        });
        handles.push(handle)
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", *counter.lock().unwrap());
}


fn arc_with_rwlock_for_read_heavy_operations() {
    println!("\n");
    println!("Using Arc with RwLock for read-heavy operations==================");


    // Create a shared vector wrapped in a RwLock and Arc
    // Arc allows multiple threads to share ownership
    // RwLock allows multiple readers or one writer at a time
    let shared_data = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];

    // Spawn multiple reader threads
    for i in 0..5 {
        let data_clone = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            // Acquire read lock (multiple readers can access simultaneously)
            let data = data_clone.read().unwrap();
            println!("Reader {} sees: {:?}", i, *data);
            thread::sleep(Duration::from_millis(100)); // Simulate reading delay
        });
        handles.push(handle);
    }

    // Spawn one writer thread
    let data_clone = Arc::clone(&shared_data);
    let writer_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50)); // Let some readers read first
        // Acquire write lock (exclusive access; blocks readers and other writers)
        let mut data = data_clone.write().unwrap();
        data.push(6); // Modify the shared vector
        println!("Writer added element 6");
    });
    handles.push(writer_handle);

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}
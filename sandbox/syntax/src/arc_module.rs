use std::sync::Arc;
use std::{thread, vec};

pub fn arc_main() {
    let data = Arc::new(String::from("Hello, Arc!"));
    let data_clone1 = Arc::clone(&data); // Cheap reference count increment
    let data_clone2 = Arc::clone(&data); // Another clone

    // All three variables point to the same string in memory
    println!("Original data: {}", data);
    println!("Clone 1: {}", data_clone1);
    println!("Clone 2: {}", data_clone2);

    sharing_data_between_threads();
}

// sharing data between threads
fn sharing_data_between_threads() {
    let numbers = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handlers = vec![];

    for i in 0..3 {
        let number_clone = Arc::clone(&numbers);

        let handle = thread::spawn(move || {
            let sum: i32 = number_clone.iter().sum();
            println!("Thread {}: Sum is {}", i, sum);
        });

        handlers.push(handle);
    }

    for handle in handlers {
        handle.join().unwrap()
    }
}

use std::thread;
use std::sync::Arc;
use std::time::Duration;

fn main() {
    basic_operations();
    move_data_into_thread_operations();
    sharing_data_between_thread_operations()
}

fn basic_operations() {
    println!("Basic thread operations==================");

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Thread is running: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Main: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // Wait for thread to finish
}

fn move_data_into_thread_operations() {
    println!("\n");
    println!("Moving data into threads==================");

    let data = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(move || {
        for i in data {
            println!("Thread received: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap(); // Wait for thread to finish
}

fn sharing_data_between_thread_operations() {
    println!("\n");
    println!("Sharing data between thread==================");

    let data = Arc::new(vec![1, 2, 3, 4, 5]);

    let mut handles = vec![];

    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("Thread {}: {:?}", i, data_clone);
            // // Do some work with the shared data
            let sum: i32 = data_clone.iter().sum();
            println!("Thread {}: Sum is {}", i, sum);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); // Wait for all threads to finish
    }
}

// fn mutex_for_mutable_shared_data

use std::thread;
use std::time::Duration;

pub fn thread_main() {
    thread_basic();
    thread_move();
}

pub fn thread_basic() {
  // Create a new thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    // Main thread continues
    for i in 1..5 {
        println!("Main: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    // Wait for the spawned thread to finish
    handle.join().unwrap();
}


fn thread_move() {
    let v = vec![1, 2, 3];
    
    // Move the vector into the thread
    let handle = thread::spawn(move || {
        println!("Vector in thread: {:?}", v);
    });
    
    // Wait for the thread to finish
    handle.join().unwrap();
}
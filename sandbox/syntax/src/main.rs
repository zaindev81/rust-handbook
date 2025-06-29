use std::vec;

fn main() {
    handle_for();
    handle_iter();
    handle_unwrap();
}

fn handle_for() {
    // simple loop
    for i in 1..10 {
        if i % 2 == 0 {
            println!("{} is even", i);
        } else {
            println!("{} is odd", i);
        }
    }

    // loop with a range
    let fruits = vec!["apple", "banana", "cherry"];
    for fruit in fruits {
        println!("I like {}", fruit);
    }

    // loop with index
    let colors = vec!["red", "green", "blue"];
    for (index, color) in colors.iter().enumerate() {
        println!("Color {}: {}", index + 1, color);
    }
}

fn handle_iter() {
    println!("=== UNDERSTANDING find() AND |&&x| ===\n");
    let numbers = vec![10, 20, 30, 40];

    for item in numbers.iter() {
        println!("Item: {}", item);
    }

    // Method 1
    println!("Method 1: |&&x| x == 20");
    let found1 = numbers.iter().find(|&&x| x == 20);
    println!("Found using find: {:?}", found1);

    // Method 3: Using |x|
    println!("Method 3: |x| *x == 20");
    let found3 = numbers.iter().find(|x| **x == 20);
    println!("  Result: {:?}\n", found3);

}

fn handle_unwrap() {
    let maybe_number: Option<i32> = Some(42);

    println!("Safe unwrap (we know it has a value):");
    let value = maybe_number.unwrap();
    println!("  maybe_number.unwrap() = {}", value);

      // Finding in a vector
    let numbers = vec![1, 2, 3, 4, 5];
    let found = numbers.iter().find(|&&x| x == 3);
    println!("  Found 3: {:?}", found);
    println!("  Unwrapped: {}", found.unwrap()); // Safe because we know 3 exists
}
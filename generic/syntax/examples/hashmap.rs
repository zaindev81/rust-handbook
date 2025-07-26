use std::collections::HashMap;

fn main() {
    basic_hashmap_operations();
    common_operations();
    iteration_operations();
    entry_api_operations();
}

fn basic_hashmap_operations() {
    // Create a new HashMap
    let mut scores = HashMap::new();

    // Insert some key-value pairs
    scores.insert("Alice", 50);
    scores.insert("Bob", 40);
    scores.insert("Charlie", 60);

    // Access values
    match scores.get("Alice") {
        Some(value) => println!("Alice's score: {}", value),
        None => println!("Alice not found"),
    }

    // Access a value using a key with a default value
    let bob_score = scores.get("Bob").unwrap_or(&0);
    println!("Bob's score: {}", bob_score);

    // Access a value using a key
    if let Some(score) = scores.get("Alice") {
        println!("Alice's score: {}", score);
    } else {
        println!("Alice not found");
    }

    // Iterate over the HashMap
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    // Remove a key-value pair
    scores.remove("Bob");

    // Check if a key exists
    if scores.contains_key("Bob") {
        println!("Bob is still in the scores.");
    } else {
        println!("Bob has been removed from the scores.");
    }
}

fn common_operations() {
    println!("\nCommon HashMap Operations====================");

    let mut scores = HashMap::new();

    // Insert values
    scores.insert("Blue", 10);
    scores.insert("Red", 50);

    // Update a value
    scores.insert("Blue", 25); // Overwrites the previous value

    // Insert only if key doesn't exist
    scores.entry("Yellow").or_insert(30);
    scores.entry("Blue").or_insert(100); // Won't change Blue's value

    // Remove a key-value pair
    scores.remove("Red");

    // Check if a key exists
    if scores.contains_key("Blue") {
        println!("Blue team exists!");
    }

    // Get the number of elements
    println!("Number of teams: {}", scores.len());

    // Check if the HashMap is empty
    if scores.is_empty() {
        println!("No teams in the scores.");
    } else {
        println!("There are teams in the scores.");
    }

    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
}

fn iteration_operations() {
    println!("\nIteration Operations====================");

    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    // Iterate over key-value pairs
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // Iterate over keys only
    for key in map.keys() {
        println!("Key: {}", key);
    }

    // Iterate over values only
    for value in map.values() {
        println!("Value: {}", value);
    }

    // Mutable iteration over values
    for value in map.values_mut() {
        *value *= 2; // Double all values
    }
}

fn entry_api_operations() {
    println!("\nEntry API Operations====================");

    let mut map = HashMap::new();

    // Insert or update based on existing value
    let count = map.entry("word").or_insert(0);
    *count += 1;

    // More complex logic with match
    match map.entry("key") {
        std::collections::hash_map::Entry::Occupied(mut entry) => {
            println!("Key exists with value: {}", entry.get());
            entry.insert(entry.get() + 1); // Increment existing value
        }
        std::collections::hash_map::Entry::Vacant(entry) => {
            println!("Key doesn't exist, inserting default");
            entry.insert(1);
        }
    }
}
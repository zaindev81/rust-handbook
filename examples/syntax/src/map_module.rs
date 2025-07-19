use std::collections::HashMap;

pub fn map_main() {
    let mut scores = HashMap::new();

    scores.insert("Alice", 50);
    scores.insert("Bob", 40);
    scores.insert("Charlie", 30);

    if let Some(score) = scores.get("Alice") {
        println!("Alice's score: {}", score);
    } else {
        println!("Alice not found");
    }

    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
}
use std::collections::HashMap;

pub fn iter_main() {
    normal_iter();
    score_inter();
}

fn normal_iter() {
    let number = vec![1, 2, 3, 4, 5];

     // iter() creates an iterator over references
    let doubled: Vec<i32> = number.iter().map(|&x| x * 2).collect();

    println!("Original numbers: {:?}", doubled);

    // into_iter() takes ownership
    let double_owned: Vec<i32> = number.into_iter().map(|x| x * 2).collect();
    println!("Doubled numbers: {:?}", double_owned);
}

fn score_inter() {
    let mut scores = HashMap::new();
    scores.insert("Alice", 50);
    scores.insert("Bob", 40);
    scores.insert("Charlie", 30);

    // values() returns an iterator over the values
    let total: i32 = scores.values().sum();
    println!("Total score: {}", total);

    // Collect all values into a Vec
    let all_scores: Vec<&i32> = scores.values().collect();
    println!("All scores: {:?}", all_scores); // [95, 87, 92] (order may vary)

    // Find hightest score
    let highest = scores.values().max();
    println!("Highest score: {:?}", highest);
}
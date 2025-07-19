// // &'static str 
// to_string

// Immutable.
// Fixed in memory (lives for the entire program).
// Very fast and efficient.
// Zero heap allocation.
fn example_static_str() {
    let s: &'static str = "Hello, Rust!";
    println!("{}", s);
}

// Empty string.
// Mutable and growable.
// No initial data.
// Used when building a string dynamically.
fn example_string_new() {
    let mut s = String::new();
    s.push_str("Hello");
    s.push(' ');
    s.push_str("Rust!");
    println!("{}", s);
}

// Simple and idiomatic.
// Uses the ToString trait.
// Equivalent to String::from("text").
fn example_to_string() {
    let s = "Hello, Rust!".to_string();
    println!("{}", s);
}

// Explicit and readable.
// Commonly used when converting &str to String.
// Functionally the same as "text".to_string().
fn example_string_from() {
    let s = String::from("Hello, Rust!");
    println!("{}", s);
}

pub fn str_main() {
    let s1 = String::from("Hello, Rust!");
    let s2 = "Hello, Rust!".to_string();
    let s3= String::new();
    let s4 = String::with_capacity(10);

    example_static_str();
    example_string_new();
    example_to_string();
    example_string_from();

    println!("s1: {}, s2: {}, s3: {}, s4: {}", s1, s2, s3, s4);
}
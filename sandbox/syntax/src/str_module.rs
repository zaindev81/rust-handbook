// // &'static str 
// to_string

pub fn str_main() {
    let s1 = String::from("Hello, Rust!");
    let s2 = "Hello, Rust!".to_string();
    let s3= String::new();
    let s4 = String::with_capacity(10);

    println!("s1: {}, s2: {}, s3: {}, s4: {}", s1, s2, s3, s4);
}
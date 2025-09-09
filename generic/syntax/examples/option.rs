fn main() {
    let some_number = Some(5);
    let no_number: Option<i32> = None;

    match some_number {
        Some(n) => println!("Got a number: {}", n),
        None => println!("No number"),
    }

    match no_number {
        Some(n) => println!("Got a number: {}", n),
        None => println!("No number"),
    }
}
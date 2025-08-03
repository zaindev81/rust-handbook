// map_err

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}


fn basic_error_handling() {
    // Method 1: Using match
    match divide(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(error) => println!("Error: {}", error),
    }

    // Method 2: Using if let
    if let Ok(result) = divide(10, 0) {
        println!("Result: {}", result);
    } else {
        println!("Division failed");
    }
}

fn main() {
    basic_error_handling();
}
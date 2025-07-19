use std::fs;
use std::io;
use std::num::ParseIntError;
use std::fmt;

pub fn dyn_main() {
    println!("=== DYNAMIC MODULE ===");

    match read_and_parse_file("data.txt") {
        Ok(number) => {
            println!("Data read successfully: {:?}", number);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }

    match complex_operation() {
        Ok(result) => {
            println!("Complex operation result: {}", result);
        }
        Err(e) => {
            println!("Error in complex operation: {}", e);
            let mut source = e.source();
            while let Some(err) = source {
                println!("Caused by: {}", err);
                source = err.source();
            }
        }
    }

    match chain_operations() {
        Ok(sum) => {
            println!("Sum of numbers: {}", sum);
        }
        Err(e) => {
            eprintln!("Error in chain operations: {}", e);
        }
    }

}

fn read_and_parse_file(filename: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(filename)?;
    let number = content.trim().parse::<i32>()?;
    Ok(number)
}

fn complex_operation() -> Result<String, Box<dyn std::error::Error>> {
  let content = fs::read_to_string("data.txt")
        .unwrap_or_else(|_| "42".to_string());
    
    let number: i32 = content.trim().parse()?;
    
    if number < 0 {
        return Err("Negative number is now allowed".into());
    }
    
    let result = number * 2;
    Ok(format!("result: {}", result))
}

fn chain_operations() -> Result<i32, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("numbers.txt")
        .unwrap_or_else(|_| "10".to_string());
    
    let numbers: Result<Vec<i32>, _> = content
        .split_whitespace()
        .map(|s| s.parse())
        .collect();
    
    let numbers = numbers?;
    let sum: i32 = numbers.iter().sum();
    
    Ok(sum)
}
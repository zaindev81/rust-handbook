use std::fs::File;
use std::io::{self, Read};

fn main() {
    match_operations();
    unwrap_and_expect_operations();

    if let Err(e) = question_mark_operations() {
        eprintln!("Caught error: {}", e);
    }

    map_error_operations().unwrap();
    let chained_val1 = chained_operations("30").unwrap();
    println!("chained_val1 {}", chained_val1);

    let chained_val2 = chained_operations("31").unwrap_or(10);
    println!("chained_val2 {}", chained_val2);

    let read_file_val= read_file_operations("README.md").unwrap();
    // println!("read_file_val {}", read_file_val);
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero.".to_string())
    } else {
        Ok(a / b)
    }
}

fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|e| format!("Custom parse error: {}", e))
}

fn double_if_even(n: i32) -> Result<i32, String> {
    if n % 2 == 0 {
        Ok(n * 2)
    } else {
        Err("Not an even number".to_string())
    }
}

fn match_operations() {
    let result = divide(10.0, 2.0);
    match result  {
        Ok(val) => println!("Success: {}", val),
        Err(e) => eprintln!("Failed: {}", e),
    }
    let result2 = divide(10.0, 0.0);
    match result2 {
        Ok(val) => println!("Success: {}", val),
        Err(e) => eprintln!("Failed: {}", e),
    }
}

fn unwrap_and_expect_operations() {
    let result = divide(10.0, 2.0);
    let val = result.unwrap();
    println!("val: {}", val);

    // let result2 = divide(10.0, 0.0);
    // let val2 = result2.unwrap(); // panics if it's an Err

    // let result3 = divide(10.0, 0.0);
    // let val3 = result3.expect("Something went wrong"); // better error message
}

fn question_mark_operations() -> Result<(), String> {
    let result = divide(10.0, 0.0)?;
    println!("result: {}", result);

    Ok(())
}

fn map_error_operations() -> Result<(), String> {
    let num = parse_number("42")?;
    println!("Parse number: {}", num);

    // let num = parse_number("hello")?;
    // println!("Parse number: {}", num);

    Ok(())
}

fn chained_operations(s: &str) -> Result<i32, String> {
    parse_number(s).and_then(double_if_even)
}

fn read_file_operations(path: &str) -> Result<String, io::Error> {
    let mut content = String::new();
    File::open(path)?.read_to_string(&mut content)?;
    Ok(content)
}
fn main() {
    match_operations();
    unwrap_and_expect_operations();

    if let Err(e) = question_mark_operations() {
        eprintln!("Caught error: {}", e);
    }
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero.".to_string())
    } else {
        Ok(a / b)
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
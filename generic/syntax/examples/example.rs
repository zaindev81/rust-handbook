fn main() {
    match_operations();
    unwrap_and_expect_operations();

    if let Err(e) = question_mark_operations() {
        eprintln!("e {}", e);
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
    // let val = result.unwrap();
    match result {
        Ok(val) => println!("val {}", val),
        Err(e) => eprintln!("err {}", e)
    };

    let result2 = divide(10.0, 0.0);
    // let val = result.unwrap();
    match result2 {
        Ok(val) => println!("val {}", val),
        Err(e) => eprintln!("err {}", e)
    };
}

fn unwrap_and_expect_operations() {
    let result = divide(10.0, 2.0);
    let val = result.unwrap();
    println!("val {}", val);

    // let result = divide(10.0, 0.0);
    // let val = result.expect("something went wrong");
    // println!("val {}", val);
}

fn question_mark_operations() -> Result<(), String> {
    let result = divide(10.0, 0.0)?;
    Ok(())
}
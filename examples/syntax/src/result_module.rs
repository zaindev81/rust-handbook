pub fn result_main() {
    let some_value: Option<i32> = Some(10);
    let none_value: Option<i32> = None;

    if some_value.is_some() {
        println!("Some value: {}", some_value.unwrap());
    } else {
        println!("No value present");
    }

    if none_value.is_none() {
        println!("No value present");
    } else {
        println!("Some value: {}", none_value.unwrap());
    }

    let name: Option<String> = get_user_name();

    if name.is_some() {
        println!("User name: {}", name.unwrap());
    } else {
        println!("No user name found");
    }

    result_basic();
}

fn get_user_name() -> Option<String> {
    Some("Alice".to_string())
}

pub fn result_basic() {
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("An error occurred");

    let value = success.unwrap(); // 42
    let value1 = success.expect("should be a number");

    let value2 = success.unwrap_or(0);
    let value3 = failure.unwrap_or(0);

    let value4 = failure.unwrap_or_else(|err| {
        println!("Error: {}", err);
        -1
    });

    println!("Success value: {}", value);
    println!("Success value with expect: {}", value1);
    println!("Success value with default: {}", value2);
    println!("Failure value with default: {}", value3);
    println!("Failure value with error handling: {}", value4);
}

// ? operator
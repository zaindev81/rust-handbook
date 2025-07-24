use std::fs::File;
use std::io::{self, Read};

pub fn result_main() {
    result_simple();
    result_basic();
    result_divide();
    result_option();
    result_question_mark();
    result_custom();
}

fn get_user_name() -> Option<String> {
    Some("Alice".to_string())
}

pub fn result_simple() {
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

}

fn result_basic() {
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

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn result_divide() {
    match divide(10, 2) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }

    match divide(10, 0) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }

    let result1 = divide(10, 2);
    let value1 = result1.unwrap();
    println!("Unwrapped value: {}", value1);

    let result2 = divide(10, 2);
    let value2 = result2.unwrap_or(0);
    println!("Unwrapped or default value: {}", value2);

    let result3 = divide(10, 2);
    let value3 = result3.unwrap_or_else(|err| {
        println!("Error occurred: {}", err);
        -1
    });
    print!("Unwrapped or error handling value: {}", value3);

    let result4 = divide(10, 0);
    if result4.is_ok() {
        println!("Division successful");
    } else {
        println!("Division failed: {}", result4.unwrap_err());
    }

    let result5 = divide(10, 0);

}

// ================================
// OPTION TYPE BASICS
// ================================

// Option<T> represents either Some value or None
fn find_word(text: &str, word: &str) -> Option<usize> {
    text.find(word)
}

fn get_first_char(s: &str) -> Option<char> {
    s.chars().next()
}

fn result_option() {
    let text = "Hello, world!";
    let word = "world";

    match find_word(text, word) {
        Some(index) => println!("Found '{}' at index {}", word, index),
        None => println!("'{}' not found in text", word),
    }

    let first_char = get_first_char(text);
    match first_char {
        Some(c) => println!("First character: {}", c),
        None => println!("String is empty"),
    }
}

// ================================
// THE ? OPERATOR WITH RESULT
// ================================

// WITHOUT ? operator - verbose error handling
fn read_file_content_verbose(filename: &str) -> Result<String, io::Error> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(error) => Err(error),
    }
}

// WITH ? operator - clean and concise
fn read_file_content_clean(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;  // If Err, return early
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;   // If Err, return early
    Ok(contents)
}

fn result_question_mark() {
    let filename = "example.txt";

    match read_file_content_verbose(filename) {
        Ok(content) => println!("File content: {}", content),
        Err(error) => println!("Error reading file: {}", error),
    }

    match read_file_content_clean(filename) {
        Ok(content) => println!("File content: {}", content),
        Err(error) => println!("Error reading file: {}", error),
    }
}

fn result_custom() {
    let filename = "src/result_module.rs";

    match load_file(filename) {
        Ok(content) => println!("File content: {}", content),
        Err(error) => println!("Error reading file: {:?}", error),
    }
}

#[derive(Debug)]
enum MyError {
    EmptyInput,
    IoError(std::io::Error),
}

impl From<std::io::Error> for MyError {
    fn from(error: std::io::Error) -> Self {
        MyError::IoError(error)
    }
}

// alias
type ResultInput<T> = std::result::Result<T, MyError>;

fn load_file(path: &str) -> ResultInput<String> {
    let content = std::fs::read_to_string(path)?; // MyError
    Ok(content)
}

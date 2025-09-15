use anyhow::{Result, Context}; // needs Context

// needs Context for "with_context"
// ||
// ? => raises error if Err, otherwise unwraps Ok

fn read_file() -> Result<String> {
    let content = std::fs::read_to_string("config.txt")
        .with_context(|| format!("Failed to read config.txt"))?;
    Ok(content)
}

fn main() -> Result<()> {
    // closures
    let add = |a: i32, b: i32| a + b;
    let result = add(3, 5); // 8
    println!("3 + 5 = {}", result);

    let text = read_file()?;
    println!("File content: {}", text);

    Ok(())
}

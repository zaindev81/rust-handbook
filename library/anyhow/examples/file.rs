use anyhow::{Result, anyhow};

fn read_file() -> Result<String> {
    let content = std::fs::read_to_string("config.txt")?;
    Ok(content)
}

fn main() -> Result<()> {
    let text = read_file()?;
    println!("File content: {}", text);
    Ok(())
}